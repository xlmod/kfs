

use super::port::Port;

use core::hint::spin_loop;
use core::mem::transmute;

static KEY_TRANSLATION: [u8; 58] = [
    b'\0',      b'\x1b',    b'1',       b'2',
    b'3',       b'4',       b'5',       b'6',
    b'7',       b'8',       b'9',       b'0',
    b'-',       b'=',       b'\x08',    b'\x09',
    b'q',       b'w',       b'e',       b'r',
    b't',       b'y',       b'u',       b'i',
    b'o',       b'p',       b'[',       b']',
    b'\x0a',    b'\0',      b'a',       b's',
    b'd',       b'f',       b'g',       b'h',
    b'j',       b'k',       b'l',       b';',
    b'\'',      b'`',       b'\0',      b'\\',
    b'z',       b'x',       b'c',       b'v',
    b'b',       b'n',       b'm',       b',',
    b'.',       b'/',       b'\0',      b'\0',
    b'\0',      b' ',
];

static KEY_CAP_TRANSLATION: [u8; 58] = [
    b'\0',      b'\x1b',    b'!',       b'@',
    b'#',       b'$',       b'%',       b'^',
    b'&',       b'*',       b'(',       b')',
    b'_',       b'+',       b'\x08',    b'\x09',
    b'Q',       b'W',       b'E',       b'R',
    b'T',       b'Y',       b'U',       b'I',
    b'O',       b'P',       b'{',       b'}',
    b'\x0a',    b'\0',      b'A',       b'S',
    b'D',       b'F',       b'G',       b'H',
    b'J',       b'K',       b'L',       b':',
    b'"',       b'~',       b'\0',      b'|',
    b'Z',       b'X',       b'C',       b'V',
    b'B',       b'N',       b'M',       b'<',
    b'>',       b'?',       b'\0',      b'\0',
    b'\0',      b' ',
];

/// Represent a modifier key.
///
/// Can be bitwise together with KeyModifier struct.
#[repr(u8)]
pub enum Modifier {
    LeftCtrl = 1,
    LeftShift = (1 << 1),
    RightShift = (1 << 2),
    LeftAlt = (1 << 3),
    CapsLock = (1 << 4),
    RightCtrl = (1 << 5),
    RightAlt = (1 << 6),
}

static mut KEYMODIFIER: KeyModifier = KeyModifier::new();

/// Can store different Modifier to send along ascii_character in Key struct.
#[derive(Debug, Clone, Copy)]
pub struct KeyModifier(u8);

impl KeyModifier {
    /// Return a KeyModifier set to 0.
    const fn new () -> Self {
        Self(0)
    }

    /// Set the modifier.
    fn set_modifier(&mut self, modifier: Modifier) {
        unsafe {
            let modifiers = transmute::<Self, u8>(self.clone());
            *self = transmute::<u8, Self>(modifiers | (modifier as u8));
        }
    }

    /// Reset the modifier.
    #[allow(dead_code)]
    fn reset_modifier(&mut self, modifier: Modifier) {
        unsafe {
            let modifiers = transmute::<Self, u8>(self.clone());
            *self = transmute::<u8, Self>(modifiers & (!(modifier as u8)));
        }
    }

    /// Check if modifier is set.
    pub fn is_set(&self, modifier: Modifier) -> bool {
        unsafe {
            let modifiers = transmute::<Self, u8>(self.clone());
            (modifiers & (modifier as u8)) != 0
        }
    }

}

/// Used to return a ascii_character and current global KeyModifier.
pub struct Key {
    /// Ascii value of a character.
    pub ascii_character: u8,
    /// Global key modifiers.
    pub modifier: KeyModifier,
}

impl Key {

    /// Return a Key struct with the curretn key pressed.
    pub fn get_key() -> Self {
        let ascii_character = loop {
            let scancode = Key::get_scancode();
            let key_translation: &[u8; 58];
            unsafe {
                if (KEYMODIFIER.is_set(Modifier::LeftShift) ||
                        KEYMODIFIER.is_set(Modifier::RightShift)) &&
                        !KEYMODIFIER.is_set(Modifier::CapsLock) {
                    key_translation = &KEY_CAP_TRANSLATION;
                } else if (KEYMODIFIER.is_set(Modifier::LeftShift) ||
                        KEYMODIFIER.is_set(Modifier::RightShift)) &&
                        KEYMODIFIER.is_set(Modifier::CapsLock) {
                    key_translation = &KEY_TRANSLATION;
                } else if (!KEYMODIFIER.is_set(Modifier::LeftShift) &&
                        !KEYMODIFIER.is_set(Modifier::RightShift)) &&
                        KEYMODIFIER.is_set(Modifier::CapsLock) {
                    key_translation = &KEY_CAP_TRANSLATION;
                } else {
                    key_translation = &KEY_TRANSLATION;
                }

                match scancode {
                    0x01..=0x1C => break key_translation[scancode as usize],
                    0x1D => KEYMODIFIER.set_modifier(Modifier::LeftCtrl),
                    0x1E..=0x29 => break key_translation[scancode as usize],
                    0x2A => KEYMODIFIER.set_modifier(Modifier::LeftShift),
                    0x2B..=0x35 => break key_translation[scancode as usize],
                    0x36 => KEYMODIFIER.set_modifier(Modifier::RightShift),
                    0x38 => KEYMODIFIER.set_modifier(Modifier::LeftAlt),
                    0x39 => break key_translation[scancode as usize],
                    0x3A => {
                        if KEYMODIFIER.is_set(Modifier::CapsLock) {
                            KEYMODIFIER.reset_modifier(Modifier::CapsLock);
                        } else {
                            KEYMODIFIER.set_modifier(Modifier::CapsLock);
                        }
                    },
                    0xE0 => {match Key::get_scancode() {
                        0x1D => KEYMODIFIER.set_modifier(Modifier::RightCtrl),
                        0x38 => KEYMODIFIER.set_modifier(Modifier::RightAlt),
                        0x9D => KEYMODIFIER.reset_modifier(Modifier::RightCtrl),
                        0xB8 => KEYMODIFIER.reset_modifier(Modifier::RightAlt),
                        _ => continue,
                    }},
                    0x9D => KEYMODIFIER.reset_modifier(Modifier::LeftCtrl),
                    0xAA => KEYMODIFIER.reset_modifier(Modifier::LeftShift),
                    0xB6 => KEYMODIFIER.reset_modifier(Modifier::RightShift),
                    0xB8 => KEYMODIFIER.reset_modifier(Modifier::LeftAlt),
                    _ => continue,
                };
            }
        };
        Self {ascii_character, modifier: unsafe { KEYMODIFIER }}
    }

    /// Return the scancode from keyboard ports.
    fn get_scancode() -> u8 {
        let mut keyboard_cmd = Port::<u8>::new(0x64);
        let mut keyboard_data = Port::<u8>::new(0x60);
        unsafe {
            while keyboard_cmd.read() & 1 != 1 {
                spin_loop()
            }
            keyboard_data.read()
        }
    }

}
