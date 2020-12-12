#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod vga;
mod macros;

/// The kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
