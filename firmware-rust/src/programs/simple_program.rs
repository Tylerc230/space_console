use smart_leds::colors::*;
use super::super::Program;
use super::super::PixelBuffer;
use super::ProgramKind;

pub struct SimpleProgram {
    step:usize,
}
impl SimpleProgram {
    pub fn new() -> ProgramKind {
        let program = SimpleProgram {step: 0};
        ProgramKind::Simple(program)
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
