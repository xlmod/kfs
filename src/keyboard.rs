

use super::port::Port;

use core::hint::spin_loop;
use core::mem::transmute;

static KEY_TRANSLATION: [char; 58] = [
    '\0',      '\x1b',    '1',       '2',
    '3',       '4',       '5',       '6',
    '7',       '8',       '9',       '0',
    '-',       '=',       '\x08',    '\x09',
    'q',       'w',       'e',       'r',
    't',       'y',       'u',       'i',
    'o',       'p',       '[',       ']',
    '\x0a',    '\0',      'a',       's',
    'd',       'f',       'g',       'h',
    'j',       'k',       'l',       ';',
    '\'',      '`',       '\0',      '\\',
    'z',       'x',       'c',       'v',
    'b',       'n',       'm',       ',',
    '.',       '/',       '\0',      '\0',
    '\0',      ' ',
];

static KEY_CAP_TRANSLATION: [char; 58] = [
    '\0',      '\x1b',    '!',       '@',
    '#',       '$',       '%',       '^',
    '&',       '*',       '(',       ')',
    '_',       '+',       '\x08',    '\x09',
    'Q',       'W',       'E',       'R',
    'T',       'Y',       'U',       'I',
    'O',       'P',       '{',       '}',
    '\x0a',    '\0',      'A',       'S',
    'D',       'F',       'G',       'H',
    'J',       'K',       'L',       ':',
    '"',       '~',       '\0',      '|',
    'Z',       'X',       'C',       'V',
    'B',       'N',       'M',       '<',
    '>',       '?',       '\0',      '\0',
    '\0',      ' ',
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
    pub ascii_character: char,
    /// Global key modifiers.
    pub modifier: KeyModifier,
}

impl Key {

    /// Return a Key struct with the curretn key pressed.
    pub fn get_key() -> Self {
        let ascii_character = loop {
            let scancode = Key::get_scancode();
            let key_translation: &[char; 58];
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
                    0x3A => KEYMODIFIER.set_modifier(Modifier::CapsLock),
                    0xE0 => {match Key::get_scancode() {
                        0x1D => KEYMODIFIER.set_modifier(Modifier::LeftCtrl),
                        0x2A => KEYMODIFIER.set_modifier(Modifier::LeftShift),
                        0x36 => KEYMODIFIER.set_modifier(Modifier::RightShift),
                        0x38 => KEYMODIFIER.set_modifier(Modifier::LeftAlt),
                        0x3A => KEYMODIFIER.set_modifier(Modifier::CapsLock),
                        _ => continue,
                    }},
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
