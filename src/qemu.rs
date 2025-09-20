#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub fn exit(code: ExitCode) {
    let port = crate::port::Port::new(0xf4);
    unsafe {
        port.write(code as u32);
    }
}
