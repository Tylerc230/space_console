// make `std` available when testing
#![cfg_attr(not(test), no_std)]
pub mod program_runner;
pub trait Program {
    fn init(&self);
    fn update(&self);
}

pub struct TestProgram {
}

impl TestProgram {
}

impl Program for TestProgram {
    fn init(&self) {
    }
    fn update(&self) {
    }
}

