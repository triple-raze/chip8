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
    pub fn new() -> Self {
        return Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
            display: Display::new(),
            keypad: Keypad::new(),
        };
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        self.memory.write_bytes(0x200, data);
    }

    pub fn cycle(&mut self) {
        self.cpu
            .cycle(&mut self.memory, &mut self.display, &self.keypad);
    }

    pub fn decrement_timers(&mut self) {
        self.cpu.decement_timers();
    }

    pub fn press_key(&mut self, key: u8) {
        self.keypad.press_key(key);
    }

    pub fn release_key(&mut self, key: u8) {
        self.keypad.release_key(key);
    }

    pub fn render(&self) -> [u32; 2048] {
        self.display.render()
    }
}
