#![no_std]
#![no_main]

extern crate libc;

#[no_mangle]
// `pub extern "C" fn main() -> isize {` and the syntax without "C" seem to be equivalent.
pub extern fn main() -> isize {
    0
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
