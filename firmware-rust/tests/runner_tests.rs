use firmware_rust::ProgramRunner;
use firmware_rust::PixelBuffer;
use firmware_rust::programs::{
    simple_program::SimpleProgram,
    zero_program::ZeroProgram
};
use smart_leds::{
    colors::*
};

#[test]
fn simple_program_test() {
    let mut z = ZeroProgram::new();
    let mut s = SimpleProgram::new();

    let mut r = ProgramRunner::new(&mut z);
    r.run_program(&mut s);
    r.update();
    let expected = PixelBuffer::new_fill_color(RED);
    assert_eq!(r.pixel_buffer, expected);
}


