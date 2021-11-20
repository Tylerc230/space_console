use super::super::Program;
use super::super::PixelBuffer;
use super::ProgramKind;
pub struct ZeroProgram {
}

impl ZeroProgram {
    pub fn new() -> ProgramKind {
        let prog = ZeroProgram{};
        ProgramKind::Zero(prog)
    }
}

impl Program for ZeroProgram {
    fn init(&self) {

    }

    fn update(&mut self, _buffer: &mut PixelBuffer) {
    }
}


