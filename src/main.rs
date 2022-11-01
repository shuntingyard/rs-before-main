#![no_std]
#![no_main]

#[no_mangle]
// `pub extern "C" fn main() -> isize {` and the syntax without "C" seem to be equivalent.
pub extern fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
