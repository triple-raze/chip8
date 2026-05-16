pub struct Memory {
    data: [u8; 0x1000],
}

impl Memory {
    pub fn new() -> Self {
        return Self { data: [0; 0x1000] };
    }

    pub fn read_byte(&self, pointer: usize) -> u8 {
        return self.data[pointer];
    }

    pub fn read_bytes(&self, pointer: usize, len: usize) -> &[u8] {
        return &self.data[pointer..pointer + len];
    }

    pub fn read_word(&self, pointer: usize) -> u16 {
        return (self.data[pointer] as u16) << 8 | self.data[pointer + 1] as u16;
    }
}
