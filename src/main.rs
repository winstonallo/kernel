#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner::test_runner)]
#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]

use kernel::{interrupts, printkln};
mod panic;
use macros::make_answer;

fn divide_by_zero() {
    unsafe { core::arch::asm!("mov dx, 0; div dx") }
}

make_answer!();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    printkln!("Hello, World! The answer is {}", answer());

    kernel::init();

    #[cfg(test)]
    test_main();

    interrupts::init_idt();

    divide_by_zero();

    printkln!("It did not crash");
    #[allow(clippy::empty_loop)]
    loop {}
}
