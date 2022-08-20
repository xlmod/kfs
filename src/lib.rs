
// features
#![feature(ptr_internals)]

#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod vga_buffer;
use vga_buffer::color::Color;

pub mod port;

const VERSION: &str = "0.1.0";

fn kinit() {
    screen_clear!();
    screen_setcolor!(Default::default());
    kprintln!("Welcome on Kfs");
    kprintln!("version: {}", VERSION);
}

#[no_mangle]
pub extern fn kmain() {
    kinit();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("{:?}", info);
    loop{}
}
