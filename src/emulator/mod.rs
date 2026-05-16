mod cpu;
mod display;
mod keypad;
mod memory;

pub use cpu::Cpu;
pub use display::Display;
pub use keypad::Keypad;
pub use memory::Memory;

pub struct Emulator {
    cpu: Cpu,
    memory: Memory,
    display: Display,
    keypad: Keypad,
}

impl Emulator {
    fn new() -> Self {
        return Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        };
    }
}
