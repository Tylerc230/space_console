use crate::program::Program;
use crate::pixel_buffer::PixelBuffer;
pub struct ProgramRunner<'a> {
    pub current_program: &'a dyn Program
}

impl<'a> ProgramRunner<'a> {
    pub fn run_program(&mut self, program: &'a dyn Program) {
        self.current_program = program;
    }
    pub fn update(&self, buffer: &mut PixelBuffer) {
        self.current_program.update(buffer);
    }
}
