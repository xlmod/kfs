
use core::fmt;
use core::ptr::Unique;

use crate::vga_buffer::{
    color,
    BUFFER_WIDTH,
    BUFFER_HEIGHT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: color::ColorCodeU8,
}

pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    color_code: color::ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {

    pub const fn new(
        column_position: usize,
        row_position: usize,
        color_code: color::ColorCode,
        buffer: Unique<Buffer>
    ) -> Self {
        Self {
            column_position,
            row_position,
            color_code,
            buffer,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;
                let color_code = self.color_code.get_color_code_u8();

                self.buffer().chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn set_color_code(&mut self, color_code: color::ColorCode) {
        self.color_code = color_code;
    }

    pub fn set_foreground(&mut self, fg: color::Color) {
        self.color_code.set_foreground(fg);
    }

    pub fn set_background(&mut self, bg: color::Color) {
        self.color_code.set_background(bg);
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() }
    }

    fn new_line(&mut self) {
        if self.row_position >= BUFFER_HEIGHT - 1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let buffer = self.buffer();
                    let character = buffer.chars[row][col];
                    buffer.chars[row - 1][col] = character;
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        } else {
            self.row_position += 1;
        }
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            let buffer = self.buffer();
            let screen_char = buffer.chars[row][col];
            buffer.chars[row][col] = ScreenChar {
                ascii_character: b' ',
                color_code: screen_char.color_code,
            };
        }
    }

}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}
