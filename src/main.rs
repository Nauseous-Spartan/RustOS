#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

// This function is called on Panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}


mod vga_buffer;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // entry point of the funtion where the linker will look
    // for start by default
    vga_buffer::print_something();
    
    loop{}
}
