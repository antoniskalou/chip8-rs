use crate::memory::Memory;
use crate::screen::Screen;

use rand::Rng;

fn u8_from_nibbles(n1: u8, n2: u8) -> u8 {
    (n1 << 4) | n2
}

#[test]
fn test_u8_from_nibbles() {
    assert_eq!(0xAB, u8_from_nibbles(0xA, 0xB));
}

fn u16_from_nibbles(n1: u8, n2: u8, n3: u8, n4: u8) -> u16 {
    ((n1 as u16) << 12) | ((n2 as u16) << 8) | ((n3 as u16) << 4) | n4 as u16
}

#[test]
fn test_u16_from_nibbles() {
    assert_eq!(0xABCD, u16_from_nibbles(0xA, 0xB, 0xC, 0xD));
}

fn u8_to_nibbles(i: u8) -> (u8, u8) {
    ((0xF0 & i) >> 4, 0x0F & i)
}

#[test]
fn test_u8_to_nibbles() {
    let (n1, n2) = u8_to_nibbles(0xAB);
    assert_eq!(n1, 0xA);
    assert_eq!(n2, 0xB);
}

fn u16_to_nibbles(i: u16) -> (u8, u8, u8, u8) {
    (
        ((0xF000 & i) >> 12) as u8,
        ((0x0F00 & i) >> 8) as u8,
        ((0x00F0 & i) >> 4) as u8,
        0x000F & i as u8,
    )
}

#[test]
fn test_u16_to_nibbles() {
    let (n1, n2, n3, n4) = u16_to_nibbles(0xABCD);
    assert_eq!(n1, 0xA);
    assert_eq!(n2, 0xB);
    assert_eq!(n3, 0xC);
    assert_eq!(n4, 0xD);
}

const NUM_REGS: usize = 16;
const NUM_KEYS: usize = 16;
// screen always has 64x32, it can be upscaled by the renderer
const SCREEN_SIZE: usize = 64 * 32;

#[derive(Debug)]
pub struct CPU {
    // TODO: consider type alias for register
    /// V registers
    v: [u8; NUM_REGS],
    /// I register
    i: u16,
    /// program counter
    pc: u16,
    /// stack pointer
    sp: u16,
    /// delay timer
    dt: u8,
    /// sound timer
    st: u8,
    /// currently pressed keys
    keys: [bool; NUM_KEYS],
    /// RAM
    memory: Memory,
    /// screen buffer
    screen: Screen,
}

#[derive(Debug)]
enum Instruction {
    Clear,
    Return,
    Set(u8, u8),
    SetIndex(u16),
    SetDelay(u8),
    SetSound(u8),
    SetVxToVy(u8, u8),
    SetFont(u8),
    ReadDelay(u8),
    Random(u8, u8),
    Add(u8, u8),
    AddToIndex(u8),
    AddVxToVy(u8, u8),
    SubtractVyFromVx(u8, u8),
    SubtractVxFromVy(u8, u8),
    ShiftRight(u8, u8),
    ShiftLeft(u8, u8),
    BinaryOr(u8, u8),
    BinaryAnd(u8, u8),
    BinaryXor(u8, u8),
    Bcd(u8),
    Jump(u16),
    JumpV0(u16),
    Call(u16),
    SkipIfEq(u8, u8),
    SkipIfNe(u8, u8),
    SkipIfVxVyEq(u8, u8),
    SkipIfVxVyNe(u8, u8),
    SkipIfPressed(u8),
    SkipIfNotPressed(u8),
    Draw(u8, u8, u8),
    Load(u8),
    Store(u8),
    WaitUntilPressed(u8),
}

impl CPU {
    pub fn new(memory: Memory) -> Self {
        Self {
            v: [0; NUM_REGS],
            i: 0,
            pc: 0x200,
            sp: 0xFA0,
            dt: 0,
            st: 0,
            keys: [false; NUM_KEYS],
            memory,
            screen: Screen::new(),
        }
    }

    pub fn press_key(&mut self, key: usize, pressed: bool) {
        self.keys[key] = pressed;
    }

    pub fn screen_buffer(&self) -> &[bool] {
        self.screen.buffer()
    }

    pub fn is_sound_playing(&self) -> bool {
        self.st > 2
    }

    fn fetch(&mut self) -> u16 {
        let opcode = self.memory.read_u16(self.pc);
        self.pc += 2;
        opcode
    }

