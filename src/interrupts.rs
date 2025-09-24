use lazy_static::lazy_static;

mod idt;
use crate::{interrupts::idt::InterruptDescriptorTable, printkln};

pub use idt::InterruptDescriptorTablePointer;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.set_handler(0, divide_by_zero_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "C" fn divide_by_zero_handler() -> ! {
    printkln!("EXCEPTION: DIVIDE BY ZERO");
    #[allow(clippy::empty_loop)]
    loop {}
}
