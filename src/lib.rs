#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{BootInfo, entry_point};

pub mod address;
pub mod gdt;
pub mod interrupts;
pub mod macros;
pub mod memory;
pub mod port;
pub mod qemu;
pub mod registers;
pub mod serial;
pub mod test_runner;
pub mod vga_buffer;

pub fn init() {
    interrupts::init_idt();
    gdt::init();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_runner::panic_handler(info)
}

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
#[unsafe(no_mangle)]
fn test_kernel_main(_: &'static BootInfo) -> ! {
    init();
    test_main();
    loop {}
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