    fn decode(&self, opcode: u16) -> Instruction {
        use Instruction::*;
        match u16_to_nibbles(opcode) {
            (0x0, 0x0, 0xE, 0x0) => Clear,
            (0x0, 0x0, 0xE, 0xE) => Return,
            (0x1, n1, n2, n3) => Jump(u16_from_nibbles(0x0, n1, n2, n3)),
            (0x2, n1, n2, n3) => Call(u16_from_nibbles(0x0, n1, n2, n3)),
            (0x3, x, n1, n2) => SkipIfEq(x, u8_from_nibbles(n1, n2)),
            (0x4, x, n1, n2) => SkipIfNe(x, u8_from_nibbles(n1, n2)),
            (0x5, x, y, 0x0) => SkipIfVxVyEq(x, y),
            (0x6, x, n1, n2) => Set(x, u8_from_nibbles(n1, n2)),
            (0x7, x, n1, n2) => Add(x, u8_from_nibbles(n1, n2)),
            (0x8, x, y, 0x0) => SetVxToVy(x, y),
            (0x8, x, y, 0x1) => BinaryOr(x, y),
            (0x8, x, y, 0x2) => BinaryAnd(x, y),
            (0x8, x, y, 0x3) => BinaryXor(x, y),
            (0x8, x, y, 0x4) => AddVxToVy(x, y),
            (0x8, x, y, 0x5) => SubtractVyFromVx(x, y),
            (0x8, x, y, 0x6) => ShiftRight(x, y),
            (0x8, x, y, 0x7) => SubtractVxFromVy(x, y),
            (0x8, x, y, 0xE) => ShiftLeft(x, y),
            (0x9, x, y, 0x0) => SkipIfVxVyNe(x, y),
            (0xA, n1, n2, n3) => SetIndex(u16_from_nibbles(0x0, n1, n2, n3)),
            (0xB, n1, n2, n3) => JumpV0(u16_from_nibbles(0x0, n1, n2, n3)),
            (0xC, x, n1, n2) => Random(x, u8_from_nibbles(n1, n2)),
            (0xD, x, y, n) => Draw(x, y, n),
            (0xE, x, 0xA, 0x1) => SkipIfNotPressed(x),
            (0xE, x, 0x9, 0xE) => SkipIfPressed(x),
            (0xF, x, 0x0, 0x7) => ReadDelay(x),
            (0xF, x, 0x0, 0xA) => WaitUntilPressed(x),
            (0xF, x, 0x1, 0x5) => SetDelay(x),
            (0xF, x, 0x1, 0x8) => SetSound(x),
            (0xF, x, 0x1, 0xE) => AddToIndex(x),
            (0xF, x, 0x2, 0x9) => SetFont(x),
            (0xF, x, 0x3, 0x3) => Bcd(x),
            (0xF, x, 0x5, 0x5) => Store(x),
            (0xF, x, 0x6, 0x5) => Load(x),
            _ => panic!("Invalid opcode: 0x{:04X}", opcode),
        }
    }

