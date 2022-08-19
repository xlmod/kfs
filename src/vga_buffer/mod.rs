
use core::{ptr::Unique, fmt};

pub mod color;
mod writer;
pub mod cursor;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

pub static mut WRITER: writer::Writer = writer::Writer::new(
    0,
    0,
    color::ColorCode::new(color::Color::LightGray, color::Color::Black),
    unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
);

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe { WRITER.write_fmt(args).unwrap() };
}

#[macro_export]
macro_rules! setfgcolor {
    ($fg:expr) => (unsafe { crate::vga_buffer::WRITER.set_foreground($fg) });
}

#[macro_export]
macro_rules! setbgcolor {
    ($bg:expr) => (unsafe { crate::vga_buffer::WRITER.set_background($bg) });
}

#[macro_export]
macro_rules! clearscreen {
    () => (unsafe { crate::vga_buffer::WRITER.clear() });
}

