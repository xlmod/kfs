

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
pub struct ColorCodeU8(u8);

impl ColorCodeU8 {
    pub const fn new(fg: Color, bg: Color) -> Self {
        Self ((bg as u8) << 4 | (fg as u8))
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode {
    fg: Color,
    bg: Color,
    color_code_u8: ColorCodeU8,
}

impl ColorCode {
    pub const fn new(fg: Color, bg: Color) -> Self {
        Self {
            fg,
            bg,
            color_code_u8: ColorCodeU8::new(fg, bg),
        }
    }

    pub fn set_foreground(&mut self, fg: Color) {
        self.fg = fg;
        self.color_code_u8 = ColorCodeU8::new(self.fg, self.bg);
    }

    pub fn set_background(&mut self, bg: Color) {
        self.bg = bg;
        self.color_code_u8 = ColorCodeU8::new(self.fg, self.bg);
    }

    pub fn get_color_code_u8(&mut self) -> ColorCodeU8 {
        self.color_code_u8
    }


}
