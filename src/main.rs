// based on http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

extern crate rand;
use rand::random;

const MEMSIZE: usize = 4096;
const DISPWIDTH: usize = 64;
const DISPHEIGHT: usize = 32;
const DISPSIZE: usize = DISPWIDTH * DISPHEIGHT;
const STACKSIZE: usize = 16;
const NUM_REGS: usize = 16;
const NUM_KEYS: usize = 16;

const FONTSET: [u8; 80] =
[ 
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

#[derive(Debug, PartialEq)]
enum Chip8Op {
    DisplayClear,
    Return,
    Jump(usize),
    Call(usize),
    CmpEqualConst(usize, u8),
    CmpNotEqualConst(usize, u8),
    CmpEqualReg(usize, usize),
    SetRegConst(usize, u8),
    AddConstReg(usize, u8),
    SetRegReg(usize, usize),
    BitOpOr(usize, usize),
    BitOpAnd(usize, usize),
    BitOpXor(usize, usize),
    MathOpAdd(usize, usize),
    MathOpSub(usize, usize),
    BitOpShiftRight(usize, usize),
    MathOpSubNeg(usize, usize),
    BitOpShiftLeft(usize, usize),
    CmpNotEqualReg(usize, usize),
    SetMemoryAddress(usize),
    JumpPlus(usize),
    Random(usize, u8),
    DrawSprite(usize, usize, u8),
    KeyPressed(usize),
    KeyNotPressed(usize),
    GetDelay(usize),
    GetKey(usize),
    SetDelay(u8),
    SetSound(u8),
    AddMemoryAddress(usize),
    GetSprite(usize),
    BinaryCoding(usize),
    RegisterDump(usize),
    RegisterLoad(usize),
}

fn x_nibble(opcode: u16) -> u8 {
    ((opcode & 0x0F00) >> 8) as u8
}

fn y_nibble(opcode: u16) -> u8 {
    ((opcode & 0x00F0) >> 4) as u8
}

fn nn_byte(opcode: u16) -> u8 {
    (opcode & 0x00FF) as u8
}

fn nnn_word(opcode: u16) -> u16 {
    opcode & 0x0FFF
}

fn first_nibble(opcode: u16) -> u8 {
    (opcode >> 12) as u8
}

fn last_nibble(opcode: u16) -> u8 {
    (opcode & 0x000F) as u8
}

#[allow(non_snake_case)]
fn aXYb(opcode: u16, a: u8, b: u8) -> bool {
    first_nibble(opcode) == a && last_nibble(opcode) == b
}

#[allow(non_snake_case)]
fn aXbb(opcode: u16, a: u8, b: u8) -> bool {
    first_nibble(opcode) == a && nn_byte(opcode) == b
}

fn decode_opcode(opcode: u16) -> Chip8Op {
    let x = x_nibble(opcode) as usize;
    let y = y_nibble(opcode) as usize;
    let word = nnn_word(opcode) as usize;
    match opcode {
        0x00E0 => Chip8Op::DisplayClear,
        0x00EE => Chip8Op::Return,
        o if first_nibble(o) == 0x1 => Chip8Op::Jump(word),
        o if first_nibble(o) == 0x2 => Chip8Op::Call(word),
        o if first_nibble(o) == 0x3 => Chip8Op::CmpEqualConst(x, nn_byte(o)),
        o if first_nibble(o) == 0x4 => Chip8Op::CmpNotEqualConst(x, nn_byte(o)),
        o if aXYb(o, 0x5, 0x0) => Chip8Op::CmpEqualReg(x, y),
        o if first_nibble(o) == 0x6 => Chip8Op::SetRegConst(x, nn_byte(o)),
        o if first_nibble(o) == 0x7 => Chip8Op::AddConstReg(x, nn_byte(o)),
        o if aXYb(o, 0x8, 0x0) => Chip8Op::SetRegReg(x, y),
        o if aXYb(o, 0x8, 0x1) => Chip8Op::BitOpOr(x, y),
        o if aXYb(o, 0x8, 0x2) => Chip8Op::BitOpAnd(x, y),
        o if aXYb(o, 0x8, 0x3) => Chip8Op::BitOpXor(x, y),
        o if aXYb(o, 0x8, 0x4) => Chip8Op::MathOpAdd(x, y),
        o if aXYb(o, 0x8, 0x5) => Chip8Op::MathOpSub(x, y),
        o if aXYb(o, 0x8, 0x6) => Chip8Op::BitOpShiftRight(x, y),
        o if aXYb(o, 0x8, 0x7) => Chip8Op::MathOpSubNeg(x, y),
        o if aXYb(o, 0x8, 0xE) => Chip8Op::BitOpShiftLeft(x, y),
        o if aXYb(o, 0x9, 0x0) => Chip8Op::CmpNotEqualReg(x, y),
        o if first_nibble(o) == 0xA => Chip8Op::SetMemoryAddress(word),
        o if first_nibble(o) == 0xB => Chip8Op::JumpPlus(word),
        o if first_nibble(o) == 0xC => Chip8Op::Random(x, nn_byte(o)),
        o if first_nibble(o) == 0xD => Chip8Op::DrawSprite(x, y, last_nibble(o)),
        o if aXbb(o, 0xE, 0x9E) => Chip8Op::KeyPressed(x),
        o if aXbb(o, 0xE, 0xA1) => Chip8Op::KeyNotPressed(x),
        o if aXbb(o, 0xF, 0x07) => Chip8Op::GetDelay(x),
        o if aXbb(o, 0xF, 0x0A) => Chip8Op::GetKey(x),
        o if aXbb(o, 0xF, 0x15) => Chip8Op::SetDelay(x as u8),
        o if aXbb(o, 0xF, 0x18) => Chip8Op::SetSound(x as u8),
        o if aXbb(o, 0xF, 0x1E) => Chip8Op::AddMemoryAddress(x),
        o if aXbb(o, 0xF, 0x29) => Chip8Op::GetSprite(x),
        o if aXbb(o, 0xF, 0x33) => Chip8Op::BinaryCoding(x),
        o if aXbb(o, 0xF, 0x55) => Chip8Op::RegisterDump(x),
        o if aXbb(o, 0xF, 0x65) => Chip8Op::RegisterLoad(x),
        _ => panic!(format!("invalid opcode {:x}", opcode)),
    }
}

struct Chip8 {
    memory: [u8; MEMSIZE],
    registers: [u8; NUM_REGS],
    stack: [usize; STACKSIZE],
    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; NUM_KEYS],
    display: [bool; DISPSIZE],
    draw_flag: bool,
    pc: usize, // program counter
    sp: usize, // stack pointer
    ma: usize, // memory address
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {memory: [0; MEMSIZE],
                registers: [0; NUM_REGS],
                stack: [0; STACKSIZE],
                delay_timer: 0,
                sound_timer: 0,
                keys: [false; NUM_KEYS],
                display: [false; DISPSIZE],
                draw_flag: false,
                pc: 0x200,
                sp: 0,
                ma: 0,
            }
    }
    pub fn load_fonts(&mut self) {
        for i in 0..80 {
            self.memory[i] = FONTSET[i];
        }
    }
    pub fn load(&mut self, filename: &str) {
        unimplemented!();
    }
    pub fn emulate_cycle(&mut self) {
        self.draw_flag = false;

        let opcode = self.fetch_opcode();
        let op = decode_opcode(opcode);
        self.execute_op(op);

        if self.delay_timer > 0 { self.delay_timer -= 1; }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
            if self.sound_timer == 0 { beep(); }
        }
    }
    pub fn set_keys(&mut self) {
        unimplemented!();
    }
    fn fetch_opcode(&mut self) -> u16 {
        let opcode = ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16);
        self.pc += 2;
        return opcode;
    }
    fn no_advance(&mut self) {
        self.pc -= 2;
    }
    fn execute_op(&mut self, op: Chip8Op) {
        match op {
            Chip8Op::DisplayClear => {
                self.display = [false; DISPSIZE];
                self.draw_flag = true;
            },
            Chip8Op::Return => {
                self.no_advance();
                self.pc = self.stack[self.sp];
                self.sp -= 1;
            },
            Chip8Op::Jump(addr) => {
                self.no_advance();
                self.pc = addr;
            },
            Chip8Op::Call(addr) => {
                self.no_advance();
                self.sp += 1;
                self.stack[self.sp] = self.pc + 2;
                self.pc = addr;
            },
            Chip8Op::CmpEqualConst(x, c) => {
                if self.registers[x] == c { self.pc += 2; }
            },
            Chip8Op::CmpNotEqualConst(x, c) => {
                if self.registers[x] != c { self.pc += 2; }
            },
            Chip8Op::CmpEqualReg(x, y) => {
                if self.registers[x] == self.registers[y] { self.pc += 2; }
            },
            Chip8Op::SetRegConst(x, c) => {
                self.registers[x] = c;
            },
            Chip8Op::AddConstReg(x, c) => {
                self.registers[x] += c;
            },
            Chip8Op::SetRegReg(x, y) => {
                self.registers[x] = self.registers[y];
            },
            Chip8Op::BitOpOr(x, y) => {
                self.registers[x] = self.registers[x] | self.registers[y];
            },
            Chip8Op::BitOpAnd(x, y) => {
                self.registers[x] = self.registers[x] & self.registers[y];
            },
            Chip8Op::BitOpXor(x, y) => {
                self.registers[x] = self.registers[x] ^ self.registers[y];
            },
            Chip8Op::MathOpAdd(x, y) => {
                self.registers[x] = self.registers[x] + self.registers[y];
            },
            Chip8Op::MathOpSub(x, y) => {
                self.registers[x] = self.registers[x] - self.registers[y];
            },
            Chip8Op::BitOpShiftRight(x, y) => {
                let lsb = self.registers[y] & 0x01;
                self.registers[x] = self.registers[y] >> 1;
                self.registers[y] = self.registers[y] >> 1;
                self.registers[0x0F] = lsb;
            },
            Chip8Op::MathOpSubNeg(x, y) => {
                self.registers[x] = self.registers[y] - self.registers[x];
            },
            Chip8Op::BitOpShiftLeft(x, y) => {
                let msb = self.registers[y] & 0x80;
                self.registers[x] = self.registers[y] << 1;
                self.registers[y] = self.registers[y] << 1;
                self.registers[0x0F] = msb;
            },
            Chip8Op::CmpNotEqualReg(x, y) => {
                if self.registers[x] != self.registers[y] { self.pc += 2; }
            },
            Chip8Op::SetMemoryAddress(addr) => {
                self.ma = addr;
            },
            Chip8Op::JumpPlus(addr) => {
                self.no_advance();
                self.pc = addr + self.registers[0x00] as usize;
            },
            Chip8Op::Random(x, mask) => {
                self.registers[x] = random::<u8>() & mask;
            },
            Chip8Op::DrawSprite(x, y, h) => {
                let left = self.registers[x] as usize;
                let top = self.registers[y] as usize;
                let mut collision = false;
                for row in 0..(h as usize) {
                    let sprite_row = self.memory[self.ma + row];
                    for offset in 0..8 {
                        let sprite_bit = ((0x1 << offset) & sprite_row) > 0;
                        let pixel_index = (top + row) * DISPWIDTH + (left + offset);
                        let pixel_val = self.display[pixel_index];
                        if pixel_val && sprite_bit { collision = true; }
                        self.display[pixel_index] = sprite_bit ^ pixel_val;
                    }
                }
                if collision {
                    self.registers[0xF] = 0x1;
                } else {
                    self.registers[0xF] = 0x0;
                }
            },
            Chip8Op::KeyPressed(x) => {
                if self.keys[self.registers[x] as usize] { self.pc += 2; }
            },
            Chip8Op::KeyNotPressed(x) => {
                if !self.keys[self.registers[x] as usize] { self.pc += 2; }
            },
            Chip8Op::GetDelay(x) => {
                self.registers[x] = self.delay_timer;
            },
            Chip8Op::GetKey(x) => {
                unimplemented!();
            },
            Chip8Op::SetDelay(c) => {
                self.delay_timer = c;
            },
            Chip8Op::SetSound(c) => {
                self.sound_timer = c;
            },
            Chip8Op::AddMemoryAddress(x) => {
                self.ma += self.registers[x] as usize;
            },
            Chip8Op::GetSprite(x) => {
                self.ma = (5 * self.registers[x]) as usize;
            },
            Chip8Op::BinaryCoding(x) => {
                self.memory[self.ma] = (x / 100) as u8;
                self.memory[self.ma + 1] = ((x / 10) % 10) as u8;
                self.memory[self.ma + 2] = (x % 10) as u8;
            },
            Chip8Op::RegisterDump(x) => {
                for i in 0..x {
                    self.memory[self.ma] = self.registers[i];
                    self.ma += 1;
                }
            },
            Chip8Op::RegisterLoad(x) => {
                for i in 0..x {
                    self.registers[i] = self.memory[self.ma];
                    self.ma += 1;
                }
            },
        }
    }
}

fn draw_graphics(display: [bool; DISPSIZE]) {
    unimplemented!();
}

fn beep() {
    unimplemented!();
}

fn main() {

    let mut chip8 = Chip8::new();
    chip8.load_fonts();
    chip8.load("pong");

    loop {
        chip8.emulate_cycle();

        if chip8.draw_flag {
            draw_graphics(chip8.display);
        }

        chip8.set_keys();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_nibble() {
        assert_eq!(first_nibble(0x1234), 0x1);
    }
    #[test]
    fn test_x_nibble() {
        assert_eq!(x_nibble(0x1234), 0x2);
    }
    #[test]
    fn decode_jump() {
        assert_eq!(decode_opcode(0x1456), Chip8Op::Jump(0x0456));
    }
}