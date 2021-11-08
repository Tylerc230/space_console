use crate::program::Program;
use crate::pixel_buffer::PixelBuffer;
pub struct ProgramRunner<'a> {
    current_program: &'a dyn Program,
    pub pixel_buffer: PixelBuffer
}

impl<'a> ProgramRunner<'a> {
    pub fn new(current_program: &dyn Program) -> ProgramRunner {
        ProgramRunner{current_program, pixel_buffer: PixelBuffer::new() }
    }

    pub fn run_program(&mut self, program: &'a dyn Program) {
        self.current_program = program;
    }

    pub fn update(&mut self) {
        self.current_program.update(&mut self.pixel_buffer);
    }
}
