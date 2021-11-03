// make `std` available when testing
#![cfg_attr(not(test), no_std)]
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

#[test]
fn test_test() {
    assert_eq!(4, 2 + 2);
}
