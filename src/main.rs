mod emulator;
mod platform;

use std::{fs::File, io::Read};

use emulator::Emulator;
use platform::handle_keys;

use minifb::{Window, WindowOptions};

fn main() {
    // Creating window
    let mut window = Window::new("CHIP-8 emulator", 640, 320, WindowOptions::default()).unwrap();
    window.set_target_fps(60);

    // Reading ROM data
    let mut buffer = [0u8; 0x800];
    let mut file = File::open("./cal.ch8").unwrap();
    file.read(&mut buffer).unwrap();

    // Creatin emulator and loading ROM data
    let mut emulator = Emulator::new();
    emulator.load_rom(&buffer);

    while window.is_open() {
        handle_keys(&window, |key, is_pressed| {
            if is_pressed {
                emulator.press_key(key)
            } else {
                emulator.release_key(key)
            }
        });

        for _ in 0..10 {
            emulator.cycle();
        }

        emulator.decrement_timers();

        window
            .update_with_buffer(&emulator.render(), 64, 32)
            .unwrap()
    }
}
