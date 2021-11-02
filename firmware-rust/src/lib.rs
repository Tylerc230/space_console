#![no_std]
mod hal_timer;
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
