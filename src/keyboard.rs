use std::char;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

#[rustfmt::skip]
const SOURCE_KEYS: [char; 16] = [
    '1', '2', '3', '4', 
    'Q', 'W', 'E', 'R', 
    'A', 'S', 'D', 'F', 
    'Z', 'X', 'C', 'V',
];

const TARGET_KEYS: [u8; 16] = [
    0x1, 0x2, 0x3, 0xA, // 1, 2, 3, C,
    0x4, 0x5, 0x6, 0xB, // 4, 5, 6, D,
    0x7, 0x8, 0x9, 0xC, // 7, 8, 9, E,
    0xD, 0x0, 0xE, 0xF, // A, 0, B, F,
];

fn map_key(key: char) -> Option<u8> {
    for i in 0..TARGET_KEYS.len() {
        if key.to_ascii_uppercase() == SOURCE_KEYS[i] {
            return Some(TARGET_KEYS[i]);
        }
    }
    None
}

// Reading input with no delay
const TIMEOUT: Duration = Duration::ZERO;

pub fn read_key() -> Option<u8> {
    // Checks if any key was pressed
    if !event::poll(TIMEOUT).ok()? {
        return None;
    }

    // Processing only KeyEvents because all input comes from keyboard
    if let Ok(Event::Key(key_event)) = event::read() {
        // Ignores everything except char keys (special symbols like Esc will be ignored)
        if let KeyCode::Char(ch) = key_event.code {
            // Aborts program if user pressed Ctrl+C
            if key_event.modifiers.contains(KeyModifiers::CONTROL) && ch.to_ascii_lowercase() == 'c'
            {
                std::process::exit(0)
            };
            // Otherwise, returns mapped key.
            return map_key(ch);
        }
    };

    None
}
