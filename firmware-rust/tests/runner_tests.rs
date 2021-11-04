use firmware_rust::Program;
use firmware_rust::ProgramRunner;
use smart_leds::{
    RGB8, 
    colors::*
};

struct ZeroProgram {
}

impl Program for ZeroProgram {
    fn init(&self) {

    }

    fn update(&self, buffer: &mut [RGB8]) {
    }
}

struct SimpleProgram {
}
impl Program for SimpleProgram {
    fn init(&self) {

    }

    fn update(&self, buffer: &mut [RGB8]) {
        for led in buffer {
            *led = RED;
        }
    }
}

#[test]
fn simple_program_test() {
    let mut buffer = [BLACK; 2];
    let z = ZeroProgram {};
    let s = SimpleProgram{};

    let mut r = ProgramRunner{current_program: &z};
    r.run_program(&s);
    r.update(&mut buffer);
    assert_eq!(buffer, [RED; 2]);
}


