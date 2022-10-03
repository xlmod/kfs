
use crate::{
    port::Port,
    screen_clear,
    screen_next,
    screen_prev,
    kprintln,
    screen_setfgcolor,
    screen_setbgcolor,
    vga_buffer::color::{Color, ColorCode},
    screen_setcolor,
    kprint,
    kdebug,
    kdebugln,
};

/// Shutdown qemu system.
pub fn shutdown() {
    let mut port = Port::<u16>::new(0x604);
    unsafe {port.write(0x2000)};
}

/// Reboot system.
pub fn reboot() {
    let mut port = Port::<u8>::new(0x64);
    unsafe {port.write(0xfe)};
}

/// Clear the current virtual terminal.
pub fn clear_vt() {
    screen_clear!();
}

/// Go to the next virtual terminal.
pub fn next_vt() {
    screen_next!();
}

/// Go to the previous virtual terminal.
pub fn prev_vt() {
    screen_prev!();
}

/// Print the list of commands.
pub fn help() {
    kprintln!("exit         - quit the shell");
    kprintln!("shutdown     - shutdown qemu");
    kprintln!("reboot       - reboot the system");
    kprintln!("clear        - clear the screen");
    kprintln!("next         - go to the next virtual terminal");
    kprintln!("prev         - go to the previous virtual terminal");
    kprintln!("info         - print information of the kernel");
    kprintln!("read_serial  - print all bytes in serial port");
    kprintln!("echo         - print on terminal all arguments")
}

/// Exit the shell.
pub fn exit() {
    screen_setfgcolor!(Color::Red);
    screen_setbgcolor!(Color::White);
    kprintln!("exit");
    screen_setcolor!(ColorCode::default())
}

/// Print info about the kernel.
pub fn info() {
    kprintln!("version: {}", crate::VERSION);
}

/// Print from serial port.
pub fn read_serial() {
    use core::str::from_utf8;
    unsafe {
        loop {
            let b = match crate::serial::SERIAL.read_byte() {
                Some(b) => b,
                None => break,
            };
            kprint!("{}", from_utf8(&[b]).unwrap());
        }
    }
    kprintln!();
}

/// Print all args.
pub fn echo(args: &[&str]) {
    kprintln!("nb argument: {}", args.len());
    for (i, s) in args.iter().enumerate() {
        kprintln!("{}: '{}'", i, s);
    }
}
