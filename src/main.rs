// based on http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

const MEMSIZE: usize = 4096;
const DISPSIZE: usize = 32 * 64;
const STACKSIZE: usize = 16;
const NUM_REGS: usize = 16;
const NUM_KEYS: usize = 16;

#[derive(Debug, PartialEq)]
enum Chip8Op {
    DisplayClear,
    Return,
    Jump(u16),
    Call(u16),
    CmpEqualConst(u8, u8),
    CmpNotEqualConst(u8, u8),
    CmpEqualReg(u8, u8),
    SetRegConst(u8, u8),
    AddConstReg(u8, u8),
    SetRegReg(u8, u8),
    BitOpOr(u8, u8),
    BitOpAnd(u8, u8),
    BitOpXor(u8, u8),
    MathOpAdd(u8, u8),
    MathOpSub(u8, u8),
    BitOpShiftRight(u8, u8),
    MathOpSubNeg(u8, u8),
    BitOpShiftLeft(u8, u8),
    CmpNotEqualReg(u8, u8),
    SetMemoryAddress(u16),
    JumpPlus(u16),
    Random(u8, u8),
    DrawSprite(u8, u8, u8),
    KeyPressed(u8),
    KeyNotPressed(u8),
    GetDelay(u8),
    GetKey(u8),
    SetDelay(u8),
    SetSound(u8),
    AddMemoryAddress(u8),
    GetSprite(u8),
    BinaryCoding(u8),
    RegisterDump(u8),
    RegisterLoad(u8),
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

fn aXYb(opcode: u16, a: u8, b: u8) -> bool {
    first_nibble(opcode) == a && last_nibble(opcode) == b
}

fn aXbb(opcode: u16, a: u8, b: u8) -> bool {
    first_nibble(opcode) == a && nn_byte(opcode) == b
}

fn decode_opcode(opcode: u16) -> Chip8Op {
    match opcode {
        0x00E0 => Chip8Op::DisplayClear,
        0x00EE => Chip8Op::Return,
        x if first_nibble(x) == 0x1 => Chip8Op::Jump(nnn_word(x)),
        x if first_nibble(x) == 0x2 => Chip8Op::Call(nnn_word(x)),
        x if first_nibble(x) == 0x3 => Chip8Op::CmpEqualConst(x_nibble(x), nn_byte(x)),
        x if first_nibble(x) == 0x4 => Chip8Op::CmpNotEqualConst(x_nibble(x), nn_byte(x)),
        x if aXYb(x, 0x5, 0x0) => Chip8Op::CmpEqualReg(x_nibble(x), y_nibble(x)),
        x if first_nibble(x) == 0x6 => Chip8Op::SetRegConst(x_nibble(x), nn_byte(x)),
        x if first_nibble(x) == 0x7 => Chip8Op::AddConstReg(x_nibble(x), nn_byte(x)),
        x if aXYb(x, 0x8, 0x0) => Chip8Op::SetRegReg(x_nibble(x), y_nibble(x)),
        x if aXYb(x, 0x8, 0x1) => Chip8Op::BitOpOr(x_nibble(x), y_nibble(x)),
        x if aXYb(x, 0x8, 0x2) => Chip8Op::BitOpAnd(x_nibble(x), y_nibble(x)),
        x if aXYb(x, 0x8, 0x3) => Chip8Op::BitOpXor(x_nibble(x), y_nibble(x)),
        x if aXYb(x, 0x8, 0x4) => Chip8Op::MathOpAdd(x_nibble(x), y_nibble(x)),
        x if aXYb(x, 0x8, 0x5) => Chip8Op::MathOpSub(x_nibble(x), y_nibble(x)),
        x if aXYb(x, 0x8, 0x6) => Chip8Op::BitOpShiftRight(x_nibble(x), y_nibble(x)),
        x if aXYb(x, 0x8, 0x7) => Chip8Op::MathOpSubNeg(x_nibble(x), y_nibble(x)),
        x if aXYb(x, 0x8, 0xE) => Chip8Op::BitOpShiftLeft(x_nibble(x), y_nibble(x)),
        x if aXYb(x, 0x9, 0x0) => Chip8Op::CmpNotEqualReg(x_nibble(x), y_nibble(x)),
        x if first_nibble(x) == 0xA => Chip8Op::SetMemoryAddress(nnn_word(x)),
        x if first_nibble(x) == 0xB => Chip8Op::JumpPlus(nnn_word(x)),
        x if first_nibble(x) == 0xC => Chip8Op::Random(x_nibble(x), nn_byte(x)),
        x if first_nibble(x) == 0xD => Chip8Op::DrawSprite(x_nibble(x), y_nibble(x), last_nibble(x)),
        x if aXbb(x, 0xE, 0x9E) => Chip8Op::KeyPressed(x_nibble(x)),
        x if aXbb(x, 0xE, 0xA1) => Chip8Op::KeyNotPressed(x_nibble(x)),
        x if aXbb(x, 0xF, 0x07) => Chip8Op::GetDelay(x_nibble(x)),
        x if aXbb(x, 0xF, 0x0A) => Chip8Op::GetKey(x_nibble(x)),
        x if aXbb(x, 0xF, 0x15) => Chip8Op::SetDelay(x_nibble(x)),
        x if aXbb(x, 0xF, 0x18) => Chip8Op::SetSound(x_nibble(x)),
        x if aXbb(x, 0xF, 0x1E) => Chip8Op::AddMemoryAddress(x_nibble(x)),
        x if aXbb(x, 0xF, 0x29) => Chip8Op::GetSprite(x_nibble(x)),
        x if aXbb(x, 0xF, 0x33) => Chip8Op::BinaryCoding(x_nibble(x)),
        x if aXbb(x, 0xF, 0x55) => Chip8Op::RegisterDump(x_nibble(x)),
        x if aXbb(x, 0xF, 0x65) => Chip8Op::RegisterLoad(x_nibble(x)),
        _ => panic!(format!("invalid opcode {:x}", opcode)),
    }
}

struct Chip8 {
    memory: [u8; MEMSIZE],
    registers: [u8; NUM_REGS],
    stack: [u16; STACKSIZE],
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

