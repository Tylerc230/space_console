use smart_leds::colors::*;
use super::super::Program;
use super::super::PixelBuffer;

pub struct SimpleProgram {
}
impl Program for SimpleProgram {
    fn init(&self) {

    }

    fn update(&self, buffer: &mut PixelBuffer) {
        buffer.fill(BLUE);
    }
}
