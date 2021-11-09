use firmware_rust::Program;
use firmware_rust::ProgramRunner;
use firmware_rust::PixelBuffer;
use firmware_rust::programs::simple_program::SimpleProgram;
use smart_leds::{
    colors::*
};

struct ZeroProgram {
}

impl Program for ZeroProgram {
    fn init(&self) {

    }

    fn update(&mut self, buffer: &mut PixelBuffer) {
    }
}


#[test]
fn simple_program_test() {
    let mut buffer = PixelBuffer::new();
    let mut z = ZeroProgram {};
    let mut s = SimpleProgram::new();

    let mut r = ProgramRunner::new(&mut z);
    r.run_program(&mut s);
    r.update(&mut buffer);
    let expected = PixelBuffer::new_fill_color(RED);
    assert_eq!(buffer, expected);
}


