extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod ox_chip8;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut emulator = ox_chip8::Chip8::new(&sdl_context);
    emulator.init();
    emulator.load_program();

    let mut event_pump = sdl_context.event_pump().unwrap();
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
        emulator.cycle();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
