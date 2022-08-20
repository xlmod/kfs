
use crate::port::Port;

/// Shutdown qemu system
pub fn shutdown() {
    let mut port = Port::<u16>::new(0x604);
    unsafe {port.write(0x2000)};
}

/// Reboot system
pub fn reset() {
    let mut port = Port::<u8>::new(0x64);
    unsafe {port.write(0xfe)};
}

