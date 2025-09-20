use core::panic::PanicInfo;

use crate::{serial_print, serial_println};

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn panic_handler(info: &PanicInfo) -> ! {
    use crate::qemu::{self, exit};

    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    exit(qemu::ExitCode::Failure);
    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::qemu;

    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit(qemu::ExitCode::Success);
}
