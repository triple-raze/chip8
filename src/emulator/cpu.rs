use super::Display;
use super::Keypad;
use super::Memory;

struct Registers {
    v: [u8; 16], // General registers from 0x0 to 0xF
    i: u16,      // Needed to store memory adresses
    pc: u16,     // Program counter, stores current memory address (u12).
}

impl Registers {
    fn new() -> Self {
        return Self {
            v: [0; 16],
            i: 0,
            pc: 0x200,
        };
    }
}

struct Timers {
    dt: u8, // Delay timer. Decrements 60 times per second
    st: u8, // Sound timer. Makes sound if its not equal to 0. Decrements 60 times per second
}

impl Timers {
    fn new() -> Self {
        Self { dt: 0, st: 0 }
    }
}

struct Stack {
    data: [u16; 16],
    pointer: usize, // Stack pointer. Stores next free index in stack.
}

impl Stack {
    fn new() -> Self {
        Self {
            data: [0; 16],
            pointer: 0,
        }
    }

    fn push(&mut self, value: u16) {
        self.data[self.pointer] = value;
        self.pointer += 1
    }

    fn pop(&mut self) -> u16 {
        self.pointer -= 1;
        let value = self.data[self.pointer];
        value
    }
}

pub struct Cpu {
    registers: Registers,
    timers: Timers,
    stack: Stack,
}

impl Cpu {
    pub fn new() -> Self {
        return Self {
            registers: Registers::new(),
            timers: Timers::new(),
            stack: Stack::new(),
        };
    }

    fn execute_opcode(
        &mut self,
        opcode: u16,
        memory: &mut Memory,
        display: &mut Display,
        keypad: &Keypad,
    ) {
        // First nibble which means a group of opcodes
        let group = (opcode & 0xF000) >> 12;

        let x = ((opcode & 0x0F00) >> 8) as usize; // Register with number "x"
        let y = ((opcode & 0x00F0) >> 4) as usize; // Register with number "y"
        let n = (opcode & 0x000F) as u8; // Value in last 1 nibble
        let kk = (opcode & 0x00FF) as u8; // Value in last 2 nibbles
        let nnn = opcode & 0x0FFF; // Value in last 3 nibbles

        match group {
            0x0 => match nnn {
                0x0E0 => self.op_00e0(display),
                0x0EE => self.op_00ee(),
                _ => self.op_0nnn(),
            },
            0x1 => self.op_1nnn(nnn),
            0x2 => self.op_2nnn(nnn),
            0x3 => self.op_3xkk(x, kk),
            0x4 => self.op_4xkk(x, kk),
            0x5 => self.op_5xy0(x, y),
            0x6 => self.op_6xkk(x, kk),
            0x7 => self.op_7xkk(x, kk),
            0x8 => match n {
                0x0 => self.op_8xy0(x, y),
                0x1 => self.op_8xy1(x, y),
                0x2 => self.op_8xy2(x, y),
                0x3 => self.op_8xy3(x, y),
                0x4 => self.op_8xy4(x, y),
                0x5 => self.op_8xy5(x, y),
                0x6 => self.op_8xy6(x),
                0x7 => self.op_8xy7(x, y),
                0xE => self.op_8xye(x),
                _ => panic!("Unknown opcode {:#X}", opcode),
            },
            0x9 => match n {
                0x0 => self.op_9xy0(x, y),
                _ => panic!("Unknown opcode {:#X}", opcode),
            },
            0xA => self.op_annn(nnn),
            0xB => self.op_bnnn(nnn),
            0xC => self.op_cxkk(x, kk),
            0xD => self.op_dxyn(x, y, n, memory, display),
            0xE => match kk {
                0x9E => self.op_ex9e(x, keypad),
                0xA1 => self.op_exa1(x, keypad),
                _ => panic!("Unknown opcode {:#X}", opcode),
            },
            0xF => match kk {
                0x07 => self.op_fx07(x),
                0x0A => self.op_fx0a(x, keypad),
                0x15 => self.op_fx15(x),
                0x18 => self.op_fx18(x),
                0x1E => self.op_fx1e(x),
                0x29 => self.op_fx29(x),
                0x33 => self.op_fx33(x, memory),
                0x55 => self.op_fx55(x, memory),
                0x65 => self.op_fx65(x, memory),
                _ => panic!("Unknown opcode {:#X}", opcode),
            },
            _ => panic!("Unknown opcode {:#X}", opcode),
        }
    }

