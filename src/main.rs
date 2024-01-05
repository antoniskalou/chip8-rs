mod cpu;
mod memory;
mod rom;
mod screen;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::path::PathBuf;
use std::time::Duration;

fn main() -> Result<(), String> {
    let rom_path = std::env::args()
        .nth(1)
        .map(|p| PathBuf::from(p))
        .ok_or_else(|| "Usage: chip8 <ROM FILE>")?;

    let rom = rom::load(&rom_path);
    println!("Loaded ROM: {:?}", rom);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let _window = video_subsystem
        .window("Chip8", 800, 600)
        .position_centered()
        .build()
        .expect("window creation failed");

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
