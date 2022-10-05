use core::fmt;

use crate::{
    port::{Port, PortWriteOnly},
    spinlock::Spinlock,
};

pub static mut SERIAL: Spinlock<Serial> = Spinlock::new(Serial::new(0x3f8));

pub struct Serial {
    port0: Port<u8>,
    port1: PortWriteOnly<u8>,
    port2: PortWriteOnly<u8>,
    port3: PortWriteOnly<u8>,
    port4: PortWriteOnly<u8>,
    port5: Port<u8>,
    initialized: bool,
}

impl Serial {
    pub const fn new(port: u16) -> Self {
        Self {
            port0: Port::new(port),
            port1: PortWriteOnly::new(port + 1),
            port2: PortWriteOnly::new(port + 2),
            port3: PortWriteOnly::new(port + 3),
            port4: PortWriteOnly::new(port + 4),
            port5: Port::new(port + 5),
            initialized: false,
        }
    }

    pub fn init(&mut self) -> bool {
        unsafe {
            self.port1.write(0x00);
            self.port3.write(0x80);
            self.port0.write(0x03);
            self.port1.write(0x00);
            self.port3.write(0x03);
            self.port2.write(0xc7);
            self.port4.write(0x0b);
            self.port4.write(0x1e);
            self.port0.write(0xae);
            if self.port0.read() != 0xae {
                return false;
            }
            self.port4.write(0x0f);
            self.initialized = true;
            true
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        unsafe {
            while self.port5.read() & 0x20 == 0 {
                continue;
            }
            self.port0.write(byte);
        }
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        unsafe {
            while self.port5.read() & 0x1 == 0 {
                return None;
            }
            Some(self.port0.read())
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! kdebug {
    ($($arg:tt)*) => ($crate::serial::_debug(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kdebugln {
    () => ($crate::kdebug!("\n"));
    ($($arg:tt)*) => (crate::kdebug!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _debug(args: fmt::Arguments) {
    use core::fmt::Write;
    // TODO Remove interupt during lock to avoid dead lock.
    unsafe {
        SERIAL.lock().write_fmt(args).unwrap();
    }
}
