
// features
#![feature(ptr_internals)]

#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod vga_buffer;
pub mod keyboard;
pub mod port;
pub mod kshell;
pub mod serial;

const VERSION: &str = "0.2.0";

fn kinit() {
    screen_clear!();
    screen_setcolor!(Default::default());
    unsafe { serial::SERIAL.init() };
    kdebugln!("[OK] Seriel port initialized!")
}

#[no_mangle]
pub extern fn kmain() {
    kinit();
    loop {
        kshell::kshell()
    };
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("{:?}", info);
    loop{}
}
