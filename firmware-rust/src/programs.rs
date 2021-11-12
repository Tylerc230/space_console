pub mod simple_program;
pub mod zero_program;
pub enum ProgramKind {
    Zero(zero_program::ZeroProgram),
    Simple(simple_program::SimpleProgram)
}
