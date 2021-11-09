use smart_leds::colors::*;
use super::super::Program;
use super::super::PixelBuffer;

pub struct SimpleProgram {
    step:usize,
}
impl SimpleProgram {
    pub fn new() -> SimpleProgram {
        SimpleProgram {step: 0}
    }
}
impl Program for SimpleProgram {
    fn init(&self) {

    }

    fn update(&mut self, buffer: &mut PixelBuffer) {
        let colors = [RED, BLUE];
        buffer.fill(colors[self.step]);
        self.step = (self.step + 1) % 2;
    }
}
