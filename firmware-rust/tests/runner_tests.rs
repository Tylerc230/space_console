use firmware_rust::Program;
use firmware_rust::program_runner::ProgramRunner;

struct SimpleProgram {
}
impl Program for SimpleProgram {
    fn init(&self) {

    }

    fn update(&self) {

    }
}

#[test]
fn simple_program_test() {
    let s = SimpleProgram{};
    let r = ProgramRunner{};
    r.run_program(&s);
}


#[test]
fn test_test() {
    assert_eq!(4, 2 + 2);
}