    fn execute(&mut self, inst: Instruction) {
        use Instruction::*;
        match inst {
            Clear => self.screen.clear(),
            Set(vx, val) => self.v[vx as usize] = val,
            SetIndex(val) => self.i = val,
            SetDelay(vx) => self.dt = self.v[vx as usize],
            SetSound(vx) => self.st = self.v[vx as usize],
            SetVxToVy(vx, vy) => self.v[vx as usize] = self.v[vy as usize],
            SetFont(vx) => {
                let x = self.v[vx as usize];
                self.i = x as u16 * 5;
            },
            ReadDelay(vx) => self.v[vx as usize] = self.dt,
            Random(vx, val) => {
                let rand = rand::thread_rng().gen_range(0..=0xFF);
                self.v[vx as usize] = rand & val;
            },
            Add(vx, val) => {
                let (x, _) = self.v[vx as usize].overflowing_add(val);
                self.v[vx as usize] = x;
            },
            AddToIndex(vx) => {
                self.i += self.v[vx as usize] as u16;
                if self.i >= 0x1000 {
                    self.v[0xF] = 1;
                }
            },
            AddVxToVy(vx, vy) => {
                let x = self.v[vx as usize];
                let y = self.v[vy as usize];
                let (new_x, overflow) = x.overflowing_add(y);
                self.v[vx as usize] = new_x;
                self.v[0xF] = overflow as u8;
            },
            SubtractVyFromVx(vx, vy) => {
                let x = self.v[vx as usize];
                let y = self.v[vy as usize];
                let (new_x, overflow) = x.overflowing_sub(y);
                self.v[vx as usize] = new_x;
                self.v[0xF] = !overflow as u8;
            },
            SubtractVxFromVy(vx, vy) => {
                let x = self.v[vx as usize];
                let y = self.v[vy as usize];
                let (new_x, overflow) = y.overflowing_sub(x);
                self.v[vx as usize] = new_x;
                self.v[0xF] = !overflow as u8;
            },
            ShiftRight(vx, vy) => {
                let y = self.v[vy as usize];
                self.v[vx as usize] = y >> 1;
                self.v[0xF] = y & 1;
            },
            ShiftLeft(vx, vy) => {
                let y = self.v[vy as usize];
                self.v[vx as usize] = y << 1;
                self.v[0xF] = y >> 7;
            },
            BinaryOr(vx, vy) => {
                let x = self.v[vx as usize];
                let y = self.v[vy as usize];
                self.v[0xF] = 0;
                self.v[vx as usize] = x | y;
            },
            BinaryAnd(vx, vy) => {
                let x = self.v[vx as usize];
                let y = self.v[vy as usize];
                self.v[0xF] = 0;
                self.v[vx as usize] = x & y;
            },
            BinaryXor(vx, vy) => {
                let x = self.v[vx as usize];
                let y = self.v[vy as usize];
                self.v[0xF] = 0;
                self.v[vx as usize] = x ^ y;
            },
            Bcd(vx) => {
                let x = self.v[vx as usize];
                let ones = x % 10;
                let tens = (x / 10) % 10;
                let hundreds = (x / 100) % 10;
                self.memory.write_u8(self.i, hundreds);
                self.memory.write_u8(self.i + 1, tens);
                self.memory.write_u8(self.i + 2, ones);
            },
            Draw(vx, vy, rows) => {
                let x = self.v[vx as usize];
                let y = self.v[vy as usize];
                let f_flag = self.screen.draw(&self.memory, self.i, x, y, rows);
                self.v[0xF] = f_flag as u8;
            },
            SkipIfEq(vx, val) => {
                if self.v[vx as usize] == val {
                    self.pc += 2;
                }
            },
            SkipIfNe(vx, val) => {
                if self.v[vx as usize] != val {
                    self.pc += 2;
                }
            },
            SkipIfVxVyEq(vx, vy) => {
                if self.v[vx as usize] == self.v[vy as usize] {
                    self.pc += 2;
                }
            },
            SkipIfVxVyNe(vx, vy) => {
                if self.v[vx as usize] != self.v[vy as usize] {
                    self.pc += 2;
                }
            },
            SkipIfPressed(vx) => {
                let x = self.v[vx as usize];
                if self.keys[x as usize] {
                    self.pc += 2;
                }
            },
            SkipIfNotPressed(vx) => {
                let x = self.v[vx as usize];
                if self.keys[x as usize] {
                    self.pc += 2;
                }
            },
            Jump(addr) => self.pc = addr,
            JumpV0(addr) => self.pc = addr + (self.v[0x0] as u16),
            Call(addr) => {
                self.sp += 2;
                self.memory.write_u16(self.sp, self.pc);
                self.pc = addr;
            },
            Return => {
                let addr = self.memory.read_u16(self.sp);
                self.sp -= 2;
                self.pc = addr;
            },
            Load(vx) => {
                assert!(vx < 0x10);
                let slice = &mut self.v[0..(vx + 1) as usize];
                for (n, x) in slice.iter_mut().enumerate() {
                    *x = self.memory.read_u8(self.i + n as u16);
                }
            },
            Store(vx) => {
                assert!(vx < 0x10);
                let slice = &self.v[0..(vx + 1) as usize];
                for (n, x) in slice.iter().enumerate() {
                    let pos = self.i + (n as u16);
                    self.memory.write_u8(pos, *x);
                }
            },
            WaitUntilPressed(vx) => {
                match self.keys.iter().position(|b| *b) {
                    Some(i) => self.v[vx as usize] = i as u8,
                    // keep looping
                    None => self.pc -= 2,
                }
            }
        }
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
        let inst = self.decode(opcode);
        self.execute(inst);
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }
}
