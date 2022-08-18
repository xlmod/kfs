
// features
#![feature(ptr_internals)]

#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod vga_buffer;
use vga_buffer::color::Color;

const VERSION: &str = "0.1.0";

#[no_mangle]
pub extern fn kmain() {
    clearscreen!();
    kprintln!("Welcome on Kfs");
    kprintln!("version: {}", VERSION);
    kprintln!();
    setbgcolor!(Color::White);
    setfgcolor!(Color::Red);
    kprintln!("End of Execution!");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {loop{}}
