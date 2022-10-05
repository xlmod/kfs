use core::mem::transmute;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(fg: Color, bg: Color) -> Self {
        Self((bg as u8) << 4 | (fg as u8))
    }

    pub fn set_foreground(&mut self, fg: Color) {
        unsafe {
            let color = transmute::<ColorCode, u8>(self.clone());
            *self = transmute::<u8, ColorCode>((color & 0xf0) | ((fg as u8) & 0x0f));
        }
    }

    pub fn set_background(&mut self, bg: Color) {
        unsafe {
            let color = transmute::<ColorCode, u8>(self.clone());
            *self = transmute::<u8, ColorCode>(((bg as u8) << 4) | (color & 0x0f));
        }
    }
}

impl Default for ColorCode {
    fn default() -> Self {
        Self::new(Color::LightGray, Color::Black)
    }
}
