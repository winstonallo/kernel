use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

pub struct SerialPort {
    base: u16,
}

impl SerialPort {
    pub const unsafe fn new(base: u16) -> Self {
        Self { base }
    }

    fn port_data(&self) -> u16 {
        self.base
    }

    fn port_interrupt_enable(&self) -> u16 {
        self.base + 1
    }

    fn port_fifo_control(&self) -> u16 {
        self.base + 2
    }

    fn port_line_control(&self) -> u16 {
        self.base + 3
    }

    fn port_modem_control(&self) -> u16 {
        self.base + 4
    }

    fn port_line_status(&self) -> u16 {
        self.base + 5
    }

    fn line_status(&self) -> u8 {
        // 1 | 1 << 5
        use crate::port::Port;
        unsafe { Port::new(self.port_line_status()).read() }
    }

    pub fn send(&mut self, data: u8) {
        match data {
            8 | 0x7f => {
                self.send_raw(8);
                self.send_raw(b' ');
                self.send_raw(8);
            }
            0x0a => {
                self.send_raw(0x0d);
                self.send_raw(0x0a);
            }
            data => self.send_raw(data),
        }
    }

    pub fn send_raw(&mut self, data: u8) {
        crate::retry_until_ok!(self.try_send_raw(data))
    }

    pub fn try_send_raw(&mut self, data: u8) -> Result<(), ()> {
        if (self.line_status() >> 5) & 1 == 1 {
            unsafe {
                use crate::port::Port;
                Port::new(self.port_data()).write(data);
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn init(&mut self) {
        use crate::port::Port;
        unsafe {
            Port::new(self.port_interrupt_enable()).write(0x00 as u32);
            Port::new(self.port_line_control()).write(0x80 as u32);
            Port::new(self.port_data()).write(0x03 as u32);
            Port::new(self.port_interrupt_enable()).write(0x03 as u32);
            Port::new(self.port_line_control()).write(0x03 as u32);
            Port::new(self.port_fifo_control()).write(0xc7 as u32);
            Port::new(self.port_modem_control()).write(0x0b as u32);
            Port::new(self.port_interrupt_enable()).write(0x01 as u32);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3f8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1
        .lock()
        .write_fmt(args)
        .expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_print {
	($($arg:tt)*) => {
		$crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
