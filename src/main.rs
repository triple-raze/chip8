// GOVNOKOD!!!!!!
// чисто тестовая хуйня потом будет минимальный набор

mod keyboard;

use crossterm::terminal;
pub mod emulator;

fn main() {
    // Terminal should get all entered characters immediately, raw mode allows this
    terminal::enable_raw_mode().unwrap();

    let mut display = emulator::Display::new();

    let sprite: [u8; 8] = [
        0b00111100, //   ████
        0b01000010, //  █    █
        0b10100101, // █ █  █ █
        0b10000001, // █      █
        0b10100101, // █ █  █ █
        0b10011001, // █  ██  █
        0b01000010, //  █    █
        0b00111100, //   ████
    ];

    let mut offset = 0;

    // Main loop
    let mut offset: i32 = 0;
    let mut direction: i32 = 1; // 1 = вправо, -1 = влево

    use std::time::Instant;

    let mut frame_count = 0;
    let mut fps_timer = Instant::now();
    let start_time = Instant::now();
    let timeout = std::time::Duration::from_secs(1); // 10 секунд работы

    loop {
        // Проверка таймаута
        if start_time.elapsed() >= timeout {
            println!("Время вышло!");
            break;
        }

        // let key = keyboard::read_key();
        // if let Some(key) = key {
        //     println!("{:?}", key);
        // }
        //
        display.draw_sprite(&sprite, offset as usize, 20);
        // print!("{}", display.render());
        // display.clear();
        display.render();

        // Меняем offset
        // offset += direction;
        // if offset >= 60 {
        //     direction = -1;
        // } else if offset == 0 {
        //     direction = 1;
        // }

        // Счётчик FPS
        frame_count += 1;
        let elapsed = fps_timer.elapsed();
        if elapsed >= std::time::Duration::from_secs(1) {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            fps_timer = Instant::now();
        }
    }
}
