use crate::interrupts::InterruptDescriptorTablePointer;

/// # Safety
/// It is the caller's responsibility to ensure that `idt` points to a valid
/// IDT.
pub unsafe fn lidt(idt: &InterruptDescriptorTablePointer) {
    unsafe {
        core::arch::asm!("lidt [{}]", in(reg) idt, options(readonly, nostack, preserves_flags));
    }
}
