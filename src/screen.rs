use crate::memory::Memory;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
// screen always has 64x32, it can be upscaled by the renderer
pub const SIZE: usize = WIDTH * HEIGHT;

#[derive(Debug)]
pub struct Screen([bool; SIZE]);

impl Screen {
    pub fn new() -> Self {
        Self([false; SIZE])
    }

    pub fn buffer(&self) -> &[bool; SIZE] {
        &self.0
    }

    pub fn clear(&mut self) {
        self.0.fill(false);
    }

    pub fn draw(&mut self, memory: &Memory, i: u16, x: u8, y: u8, rows: u8) -> bool {
        let mut f_flag = false;
        for y_line in 0..rows {
            let pixels = memory.read_u8(i + (y_line as u16));
            for x_line in 0..8 {
                if pixels & (0b1000_0000 >> x_line) != 0 {
                    let x = (x + x_line) as usize % WIDTH;
                    let y = (y + y_line) as usize % HEIGHT;
                    let idx = x + WIDTH * y;

                    f_flag |= self.0[idx];
                    self.0[idx] ^= true;
                }
            }
        }
        f_flag
    }
}
