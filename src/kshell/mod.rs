
use core::str::from_utf8;

use crate::{
    keyboard::{
        KEYBOARD,
        DecodedKey,
        KeyCode,
        KeyEvent,
        KeyState,
    },
    port::PortReadOnly,
    kprint,
    kprintln,
};

mod command;

const CMD_SIZE: usize = 1024;
const MAX_ARG: usize = 20;

struct Command {
    buffer: [u8; 1024],
    index: usize,
    modkey: bool,
}

impl Command {
    fn new() -> Self {
        Self { buffer: [0; 1024],index: 0, modkey: false}
    }

    #[allow(dead_code)]
    fn get(&self) -> &str {
        from_utf8(&self.buffer[0..self.index]).unwrap()
    }

    fn get_key(&mut self, scancode: u8) -> Option<DecodedKey> {
        // TODO Remove interupt during lock to avoid dead lock.
        let mut keyboard_lock = unsafe { KEYBOARD.lock() };
        if let Ok(Some(key_event)) = keyboard_lock.add_byte(scancode) {
            match key_event {
                KeyEvent {
                    code: KeyCode::AltLeft,
                    state: KeyState::Down,
                } => {
                    self.modkey = true;
                    None
                },
                KeyEvent {
                    code: KeyCode::AltLeft,
                    state: KeyState::Up,
                } => {
                    self.modkey = false;
                    None
                },
                ev => keyboard_lock.process_keyevent(ev),
            }
        } else { None }
    }

    fn read(&mut self) -> bool {
        loop {
            let mut keyboard_cmd = PortReadOnly::<u8>::new(0x64);
            let mut keyboard_data = PortReadOnly::<u8>::new(0x60);
            while unsafe { keyboard_cmd.read() } & 1 != 1 {
                core::hint::spin_loop();
            }
            let scancode = unsafe { keyboard_data.read() };
            if let Some(k) = self.get_key(scancode) {
                match k {
                    DecodedKey::Unicode(c) => {
                        match c {
                            '\x08' => {
                                if self.index != 0 {
                                    self.index -= 1;
                                    self.buffer[self.index] = b'\x00';
                                    kprint!("{}", c);
                                }
                            },
                            '\x0a' => {
                                kprint!("{}", c);
                                break true;
                            },
                            '\x09' => {},
                            _ => {
                                if self.index != CMD_SIZE {
                                    self.buffer[self.index] = c.encode_utf8(&mut [0; 4]).as_bytes()[0];
                                    self.index += 1;
                                    kprint!("{}",c);
                                }
                            },
                        }
                    }
                    DecodedKey::RawKey(r) => { if self.shortcut(r) { break false; } },
                }
            }
        }
    }

    fn shortcut(&self, key: KeyCode) -> bool {
        match key {
            KeyCode::ArrowRight => {kprintln!(); command::next_vt()},
            KeyCode::ArrowLeft => {kprintln!(); command::prev_vt()},
            _ => return false,
        }
        return true;
    }

}

pub fn kshell() {
    kprintln!("Welcome to Kfs-{}", crate::VERSION);
    kprintln!();
    loop {
        let mut cmd = Command::new();
        kprint!("kshell# ");
        if !cmd.read() { continue }
        let list_arg  = cmd.buffer[0..cmd.index].split(|num| *num == b' ');
        let mut nb_arg: usize = 0;
        let mut args: [&str; MAX_ARG] = [""; MAX_ARG];
        for (i, b) in list_arg.clone().enumerate() {
            if i < MAX_ARG && b.len() != 0 {
                args[nb_arg] = from_utf8(b).unwrap();
                nb_arg += 1;
            }
        }
        match args[0] {
            "exit" => { command::exit(); break},
            "shutdown" => command::shutdown(),
            "reboot" => command::reboot(),
            "clear" => command::clear_vt(),
            "next" => command::next_vt(),
            "prev" => command::prev_vt(),
            "help" => command::help(),
            "info" => command::info(),
            "read_serial" => command::read_serial(),
            "echo" => command::echo(&args[1..nb_arg]),
            _ => {},
        }
    }
}
