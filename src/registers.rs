pub trait Register<T> {
    fn set(value: T);
    fn get() -> T;
}

#[repr(u8)]
pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

#[repr(transparent)]
pub struct CS(u16);

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    pub const fn new(index: u16, rpl: PrivilegeLevel) -> Self {
        Self(index << 3 | (rpl as u16))
    }
}

impl Register<SegmentSelector> for CS {
    fn get() -> SegmentSelector {
        let segment: u16;
        unsafe {
            core::arch::asm!("mov {0:x}, cs", out(reg) segment, options(nomem, nostack, preserves_flags));
        }
        SegmentSelector(segment)
    }

    fn set(value: SegmentSelector) {
        unsafe {
            core::arch::asm!(
                "push {value}",
                "lea {tmp}, [55f + rip]",
                "push {tmp}",
                "retfq",
                "55:",
                value = in(reg) u64::from(value.0),
                tmp = lateout(reg) _,
                options(preserves_flags)
            );
        }
    }
}
