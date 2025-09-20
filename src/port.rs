use core::marker::PhantomData;

pub trait PortRead<T> {
    unsafe fn read_from_port(port: u16) -> T;
}

pub trait PortWrite<T> {
    unsafe fn write_to_port(port: u16, value: T);
}

impl PortRead<u8> for u8 {
    unsafe fn read_from_port(port: u16) -> u8 {
        let res: u8;
        unsafe {
            core::arch::asm!("in al, dx", out("al") res, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        res
    }
}

impl PortRead<u16> for u16 {
    unsafe fn read_from_port(port: u16) -> u16 {
        let res: u16;
        unsafe {
            core::arch::asm!("in ax, dx", out("ax") res, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        res
    }
}

impl PortRead<u32> for u32 {
    unsafe fn read_from_port(port: u16) -> u32 {
        let res: u32;
        unsafe {
            core::arch::asm!("in ax, dx", out("eax") res, in("dx") port, options(nomem, nostack, preserves_flags));
        }
        res
    }
}

impl PortWrite<u8> for u8 {
    unsafe fn write_to_port(port: u16, value: u8) {
        unsafe {
            core::arch::asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags))
        }
    }
}

impl PortWrite<u16> for u16 {
    unsafe fn write_to_port(port: u16, value: u16) {
        unsafe {
            core::arch::asm!("out dx, ax", in("dx") port, in("ax") value, options(nomem, nostack, preserves_flags))
        }
    }
}

impl PortWrite<u32> for u32 {
    unsafe fn write_to_port(port: u16, value: u32) {
        unsafe {
            core::arch::asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags))
        }
    }
}

pub struct Port<T> {
    port: u16,
    phantom: PhantomData<T>,
}

impl<T> Port<T> {
    pub fn new(port: u16) -> Port<T> {
        Port {
            port,
            phantom: PhantomData,
        }
    }
}

impl<T: PortRead<T>> Port<T> {
    pub unsafe fn read(&self) -> T {
        unsafe { T::read_from_port(self.port) }
    }
}

impl<T: PortWrite<T>> Port<T> {
    pub unsafe fn write(&self, value: T) {
        unsafe {
            T::write_to_port(self.port, value);
        }
    }
}
