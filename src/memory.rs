const RAM_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Memory([u8; RAM_SIZE]);

impl Memory {
    pub fn new() -> Memory {
        Memory([0; RAM_SIZE])
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn load(&mut self, src: &[u8], pos: u16) {
        let range = (pos as usize)..pos as usize + src.len();
        self.0[range].copy_from_slice(src);
    }

    pub fn read_u16(&self, pos: u16) -> u16 {
        let b1 = self.0[pos as usize] as u16;
        let b2 = self.0[(pos + 1) as usize] as u16;
        (b1 << 8) | b2
    }

    pub fn read_u8(&self, pos: u16) -> u8 {
        self.0[pos as usize]
    }

    pub fn write_u16(&mut self, pos: u16, val: u16) {
        let b1 = (val >> 8) as u8;
        let b2 = val as u8;
        self.0[pos as usize] = b1;
        self.0[(pos + 1) as usize] = b2;
    }

    pub fn write_u8(&mut self, pos: u16, val: u8) {
        self.0[pos as usize] = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let mem = Memory::new();
        assert_eq!(mem.size(), RAM_SIZE);
    }

    #[test]
    fn test_load() {
        let bytes = [0xFF; 64];
        let mut mem = Memory::new();
        mem.load(&bytes, 1024);
        assert_eq!(mem.read_u8(1024), 0xFF);
        assert_eq!(mem.read_u8(1024 + 63), 0xFF);
        assert_eq!(mem.read_u8(1024 + 64), 0x00);
    }

    #[test]
    fn test_read_write_u8() {
        let mut mem = Memory::new();
        mem.write_u8(0, 0xFF);
        assert_eq!(mem.read_u8(0), 0xFF);
        // next byte shouldn't be affected
        assert_eq!(mem.read_u8(1), 0x00);
    }

    #[test]
    fn test_read_write_u16() {
        let mut mem = Memory::new();
        mem.write_u16(0, 0xFFFF);
        assert_eq!(mem.read_u16(0), 0xFFFF);
        assert_eq!(mem.read_u16(1), 0xFF00);
        assert_eq!(mem.read_u16(2), 0x0000);
    }
}
