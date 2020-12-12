#![no_std]
#![no_main]
use core::panic::PanicInfo;

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
