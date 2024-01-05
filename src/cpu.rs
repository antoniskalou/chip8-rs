use crate::memory::Memory;

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

    memory: Memory,
    /// screen buffer
    screen: [bool; SCREEN_SIZE],
}

#[derive(Debug)]
enum Instruction {
    Clear,
    Return,
    Set(u8, u8),
    SetIndex(u16),
}

impl CPU {
    pub fn new(memory: Memory) -> Self {
        Self {
            v: [0; NUM_REGS],
            i: 0,
            pc: 0,
            sp: 0xFA0,
            dt: 0,
            st: 0,
            keys: [false; NUM_KEYS],
            memory,
            screen: [false; SCREEN_SIZE],
        }
    }

    pub fn press_key(&mut self, key: usize, pressed: bool) {
        self.keys[key] = pressed;
    }

    pub fn screen_buffer(&self) -> &[bool] {
        &self.screen
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
        match u16_to_nibbles(opcode) {
            _ => unimplemented!("Unimplemented opcode: {}", opcode),
        }
    }

    fn execute(&mut self, inst: Instruction) {}

    fn tick(&mut self) {
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
