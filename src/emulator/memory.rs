pub struct Memory {
    pub data: [u8; 0x1000],
}

const CHARACTERS_DATA: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self { data: [0; 0x1000] };

        for (idx, character_data) in CHARACTERS_DATA.iter().enumerate() {
            // 0x000 -> 0x1FF
            let offset = idx * 5;
            memory.write_bytes(offset, character_data);
        }

        memory
    }

    pub fn read_byte(&self, pointer: usize) -> u8 {
        return self.data[pointer];
    }

    pub fn write_byte(&mut self, pointer: usize, value: u8) {
        self.data[pointer] = value
    }

    pub fn read_bytes(&self, pointer: usize, len: usize) -> &[u8] {
        return &self.data[pointer..pointer + len];
    }

    pub fn write_bytes(&mut self, pointer: usize, values: &[u8]) {
        self.data[pointer..pointer + values.len()].copy_from_slice(values);
    }

    pub fn read_word(&self, pointer: usize) -> u16 {
        return (self.data[pointer] as u16) << 8 | self.data[pointer + 1] as u16;
    }
}
