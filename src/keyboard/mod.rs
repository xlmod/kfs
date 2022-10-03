

use core::marker::PhantomData;

mod scancode;
pub use self::scancode::ScancodeSet1;

mod layout;
pub use self::layout::Us104Key;

// STATIC

pub static mut KEYBOARD: Keyboard<Us104Key, ScancodeSet1> = Keyboard {
            decode_state: DecodeState::Start,
            handle_ctrl: HandleCtrl::Ignore,
            modifiers: Modifiers {
                lshift: false,
                rshift: false,
                lctrl: false,
                rctrl: false,
                numlock: false,
                capslock: false,
                alt_gr: false,
            },
            _layout: PhantomData,
            _set: PhantomData,
};


// STRUCT and ENUM

#[derive(Debug)]
pub struct Keyboard<T, S> {

    decode_state: DecodeState,
    handle_ctrl: HandleCtrl,
    modifiers: Modifiers,

    _layout: PhantomData<T>,
    _set: PhantomData<S>,
}


/// Indicates differente error condition
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Error {
    UnknownKeyCode,
}

/// Keycodes that can be generated
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum KeyCode {
    AltLeft,
    AltRight,
    ArrowDown,
    ArrowRight,
    ArrowLeft,
    ArrowUp,
    BackSlash,
    Backspace,
    BackTick,
    BracketSquareLeft,
    BracketSquareRight,
    CapsLock,
    Comma,
    CtrlLeft,
    CtrlRight,
    Delete,
    End,
    Enter,
    Equals,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Fullstop,
    Home,
    Insert,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadEnter,
    NumpadMinus,
    NumpadPeriod,
    NumpadPlus,
    NumpadSlash,
    NumpadStar,
    Minus,
    NumpadLock,
    PageUp,
    PageDown,
    Quote,
    ScrollLock,
    SemiColon,
    ShiftLeft,
    ShiftRight,
    Slash,
    Spacebar,
    Tab,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    // Fn key
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum KeyState {
    Up,
    Down,
}

// Key modifier
#[derive(Debug)]
pub struct Modifiers {
    pub lshift: bool,
    pub rshift: bool,
    pub lctrl: bool,
    pub rctrl: bool,
    pub numlock: bool,
    pub capslock: bool,
    pub alt_gr: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub state: KeyState,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DecodeState {
    Start,
    Extented,
}

// Return of decoded key
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum DecodedKey {
    RawKey(KeyCode),
    Unicode(char),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum HandleCtrl {
    MapLetterToUnicode,
    Ignore,
}

// TRAITS

pub trait ScancodeSet {

    fn advance_state(state: &mut DecodeState, code: u8) -> Result<Option<KeyEvent>, Error>;

    fn map_scancode(code: u8) -> Result<KeyCode, Error>;
    fn map_extended_scancode(code: u8) -> Result<KeyCode, Error>;

}

pub trait KeyboardLayout {

    fn map_keycode(
        keycode: KeyCode,
        modifier: &Modifiers,
        handle_ctrl: HandleCtrl,
    ) -> DecodedKey;

}

// IMPLEMENTATIONS

impl<T, S> Keyboard<T, S>
where
    T: KeyboardLayout,
    S: ScancodeSet,
{

    pub fn new(_layout: T, _set: S, handle_ctrl: HandleCtrl) -> Self {
        Self {
            decode_state: DecodeState::Start,
            handle_ctrl,
            modifiers: Modifiers {
                lshift: false,
                rshift: false,
                lctrl: false,
                rctrl: false,
                numlock: false,
                capslock: false,
                alt_gr: false,
            },
            _layout: PhantomData,
            _set: PhantomData,
        }
    }

    pub fn add_byte(&mut self, code: u8) -> Result<Option<KeyEvent>, Error> {
        let r = S::advance_state(&mut self.decode_state, code);
        r
    }

    pub fn process_keyevent(&mut self, ev: KeyEvent) -> Option<DecodedKey> {
        match ev {
            KeyEvent {
                code: KeyCode::ShiftLeft,
                state: KeyState::Down,
            } => {
                self.modifiers.lshift = true;
                None
            },
            KeyEvent {
                code: KeyCode::ShiftRight,
                state: KeyState::Down,
            } => {
                self.modifiers.rshift = true;
                None
            },
            KeyEvent {
                code: KeyCode::ShiftLeft,
                state: KeyState::Up,
            } => {
                self.modifiers.lshift = false;
                None
            },
            KeyEvent {
                code: KeyCode::ShiftRight,
                state: KeyState::Up,
            } => {
                self.modifiers.rshift = false;
                None
            },
            KeyEvent {
                code: KeyCode::CapsLock,
                state: KeyState::Down,
            } => {
                self.modifiers.capslock = !self.modifiers.capslock;
                None
            },
            KeyEvent {
                code: KeyCode::NumpadLock,
                state: KeyState::Down,
            } => {
                self.modifiers.numlock = !self.modifiers.numlock;
                None
            },
            KeyEvent {
                code: KeyCode::CtrlLeft,
                state: KeyState::Down,
            } => {
                self.modifiers.lctrl = true;
                None
            },
            KeyEvent {
                code: KeyCode::CtrlRight,
                state: KeyState::Down,
            } => {
                self.modifiers.rctrl = true;
                None
            },
            KeyEvent {
                code: KeyCode::CtrlLeft,
                state: KeyState::Up,
            } => {
                self.modifiers.lctrl = false;
                None
            },
            KeyEvent {
                code: KeyCode::CtrlRight,
                state: KeyState::Up,
            } => {
                self.modifiers.rctrl = false;
                None
            },
            KeyEvent {
                code: KeyCode::AltRight,
                state: KeyState::Down,
            } => {
                self.modifiers.alt_gr = true;
                None
            },
            KeyEvent {
                code: KeyCode::AltRight,
                state: KeyState::Up,
            } => {
                self.modifiers.alt_gr = false;
                None
            },
            KeyEvent {
                code: c,
                state: KeyState::Down
            } => Some(T::map_keycode(c, &self.modifiers, self.handle_ctrl)),
            _ => None,
        }
    }

}

impl KeyEvent {

    fn new(code: KeyCode, state: KeyState) -> Self {
        Self { code, state }
    }

}

impl Modifiers {

    pub fn is_ctrl(&self) -> bool {
        self.lctrl | self.rctrl
    }

    pub fn is_shifted(&self) -> bool {
        self.lshift | self.rshift
    }

    pub fn is_caps(&self) -> bool {
        (self.lshift | self.rshift) ^ self.capslock
    }

}
