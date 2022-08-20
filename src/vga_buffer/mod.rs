
use core::fmt;

pub mod color;
mod cursor;
pub mod writer;
use self::writer::WRITER;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

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
    unsafe {
        WRITER.cursor_disable();
        WRITER.write_fmt(args).unwrap();
        WRITER.cursor_update();
        WRITER.cursor_enable();
    }
}

#[macro_export]
macro_rules! screen_setfgcolor {
    ($fg:expr) => (unsafe {
        crate::vga_buffer::writer::WRITER.set_foreground($fg)
    });
}

#[macro_export]
macro_rules! screen_setbgcolor {
    ($bg:expr) => (unsafe {
        crate::vga_buffer::writer::WRITER.set_background($bg)
    });
}

#[macro_export]
macro_rules! screen_setcolor {
    ($cc:expr) => (unsafe {
        crate::vga_buffer::writer::WRITER.set_color_code($cc)
    });
}

#[macro_export]
macro_rules! screen_clear {
    () => (unsafe {
        crate::vga_buffer::writer::WRITER.clear()
    });
}

#[macro_export]
macro_rules! screen_next {
    () => (unsafe {
        crate::vga_buffer::writer::WRITER.next_screen()
    });
}

#[macro_export]
macro_rules! screen_set {
    ($i:expr) => (unsafe {
        crate::vga_buffer::writer::WRITER.change_screen(i) 
    });
}
