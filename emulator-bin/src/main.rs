use emulator_core::{Cpu, load_rom};
use pixels::Pixels;
use pixels::SurfaceTexture;
use std::process;
use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowId;

const ONE_SECOND_IN_CYCLES: usize = 4190000;
const FIXED_TIMESTEP: f64 = 1.0 / 60.0; // 60 FPS target
const MAX_ACCUMULATED_TIME: f64 = 0.1; // Don't accumulate more than 100ms

struct App {
    window: Arc<Window>,
    pixels: Pixels<'static>,
    frame_count: u64,
    cpu: Cpu,
    last_frame_time: Option<Instant>,
    accumulated_time: f64,
}

impl App {
    fn new(window: Arc<Window>, pixels: Pixels<'static>, frame_count: u64, cpu: Cpu) -> Self {
        Self {
            window,
            pixels,
            frame_count,
            cpu,
            last_frame_time: None,
            accumulated_time: 0.0,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(new_size) => {
                // This will help to maintain the hardcoded
                // Gameboy resolution aspect ratio based on the
                // current size of the window
                self.pixels
                    .resize_surface(new_size.width, new_size.height)
                    .unwrap();
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();

                // Calculate time delta and update accumulator
                if let Some(last_time) = self.last_frame_time {
                    let delta = now.duration_since(last_time).as_secs_f64();

                    // Add delta but cap it to prevent death spiral
                    self.accumulated_time =
                        (self.accumulated_time + delta).min(MAX_ACCUMULATED_TIME);

                    // Execute fixed timesteps outerloop renders multiple frames if behind
                    while self.accumulated_time >= FIXED_TIMESTEP {
                        let target_cycles = (FIXED_TIMESTEP * ONE_SECOND_IN_CYCLES as f64) as usize;
                        let mut cycle_count_total = 0;
                        while cycle_count_total < target_cycles {
                            cycle_count_total += self.cpu.step() as usize;
                        }
                        self.accumulated_time -= FIXED_TIMESTEP;
                    }
                }

                self.last_frame_time = Some(now);

                let frame = self.pixels.frame_mut();
                let color = if (self.frame_count / 60) % 2 == 0 {
                    [0xff, 0x00, 0x00, 0xff]
                } else {
                    [0x00, 0x00, 0xff, 0xff]
                };

                for pixel in frame.chunks_exact_mut(4) {
                    pixel.copy_from_slice(&color);
                }

                self.pixels.render().unwrap();

                self.frame_count += 1;
                self.window.request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = "roms/tetris.gb";

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

    let cpu = Cpu::new(rom);
    let mut app = App::new(window, pixels, 0, cpu);
    event_loop.run_app(&mut app)?;
    Ok(())
}
