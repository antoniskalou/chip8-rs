mod buzzer;
mod cpu;
mod fonts;
mod memory;
mod rom;
mod screen;

use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const RENDER_SCALE: u32 = 20;
const TARGET_FPS: u32 = 60;
const TARGET_MHZ: u32 = 540;
// the number of CPU cycles that occur before a refresh happens
const CYCLES_PER_REFRESH: u32 = TARGET_MHZ / TARGET_FPS;
const REFRESH_PER_SECOND: f32 = 1. / TARGET_FPS as f32;

fn timed<F>(mut f: F) -> Duration
where
    F: FnMut() -> (),
{
    let start = Instant::now();
    f();
    start.elapsed()
}

fn clear_graphics(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0x00, 0x00, 0x00));
    canvas.clear();
}

fn draw_graphics(canvas: &mut Canvas<Window>, buffer: &[bool]) {
    canvas.set_draw_color(Color::RGB(0x00, 0xFF, 0x00));
    for (i, pixel) in buffer.iter().enumerate() {
        if *pixel {
            let x = (i % screen::WIDTH) as u32;
            let y = (i / screen::WIDTH) as u32;
            let rect = Rect::new(
                (x * RENDER_SCALE) as i32,
                (y * RENDER_SCALE) as i32,
                RENDER_SCALE,
                RENDER_SCALE,
            );
            canvas.fill_rect(rect).expect("fill_rect failed");
        }
    }
    canvas.present();
}

fn scancode_to_key(scancode: Scancode) -> Option<u8> {
    use Scancode::*;
    match scancode {
        Num1 => Some(0x1),
        Num2 => Some(0x2),
        Num3 => Some(0x3),
        Num4 => Some(0xC),
        Q => Some(0x4),
        W => Some(0x5),
        E => Some(0x6),
        R => Some(0xD),
        A => Some(0x7),
        S => Some(0x8),
        D => Some(0x9),
        F => Some(0xE),
        Z => Some(0xA),
        X => Some(0x0),
        C => Some(0xB),
        V => Some(0xF),
        _ => None,
    }
}

fn main() -> Result<(), String> {
    let rom_path = std::env::args()
        .nth(1)
        .map(|p| PathBuf::from(p))
        .ok_or_else(|| "Usage: chip8 <ROM FILE>")?;

    let rom = rom::load(&rom_path).map_err(|e| e.to_string())?;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let audio_subsystem = sdl_context.audio()?;

    let window = video_subsystem
        .window(
            "Chip8",
            screen::WIDTH as u32 * RENDER_SCALE,
            screen::HEIGHT as u32 * RENDER_SCALE,
        )
        .position_centered()
        .vulkan()
        .build()
        .expect("window creation failed");

    // init rendering
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));

    // init audio
    let buzzer = buzzer::Buzzer::new(audio_subsystem)?;

    // load emulator components
    let mut memory = memory::Memory::new();
    // TODO: make this a function called `Memory::with_rom(&[u8]) -> Memory`
    memory.load(&fonts::FONTSET, fonts::BASE_ADDRESS);
    memory.load(&rom, rom::BASE_ADDRESS);

    let mut cpu = cpu::CPU::new(memory);
    // main loop
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => {
                    if let Some(k) = scancode_to_key(scancode) {
                        cpu.press_key(k, true);
                    }
                }
                Event::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => {
                    if let Some(k) = scancode_to_key(scancode) {
                        cpu.press_key(k, false);
                    }
                }
                _ => {}
            }
        }
        clear_graphics(&mut canvas);

        // timers
        cpu.tick_timers();
        // cpu tick
        let elapsed = timed(|| {
            for _ in 0..=CYCLES_PER_REFRESH {
                cpu.tick();
            }
        });
        // audio
        if cpu.is_sound_playing() {
            buzzer.play();
        } else {
            buzzer.pause();
        }

        draw_graphics(&mut canvas, cpu.screen_buffer());

        // wait for next iteration
        let rps = Duration::from_secs_f32(REFRESH_PER_SECOND);
        std::thread::sleep(rps - elapsed);
    }

    Ok(())
}
