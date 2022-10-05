// features
#![feature(ptr_internals)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod keyboard;
pub mod kshell;
pub mod port;
pub mod serial;
pub mod spinlock;
pub mod vga_buffer;

use keyboard::KEYBOARD;

const VERSION: &str = "1.0.0";

#[allow(dead_code)]
extern "C" {
    fn stack_bottom();
    fn stack_top();
}

/// Initialisation of the kernel.
///
/// - clear the screen
/// - set the color to default
/// - init the serial module
fn kinit() {
    screen_clear!();
    screen_setcolor!(Default::default());
    unsafe {
        KEYBOARD.is_locked();
    } // Needed don't known why but whitout spinnlock is lock.
      // TODO Remove interupt during lock to avoid dead lock.
    unsafe { serial::SERIAL.lock().init() };
    kprintln!("42");
}

/// Entry point of the rust part.
#[no_mangle]
pub extern "C" fn kmain() {
    kinit();
    loop {
        kshell::kshell();
    }
}

/// Panic handler.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("{:?}", info);
    loop {}
}
