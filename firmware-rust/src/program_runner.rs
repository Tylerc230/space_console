use crate::program::Program;
use crate::pixel_buffer::PixelBuffer;
pub struct ProgramRunner<'a> {
    current_program: &'a mut dyn Program,
    //pub pixel_buffer: PixelBuffer
}

impl<'a> ProgramRunner<'a> {
    pub fn new(current_program: &mut dyn Program) -> ProgramRunner {
        //ProgramRunner{current_program, pixel_buffer: PixelBuffer::new() }
        ProgramRunner{current_program}
    }

    pub fn run_program(&mut self, program: &'a mut dyn Program) {
        self.current_program = program;
    }

    pub fn update(&mut self, buffer: &mut PixelBuffer) {
        self.current_program.update(buffer);
    }
}
