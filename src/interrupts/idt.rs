use crate::{
    address::VirtualAddress,
    instructions::lidt,
    registers::{self, PrivilegeLevel, Register, SegmentSelector},
};

pub type HandlerFunc = extern "C" fn() -> !;

pub struct InterruptDescriptorTable {
    entries: [Entry; 16],
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(2))]
pub struct InterruptDescriptorTablePointer {
    pub limit: u16,
    pub base: VirtualAddress,
}

impl InterruptDescriptorTable {
    pub fn new() -> Self {
        Self {
            entries: [Entry::missing(); 16],
        }
    }

    pub fn set_handler(&mut self, entry: u8, handler: HandlerFunc) -> &mut EntryOptions {
        self.entries[entry as usize] = Entry::new(registers::CS::get(), handler);
        &mut self.entries[entry as usize].options
    }

    pub fn load(&'static self) {
        let ptr = InterruptDescriptorTablePointer {
            base: VirtualAddress::new(self as *const _ as usize),
            limit: (core::mem::size_of::<Self>() - 1) as u16,
        };

        unsafe {
            lidt(&ptr);
        };
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct Entry {
    pointer_low: u16,
    gdt_selector: SegmentSelector,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

impl Entry {
    fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
        let pointer = handler as usize;
        Entry {
            gdt_selector,
            pointer_low: pointer as u16,
            pointer_middle: (pointer >> 16) as u16,
            pointer_high: (pointer >> 32) as u32,
            options: EntryOptions::new(),
            reserved: 0,
        }
    }

    fn missing() -> Self {
        Entry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: EntryOptions::minimal(),
            reserved: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EntryOptions(u16);

#[allow(unused)]
impl EntryOptions {
    fn new() -> Self {
        *Self::minimal().set_present(true).disable_interrupts(true)
    }

    fn minimal() -> Self {
        Self(0b111 << 9)
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0 |= (present as u16) << 15;
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0 |= (!disable as u16) << 8;
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        self.0 = (self.0 & !(0b11 << 13)) | ((dpl & 0b11) << 13);
        self
    }

    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0 = ((self.0 >> 3) << 3) | index;
        self
    }
}