    fn fetch_opcode(&self) -> u16 {
        ((self.memory[self.pc] as u16) << 8) | (self.memory[self.pc + 1] as u16)
    }
    fn execute_op(&mut self, op: Chip8Op) {
        match op {
            Chip8Op::DisplayClear => {
                self.display = [false; DISPSIZE];
                self.pc += 2;
            },
            Chip8Op::Return => {
                self.pc = self.stack[self.sp] as usize;
                self.sp -= 1;
            },
            Chip8Op::Jump(addr) => {
                self.pc = addr as usize;
            },
            Chip8Op::Call(addr) => {
                self.sp += 1;
                self.stack[self.sp] = self.pc as u16 + 2;
                self.pc = addr as usize;
            },
            Chip8Op::CmpEqualConst(x, c) => {
                if self.registers[x as usize] == c { self.pc += 4; }
                else { self.pc += 2; }
            },
            Chip8Op::CmpNotEqualConst(x, c) => {
                if self.registers[x as usize] != c { self.pc += 4; }
                else { self.pc += 2; }
            },
            Chip8Op::CmpEqualReg(x, y) => {
                if self.registers[x as usize] == self.registers[y as usize] { self.pc += 4; }
                else { self.pc += 2; }
            },
            Chip8Op::SetRegConst(x, c) => {
                self.registers[x as usize] = c;
                self.pc += 2;
            },
            Chip8Op::AddConstReg(x, c) => {
                self.registers[x as usize] += c;
                self.pc += 2;
            },
            Chip8Op::SetRegReg(x, y) => {
                self.registers[x as usize] = self.registers[y as usize];
                self.pc += 2;
            },
            Chip8Op::BitOpOr(x, y) => {
                self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
                self.pc += 2;
            },
            Chip8Op::BitOpAnd(x, y) => {
                self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
                self.pc += 2;
            },
            Chip8Op::BitOpXor(x, y) => {
                self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
                self.pc += 2;
            },
            Chip8Op::MathOpAdd(x, y) => {
                self.registers[x as usize] = self.registers[x as usize] + self.registers[y as usize];
                self.pc += 2;
            },
            Chip8Op::MathOpSub(x, y) => {
                self.registers[x as usize] = self.registers[x as usize] - self.registers[y as usize];
                self.pc += 2;
            },
            Chip8Op::BitOpShiftRight(x, y) => {
                let lsb = self.registers[y as usize] & 0x01;
                self.registers[x as usize] = self.registers[y as usize] >> 1;
                self.registers[y as usize] = self.registers[y as usize] >> 1;
                self.registers[0x0F] = lsb;
                self.pc += 2;
            },
            Chip8Op::MathOpSubNeg(x, y) => {
                self.registers[x as usize] = self.registers[y as usize] - self.registers[x as usize];
                self.pc += 2;
            },
            Chip8Op::BitOpShiftLeft(x, y) => {
                let msb = self.registers[y as usize] & 0x80;
                self.registers[x as usize] = self.registers[y as usize] << 1;
                self.registers[y as usize] = self.registers[y as usize] << 1;
                self.registers[0x0F] = msb;
                self.pc += 2;
            },
            Chip8Op::CmpNotEqualReg(x, y) => {
                if self.registers[x as usize] != self.registers[y as usize] { self.pc += 4; }
                else { self.pc += 2; }
            },
            Chip8Op::SetMemoryAddress(addr) => {
                self.ma = addr as usize;
                self.pc += 2;
            },
            Chip8Op::JumpPlus(addr) => {
                self.pc = (addr + self.registers[0x00] as u16) as usize;
            },
            Chip8Op::Random(x, mask) => {unimplemented!()},
            Chip8Op::DrawSprite(x, y, h) => {unimplemented!()},
            Chip8Op::KeyPressed(x) => {
                if self.keys[self.registers[x as usize] as usize] { self.pc += 4; }
                else { self.pc += 2; }
            },
            Chip8Op::KeyNotPressed(x) => {
                if !self.keys[self.registers[x as usize] as usize] { self.pc += 4; }
                else { self.pc += 2; }
            },
            Chip8Op::GetDelay(x) => {
                self.registers[x as usize] = self.delay_timer;
                self.pc += 2;
            },
            Chip8Op::GetKey(x) => {
                unimplemented!();
            },
            Chip8Op::SetDelay(c) => {
                self.delay_timer = c;
                self.pc += 2;
            },
            Chip8Op::SetSound(c) => {
                self.sound_timer = c;
                self.pc += 2;
            },
            Chip8Op::AddMemoryAddress(x) => {
                self.ma += self.registers[x as usize] as usize;
                self.pc += 2;
            },
            Chip8Op::GetSprite(x) => {unimplemented!()},
            Chip8Op::BinaryCoding(x) => {unimplemented!()},
            Chip8Op::RegisterDump(x) => {unimplemented!()},
            Chip8Op::RegisterLoad(x) => {unimplemented!()},
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