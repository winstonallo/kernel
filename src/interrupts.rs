use lazy_static::lazy_static;

mod idt;
use crate::{address::VirtualAddress, interrupts::idt::InterruptDescriptorTable, printkln, registers::SegmentSelector};

pub use idt::InterruptDescriptorTablePointer;

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

pub struct InterruptStackFrame {
    pub instruction_pointer: VirtualAddress,
    pub code_segment: SegmentSelector,
    _reserved: [u8; 6],
    pub cpu_flags: usize, // wrong
    pub stack_pointer: VirtualAddress,
    pub stack_segment: SegmentSelector,
    _reserved2: [u8; 6],
}

impl InterruptStackFrame {
    // pub fn new(inst)
}

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

// extern "C" fn breakpoint_handler(stack_frame: InterruptStackFrame) {}
