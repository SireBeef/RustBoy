use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug)]
pub enum RomLoadError {
    FileNotFound(String),
    IoError(io::Error),
    EmptyRom,
}

impl std::fmt::Display for RomLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RomLoadError::FileNotFound(path) => write!(f, "ROM file not found: {}", path),
            RomLoadError::IoError(err) => write!(f, "Failed to read ROM file: {}", err),
            RomLoadError::EmptyRom => write!(f, "ROM file is empty"),
        }
    }
}

impl std::error::Error for RomLoadError {}

impl From<io::Error> for RomLoadError {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => RomLoadError::FileNotFound(err.to_string()),
            _ => RomLoadError::IoError(err),
        }
    }
}

pub fn load_rom<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, RomLoadError> {
    let path_ref = path.as_ref();

    // Check if file exists first for a better error message
    if !path_ref.exists() {
        return Err(RomLoadError::FileNotFound(
            path_ref.display().to_string()
        ));
    }

    let mut file = File::open(path_ref)?;
    let mut rom = Vec::new();
    file.read_to_end(&mut rom)?;

    if rom.is_empty() {
        return Err(RomLoadError::EmptyRom);
    }

    Ok(rom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs;

    #[test]
    fn test_load_nonexistent_rom() {
        let result = load_rom("nonexistent_rom.gb");
        assert!(result.is_err());
        match result {
            Err(RomLoadError::FileNotFound(_)) => {},
            _ => panic!("Expected FileNotFound error"),
        }
    }

    #[test]
    fn test_load_empty_rom() {
        let temp_path = "test_empty.gb";
        File::create(temp_path).unwrap();

        let result = load_rom(temp_path);
        fs::remove_file(temp_path).unwrap();

        assert!(result.is_err());
        match result {
            Err(RomLoadError::EmptyRom) => {},
            _ => panic!("Expected EmptyRom error"),
        }
    }

    #[test]
    fn test_load_valid_rom() {
        let temp_path = "test_valid.gb";
        let mut file = File::create(temp_path).unwrap();
        file.write_all(&[0x00, 0xC3, 0x50, 0x01]).unwrap();

        let result = load_rom(temp_path);
        fs::remove_file(temp_path).unwrap();

        assert!(result.is_ok());
        let rom = result.unwrap();
        assert_eq!(rom.len(), 4);
        assert_eq!(rom, vec![0x00, 0xC3, 0x50, 0x01]);
    }
}
