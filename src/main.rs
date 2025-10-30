#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner::test_runner)]
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]

use kernel::printkln;
mod panic;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kernel::init();

    #[cfg(test)]
    test_main();

    printkln!("It did not crash");

    #[allow(clippy::empty_loop)]
    loop {}
}
