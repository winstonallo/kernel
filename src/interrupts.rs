use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
mod idt;

pub use idt::InterruptDescriptorTablePointer;

use crate::printkln;

#[allow(unused)]
pub struct CPUFlags(usize);

impl CPUFlags {
    pub fn new() -> Self {
        Self(0)
    }
}

impl Default for CPUFlags {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    printkln!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
