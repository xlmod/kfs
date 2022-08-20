
use crate::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT};
use crate::port;

/// Represent a cursor on a screen with a position (x, y).
#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    x: usize,
    y: usize,
}

impl Cursor {
    /// Return a new cursor.
    pub const fn new(x: usize, y: usize) -> Self {
        Self {x, y}
    }

    /// Enable the blinking cursor on the screen.
    pub fn enable(&self) {
        let mut port1 = port::PortWriteOnly::<u8>::new(0x3D4);
        let mut port2 = port::Port::<u8>::new(0x3D5);

        unsafe {
            port1.write(0x0a);
            let cur_start = port2.read() & 0xc0;
            port2.write(cur_start | 14);
            port1.write(0x0b);
            let cur_end = port2.read() & 0xe0;
            port2.write(cur_end | 14);
        }
    }

    /// Disable the blinking cursor on the screen.
    pub fn disable(&self) {
        let mut port1 = port::PortWriteOnly::<u8>::new(0x3D4);
        let mut port2 = port::PortWriteOnly::<u8>::new(0x3D5);

        unsafe {
            port1.write(0x0a);
            port2.write(0x20);
        }
    }

    /// Update the position of the cursor on screen.
    pub fn update(&self) {
        let pos = self.y * BUFFER_WIDTH + self.x;
        let mut port1 = port::PortWriteOnly::<u8>::new(0x3D4);
        let mut port2 = port::PortWriteOnly::<u8>::new(0x3D5);

        unsafe {
            port1.write(0x0f);
            port2.write((pos & 0xff) as u8);
            port1.write(0x0e);
            port2.write(((pos >> 8) & 0xff) as u8);
        }
    }

    /// Get the position of the cursor.
    pub fn get_pos(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// Set the position of the cursor.
    #[allow(dead_code)]
    pub fn set_pos(&mut self, x: usize, y:usize) {
        self.x = x;
        self.y = y;
    }

    /// Increment the cursor by 1 character.
    ///
    /// If it's the end of line set x to 0 and y to next line or the same if
    /// the last line.
    pub fn inc(&mut self) {
        self.x += 1;
        if self.x >= BUFFER_WIDTH {
            self.x = 0;
            if self.y <= BUFFER_HEIGHT - 1 {
                self.y += 1;
            }
        }
    }

    /// Put cursor on next line
    ///
    /// Set x to 0 and add one to y if it's not the last line.
    pub fn next_line(&mut self) {
        self.x = 0;
        if self.y < BUFFER_HEIGHT - 1 {
            self.y += 1;
        }
    }

}

impl Default for Cursor {
    fn default() -> Self {
        let cursor = Self::new(0, 0);
        cursor.update();
        cursor
    }
}
