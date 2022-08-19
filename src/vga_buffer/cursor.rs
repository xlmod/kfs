
use crate::vga_buffer::color;
use crate::port;

/// Represent a cursor on a screen with a position (x, y) and a color code.
pub struct Cursor {
    x: usize,
    y: usize,
    color_code: color::ColorCode,
}

impl Cursor {
    /// Return a new cursor.
    pub const fn new(x: usize, y: usize, color_code: color::ColorCode) -> Self {
        Self {
            x,
            y,
            color_code,
        }
    }

    /// Enable the blinking cursor on the screen.
    pub fn enable(&self) {
        let mut port1 = port::PortWriteOnly::new(0x3D4);
        let mut port2 = port::Port::new(0x3D5);

        unsafe {
            port1.write(0x0a as u8);
            let cur_start = port2.read() & 0xc0;
            port2.write(cur_start as u8);
            port1.write(0x0b as u8);
            let cur_end = port2.read() & 0xe0;
            port2.write(cur_end as u8);
        }
    }

    /// Disable the blinking cursor on the screen.
    pub fn disable(&self) {
        let mut port1 = port::PortWriteOnly::new(0x3D4);
        let mut port2 = port::PortWriteOnly::new(0x3D5);

        unsafe {
            port1.write(0x0a as u8);
            port2.write(0x20 as u8);
        }
    }

    /// Get the position of the cursor.
    pub fn get_pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Set the position of the cursor.
    pub fn set_pos(&mut self, x: usize, y:usize) {
        self.x = x;
        self.y = y;
    }

    /// Return the color code in u8 version.
    pub fn get_color_code_u8(&self) -> color::ColorCodeU8 {
        self.get_color_code_u8()
    }

    /// Return the color code struct.
    pub fn get_color_code(&self) -> color::ColorCodeU8 {
        self.get_color_code_u8()
    }

    /// Set the color code struct.
    pub fn set_color_code(&mut self, color_code: color::ColorCode) {
        self.color_code = color_code;
    }

    /// Set the foreground color.
    pub fn set_fg_color(&mut self, fg: color::Color) {
        self.color_code.set_foreground(fg);
    }

    /// Set the background color.
    pub fn set_bg_color(&mut self, bg: color::Color) {
        self.color_code.set_background(bg);
    }

}
