use firmware_rust::Program;
use firmware_rust::ProgramRunner;
use firmware_rust::PixelBuffer;
use smart_leds::{
    colors::*
};

struct ZeroProgram {
}

impl Program for ZeroProgram {
    fn init(&self) {

    }

    fn update(&self, buffer: &mut PixelBuffer) {
    }
}

struct SimpleProgram {
}
impl Program for SimpleProgram {
    fn init(&self) {

    }

    fn update(&self, buffer: &mut PixelBuffer) {
        buffer.fill(RED);
    }
}

#[test]
fn simple_program_test() {
    let mut buffer = PixelBuffer::new(BLACK);
    let z = ZeroProgram {};
    let s = SimpleProgram{};

    let mut r = ProgramRunner{current_program: &z};
    r.run_program(&s);
    r.update(&mut buffer);
    let expected = PixelBuffer::new(RED);
    assert_eq!(buffer, expected);
}


