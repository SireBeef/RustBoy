use emulator_core::{Cpu, load_rom};
use pixels::Pixels;
use pixels::SurfaceTexture;
use std::process;
use std::rc::Rc;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowId;

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    frame_count: u64,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("RustBoy Emulator")
            .with_inner_size(LogicalSize::new(160 * 4, 144 * 4));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let window_size = window.inner_size();
        let pixels = Pixels::new(
            160,
            144,
            SurfaceTexture::new(window_size.width, window_size.height, window.clone()),
        )
        .unwrap();

        self.pixels = Some(pixels);
        self.window = Some(window);

        self.window.as_ref().unwrap().request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                println!("Redraw");

                if let Some(pixels) = &mut self.pixels {
                    let frame = pixels.frame_mut();
                    let color = if (self.frame_count / 60) % 2 == 0 {
                        [0xff, 0x00, 0x00, 0xff]
                    } else {
                        [0x00, 0x00, 0xff, 0xff]
                    };

                    for pixel in frame.chunks_exact_mut(4) {
                        pixel.copy_from_slice(&color);
                    }

                    pixels.render().unwrap();
                }

                self.frame_count += 1;
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = "roms/pokemon-blue.gb";

    let rom = match load_rom(rom_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error loading ROM: {}", e);
            eprintln!("Please ensure the ROM file exists at: {}", rom_path);
            process::exit(1);
        }
    };

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
