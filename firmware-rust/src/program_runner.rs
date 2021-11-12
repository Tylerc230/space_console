use crate::program::Program;
use crate::pixel_buffer::PixelBuffer;
use crate::programs::ProgramKind;
pub struct ProgramRunner<'a> {
    current_program: &'a mut ProgramKind,//crashes when this is dyn
    pub pixel_buffer: PixelBuffer
}

impl<'a> ProgramRunner<'a> {
    //pub fn new(current_program: &'a mut SimpleProgram) -> ProgramRunner {
    pub fn new(current_program: &'a mut ProgramKind) -> ProgramRunner {
        ProgramRunner{current_program, pixel_buffer: PixelBuffer::new() }
        //ProgramRunner{current_program}
    }

    pub fn run_program(&mut self, program: &'a mut ProgramKind) {
        self.current_program = program;
    }

    pub fn update(&mut self) {
        match self.current_program {
            ProgramKind::Simple(simple_program) => simple_program.update(&mut self.pixel_buffer),
            ProgramKind::Zero(zero) => zero.update(&mut self.pixel_buffer)
        }
    }
}