    /// Makes one CPU cycle
    pub fn cycle(&mut self, memory: &mut Memory, display: &mut Display, keypad: &Keypad) {
        // Reading current opcode
        let opcode = memory.read_word(self.registers.pc as usize);

        // Moving to next opcode
        self.registers.pc += 2;

        // Executing one opcode
        self.execute_opcode(opcode, memory, display, keypad);
    }

    pub fn decrement_timers(&mut self) {
        if self.timers.dt > 0 {
            self.timers.dt -= 1;
        }

        if self.timers.st > 0 {
            self.timers.st -= 1;
        }
    }

    // 0nnn - SYS addr
    /// Ignored by modern interpreters.
    fn op_0nnn(&self) {}

    // 00E0 - CLS
    /// Clear the display.
    fn op_00e0(&self, display: &mut Display) {
        display.clear();
    }

    fn op_00ee(&mut self) {
        self.registers.pc = self.stack.pop();
    }

    // 1nnn - JP addr
    /// Jump to location nnn (u12).
    fn op_1nnn(&mut self, address: u16) {
        self.registers.pc = address;
    }

    // 2nnn - CALL addr
    /// Call subroutine at nnn (u12).
    fn op_2nnn(&mut self, address: u16) {
        self.stack.push(self.registers.pc);
        self.registers.pc = address
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
        } else {
            self.registers.v[0xF] = 0
        }
    }

    // 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    fn op_8xy5(&mut self, x: usize, y: usize) {
        if self.registers.v[x] >= self.registers.v[y] {
            self.registers.v[0xF] = 1;
        } else {
            self.registers.v[0xF] = 0
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
        } else {
            self.registers.v[0xF] = 0
        }
        self.registers.v[x] = self.registers.v[y].wrapping_sub(self.registers.v[x])
    }

    // 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    fn op_8xye(&mut self, x: usize) {
        // Checks is most significant bit == 1
        self.registers.v[0xF] = self.registers.v[x] >> 7 & 1;

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
        self.registers.v[x] = self.timers.dt
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
        self.timers.dt = self.registers.v[x]
    }

    // Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    fn op_fx18(&mut self, x: usize) {
        self.timers.st = self.registers.v[x]
    }

    // Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    fn op_fx1e(&mut self, x: usize) {
        self.registers.i += self.registers.v[x] as u16
    }

    // Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    fn op_fx29(&mut self, x: usize) {
        // There is only digits from 0 to F
        if self.registers.v[x] > 0xF {
            panic!(
                "Register V{} with value {} is too big (should be < 16)",
                x, self.registers.v[x]
            )
        }

        // Each digit sprite uses 5 bytes of memory
        self.registers.i = 5 * self.registers.v[x] as u16;
    }

    // Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.    
    fn op_fx33(&mut self, x: usize, memory: &mut Memory) {
        // 123 // 100 = 1
        let hundreds = self.registers.v[x] / 100;
        // (123 // 10) % 10 = 12 % 10 = 2
        let tens = (self.registers.v[x] / 10) % 10;
        // 123 % 10 = 3
        let ones = self.registers.v[x] % 10;

        let idx = self.registers.i as usize;

        memory.write_byte(idx, hundreds);
        memory.write_byte(idx + 1, tens);
        memory.write_byte(idx + 2, ones);
    }

    // Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    fn op_fx55(&self, x: usize, memory: &mut Memory) {
        for idx in 0..=x {
            let offset = self.registers.i as usize + idx;
            memory.write_byte(offset, self.registers.v[idx]);
        }
    }

    // Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.    
    fn op_fx65(&mut self, x: usize, memory: &Memory) {
        for idx in 0..=x {
            let offset = self.registers.i as usize + idx;
            self.registers.v[idx] = memory.read_byte(offset);
        }
    }
}
