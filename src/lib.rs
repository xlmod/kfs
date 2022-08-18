
// features
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern fn kmain() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {loop{}}
