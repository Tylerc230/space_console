#![no_std]
#![no_main]

use panic_halt as _;
use firmware_rust as fr;
use fr::Program;

#[arduino_hal::entry]
fn main() -> ! {
    let test_program = fr::TestProgram {};
    test_program.init();
    loop {
    }
}
