mod emulator;
mod platform;

use emulator::Emulator;
use platform::{map_keys, read_rom};

use minifb::{Window, WindowOptions};

fn main() {
    // Creating window
    let mut window = Window::new("CHIP-8 emulator", 640, 320, WindowOptions::default()).unwrap();
    window.set_target_fps(60);

    // Reading ROM from file specified in first argument of command call
    let args: Vec<String> = std::env::args().collect();
    let file_path = match args.get(1) {
        Some(path) => path,
        None => {
            eprintln!("File name not specified.");
            std::process::exit(1);
        }
    };

    let data = read_rom(file_path);

    // Creating emulator and loading ROM data
    let mut emulator = Emulator::new();
    emulator.load_rom(&data);

    while window.is_open() {
        map_keys(&window, |key, is_pressed| {
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
