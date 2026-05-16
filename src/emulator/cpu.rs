use std::array;

use super::Display;
use super::Keypad;
use super::Memory;

struct Registers {
    v: [u8; 16], // General registers from 0x0 to 0xF
    i: u16,      // Needed to store memory adresses
    dt: u8,      // Delay timer. Decrements 60 times per second
    st: u8,      // Sound timer. Makes sound if its not equal to 0. Decrements 60 times per second
    pc: u16,     // Program counter, stores current memory address (u12).
    sp: u8,      // Stack pointer, stores first free slot in stack
}

impl Registers {
    fn new() -> Self {
        return Self {
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0x200,
            sp: 0,
        };
    }
}

pub struct Cpu {
    registers: Registers,
    stack: [u8; 16],
}

impl Cpu {
    pub fn new() -> Self {
        return Self {
            registers: Registers::new(),
            stack: [0; 16],
        };
    }

    fn execute_opcode(&mut self, opcode: u16) {
        // First nibble which means a group of opcodes
        let group = opcode & 0xF000 >> 12;

        let x = (opcode & 0x0F00 >> 8) as usize; // Register with number "x"
        let y = (opcode & 0x00F0 >> 4) as usize; // Register with number "y"
        let n = (opcode & 0x000F) as u8; // Value in last 1 nibble
        let kk = (opcode & 0x00FF) as u8; // Value in last 2 nibbles
        let nnn = opcode & 0x0FFF; // Value in last 3 nibbles

        match group {
            0 => self.op_0nnn(),
            1 => self.op_1nnn(nnn),
            2 => self.op_2nnn(nnn),
            _ => panic!("Unknown opcode {:#X}", opcode),
        }
    }

    /// Makes one CPU cycle
    pub fn iterate(&mut self, memory: Memory) {
        // Reading current opcode
        let opcode = memory.read_word(self.registers.pc as usize);

        // Moving to next opcode
        self.registers.pc += 2;

        // Executing one opcode
        self.execute_opcode(opcode);
    }

    // 0nnn - SYS addr
    /// Ignored by modern interpreters.
    fn op_0nnn(&self) {}

    // 00E0 - CLS
    /// Clear the display.
    fn op_00e0(&self, display: &mut Display) {
        display.clear();
    }

    // 1nnn - JP addr
    /// Jump to location nnn (u12).
    fn op_1nnn(&mut self, address: u16) {
        self.registers.pc = address;
    }

    // 2nnn - CALL addr
    /// Call subroutine at nnn (u12).
    fn op_2nnn(&mut self, address: u16) {
        self.registers.sp += 1;
        self.registers.pc = address;
    }

    // 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    fn op_3xkk(&mut self, x: usize, value: u8) {
        if self.registers.v[x] == value {
            self.registers.pc += 2;
        }
    }

    // 4xkk - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    fn op_4xkk(&mut self, x: usize, value: u8) {
        if !(self.registers.v[x] == value) {
            self.registers.pc += 2;
        }
    }
    // 5xy0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.registers.v[x] == self.registers.v[y] {
            self.registers.pc += 2;
        }
    }

    // 6xkk - LD Vx, byte
    /// Set Vx = kk.
    fn op_6xkk(&mut self, x: usize, value: u8) {
        self.registers.v[x] = value
    }

    // 7xkk - ADD Vx, byte
    /// Set Vx = Vx + kk with wrapping.
    fn op_7xkk(&mut self, x: usize, value: u8) {
        self.registers.v[x] = self.registers.v[x].wrapping_add(value);
    }

    // 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.registers.v[x] = self.registers.v[y]
    }

    // 8xy1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.registers.v[x] = self.registers.v[x] | self.registers.v[y]
    }

    // 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.registers.v[x] = self.registers.v[x] & self.registers.v[y]
    }

    // 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.registers.v[x] = self.registers.v[x] ^ self.registers.v[y]
    }

    // 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    fn op_8xy4(&mut self, x: usize, y: usize) {
        let (result, is_overflowed) = self.registers.v[x].overflowing_add(self.registers.v[y]);
        self.registers.v[x] = result;
        if is_overflowed {
            self.registers.v[0xF] = 1;
        }
    }

    // 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    fn op_8xy5(&mut self, x: usize, y: usize) {
        if self.registers.v[x] >= self.registers.v[y] {
            self.registers.v[0xF] = 1;
        }
        self.registers.v[x] = self.registers.v[x].wrapping_sub(self.registers.v[y]);
    }

    // 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1
    fn op_8xy6(&mut self, x: usize) {
        self.registers.v[0xF] = self.registers.v[x] & 1;
        self.registers.v[x] >>= 1;
    }

    // Set Vx = Vy - Vx, set VF = NOT borrow.
    /// 8xy7 - SUBN Vx, Vy
    fn op_8xy7(&mut self, x: usize, y: usize) {
        if self.registers.v[y] > self.registers.v[x] {
            self.registers.v[0xF] = 1
        }
        self.registers.v[x] = self.registers.v[y] - self.registers.v[x]
    }

    // 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    fn op_8xye(&mut self, x: usize) {
        // Checks is most significant bit == 1
        if (self.registers.v[x] >> 7) & 1 == 1 {
            self.registers.v[0xF] = 1
        }
        self.registers.v[x] <<= 1
    }

    // 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.registers.v[x] != self.registers.v[y] {
            self.registers.pc += 2
        }
    }

    // Annn - LD I, addr
    /// Set I = nnn.
    fn op_annn(&mut self, value: u16) {
        self.registers.i = value;
    }

    // Bnnn - JP V0, addr
    /// Jump to location nnn + V0.
    fn op_bnnn(&mut self, address: u16) {
        self.registers.pc = self.registers.v[0] as u16 + address
    }

    // Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.    
    fn op_cxkk(&mut self, x: usize, value: u8) {
        self.registers.v[x] = rand::random::<u8>() & value
    }

    // Dxyn - DRW Vx, Vy, nibble
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    fn op_dxyn(
        &mut self,
        x: usize,
        y: usize,
        sprite_height: u8,
        memory: &Memory,
        display: &mut Display,
    ) {
        let sprite_bytes = memory.read_bytes(self.registers.i as usize, sprite_height as usize);

        let pixels_collided = display.draw_sprite(
            sprite_bytes,
            self.registers.v[x] as usize,
            self.registers.v[y] as usize,
        );

        if pixels_collided {
            self.registers.v[0xF] = 1
        } else {
            self.registers.v[0xF] = 0
        }
    }

    // Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    fn op_ex9e(&mut self, x: usize, keypad: &Keypad) {
        if keypad.is_key_pressed(self.registers.v[x]) {
            self.registers.pc += 2
        }
    }

    // ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    fn op_exa1(&mut self, x: usize, keypad: &Keypad) {
        if !keypad.is_key_pressed(self.registers.v[x]) {
            self.registers.pc += 2
        }
    }

    // Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.    
    fn op_fx07(&mut self, x: usize) {
        self.registers.v[x] = self.registers.dt
    }

    // Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    fn op_fx0a(&mut self, x: usize, keypad: &Keypad) {
        if !keypad.is_any_key_pressed() {
            // Moving program counter backwards because it would skip this opcode otherwise
            self.registers.pc -= 2;
            return;
        }

        self.registers.v[x] = keypad.get_last_pressed_key();
    }

    // Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    fn op_fx15(&mut self, x: usize) {
        self.registers.dt = self.registers.v[x]
    }
}
