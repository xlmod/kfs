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
        // TODO Remove interupt during lock to avoid dead lock.
        let mut writer_lock = WRITER.lock();
        writer_lock.cursor_disable();
        writer_lock.write_fmt(args).unwrap();
        writer_lock.cursor_update();
        writer_lock.cursor_enable();
    }
}

#[macro_export]
macro_rules! screen_setfgcolor {
    ($fg:expr) => {
        unsafe {
            // TODO Remove interupt during lock to avoid dead lock.
            crate::vga_buffer::writer::WRITER.lock().set_foreground($fg)
        }
    };
}

#[macro_export]
macro_rules! screen_setbgcolor {
    ($bg:expr) => {
        unsafe {
            // TODO Remove interupt during lock to avoid dead lock.
            crate::vga_buffer::writer::WRITER.lock().set_background($bg)
        }
    };
}

#[macro_export]
macro_rules! screen_setcolor {
    ($cc:expr) => {
        unsafe {
            // TODO Remove interupt during lock to avoid dead lock.
            crate::vga_buffer::writer::WRITER.lock().set_color_code($cc)
        }
    };
}

#[macro_export]
macro_rules! screen_clear {
    () => {
        unsafe {
            // TODO Remove interupt during lock to avoid dead lock.
            crate::vga_buffer::writer::WRITER.lock().clear()
        }
    };
}

#[macro_export]
macro_rules! screen_next {
    () => {
        unsafe {
            // TODO Remove interupt during lock to avoid dead lock.
            crate::vga_buffer::writer::WRITER.lock().next_screen()
        }
    };
}

#[macro_export]
macro_rules! screen_prev {
    () => {
        unsafe {
            // TODO Remove interupt during lock to avoid dead lock.
            crate::vga_buffer::writer::WRITER.lock().prev_screen()
        }
    };
}

#[macro_export]
macro_rules! screen_set {
    ($i:expr) => {
        unsafe {
            // TODO Remove interupt during lock to avoid dead lock.
            crate::vga_buffer::writer::WRITER.lock().change_screen(i)
        }
    };
}
