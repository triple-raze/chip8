pub struct Keypad {
    pressed_keys: u16, // Bitmask that represents an array of pressed buttons.
    last_pressed_key: Option<u8>, // Represents position of key at bitmask
}

impl Keypad {
    pub fn new() -> Self {
        return Self {
            pressed_keys: 0,
            last_pressed_key: None,
        };
    }

    pub fn press_key(&mut self, key: u8) {
        self.pressed_keys |= 1 << key;
        self.last_pressed_key = Some(key)
    }

    pub fn release_key(&mut self, key: u8) {
        self.pressed_keys &= !(1 << key)
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.pressed_keys & (1 << key) != 0
    }

    pub fn is_any_key_pressed(&self) -> bool {
        self.pressed_keys != 0
    }

    pub fn get_last_pressed_key(&self) -> u8 {
        self.last_pressed_key.unwrap()
    }
}
