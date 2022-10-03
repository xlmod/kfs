
use core::ptr::Unique;
use core::fmt;

use super::{
    BUFFER_WIDTH,
    BUFFER_HEIGHT,
    color::{Color, ColorCode},
    cursor::Cursor,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    const fn new(ascii_character: u8, color_code: ColorCode) -> Self {
        Self { ascii_character, color_code }
    }

    #[allow(dead_code)]
    fn blank() -> Self {
        Self { ascii_character: b' ', color_code: ColorCode::default() }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    const fn new(ascii_character: u8, color_code: ColorCode) -> Self {
        let screen_char = ScreenChar::new(ascii_character, color_code);
        Self { chars: [[screen_char; BUFFER_WIDTH]; BUFFER_HEIGHT] }
    }

    fn set(&mut self, x: usize, y: usize, screen_char: ScreenChar) {
        self.chars[y][x] = screen_char;
    }

    fn set_char(&mut self, x: usize, y: usize, ascii_character: u8) {
        self.chars[y][x].ascii_character = ascii_character;
    }

    #[allow(dead_code)]
    fn set_color(&mut self, x: usize, y: usize, color_code: ColorCode) {
        self.chars[y][x].color_code = color_code;
    }

    fn get(&self, x: usize, y: usize) -> ScreenChar {
        self.chars[y][x]
    }

    #[allow(dead_code)]
    fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.chars[row][col] = ScreenChar::blank();
            }
        }
    }

}

impl Default for Buffer {
    fn default() -> Self {
        Self::new(b' ', ColorCode::default())
    }
}


#[derive(Debug, Clone, Copy)]
struct Vt {
    buffer: Buffer,
    cursor: Cursor,
    color_code: ColorCode,
}

impl Vt {
    const fn new() -> Self {
        Self {
            buffer: Buffer::new(
                        b' ',
                        ColorCode::new(Color::LightGray, Color::Black)
            ),
            cursor: Cursor::new(0, 0),
            color_code: ColorCode::new(Color::LightGray, Color::Black),
        }
    }

    #[allow(dead_code)]
    fn get_color_code(&self) -> ColorCode {
        self.color_code
    }

}

pub static mut WRITER: Writer = Writer {
    vt_index: 0,
    vt: [Vt::new(); VT_NUMBER],
    buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _)},
};

pub const VT_NUMBER: usize = 2;

pub struct Writer {
    vt_index: usize,
    vt: [Vt; VT_NUMBER],
    buffer: Unique<Buffer>,
}

impl Writer {

    pub const fn new(buffer: Unique<Buffer>) -> Self {
        Self {
            vt_index: 0,
            buffer,
            vt: [Vt::new(); VT_NUMBER],
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.as_mut() }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\x08' => self.remove_char(),
            byte => {
                let pos = self.vt[self.vt_index].cursor.get_pos();
                let screen_char =
                    ScreenChar::new(byte, self.vt[self.vt_index].color_code);
                let buffer = self.buffer();
                buffer.set(pos.0, pos.1, screen_char);
                self.vt[self.vt_index].buffer.set(pos.0, pos.1, screen_char);
                if pos.0 >= BUFFER_WIDTH - 1 {
                    self.new_line();
                } else {
                    self.vt[self.vt_index].cursor.inc();
                }
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn set_color_code(&mut self, color_code: ColorCode) {
        self.vt[self.vt_index].color_code = color_code;
    }

    pub fn set_foreground(&mut self, fg: Color) {
        self.vt[self.vt_index].color_code.set_foreground(fg);
    }

    pub fn set_background(&mut self, bg: Color) {
        self.vt[self.vt_index].color_code.set_background(bg);
    }

    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
        self.vt[self.vt_index].cursor = Cursor::default();
    }

    fn update_buffer(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let screen_char = self.vt[self.vt_index].buffer.get(col, row);
                self.buffer().set(col, row, screen_char);
            }
        }
    }

    pub fn change_screen(&mut self, index: usize) {
        if index >= VT_NUMBER { () }
        self.vt[self.vt_index].cursor.disable();
        self.vt_index = index;
        self.update_buffer();
        self.vt[self.vt_index].cursor.update();
        self.vt[self.vt_index].cursor.enable();
    }

    pub fn next_screen(&mut self) {
        let index: usize;
        if self.vt_index == VT_NUMBER - 1 {
            index = 0;
        } else {
            index = self.vt_index + 1;
        }
        self.change_screen(index);
    }

    pub fn prev_screen(&mut self) {
        let index: usize;
        if self.vt_index == 0 {
            index = VT_NUMBER - 1;
        } else {
            index = self.vt_index - 1;
        }
        self.change_screen(index);
    }

    pub fn cursor_enable(&self) {
        self.vt[self.vt_index].cursor.enable();
    }

    pub fn cursor_disable(&self) {
        self.vt[self.vt_index].cursor.disable();
    }

    pub fn cursor_update(&self) {
        self.vt[self.vt_index].cursor.update();
    }

    fn remove_char(&mut self) {
        self.vt[self.vt_index].cursor.dec();
        self.write_byte(b' ');
        self.vt[self.vt_index].cursor.dec();
    }

    fn new_line(&mut self) {
        if self.vt[self.vt_index].cursor.get_pos().1 >= BUFFER_HEIGHT - 1 {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let buffer = self.buffer();
                    buffer.set(col, row - 1, buffer.get(col, row));
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
        }
        self.vt[self.vt_index].cursor.next_line()
    }

    fn clear_row(&mut self, row: usize) {
        for col in 0..BUFFER_WIDTH {
            self.buffer().set_char(col, row, b' ');
            self.vt[self.vt_index].buffer.set_char(col, row, b' ');
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
