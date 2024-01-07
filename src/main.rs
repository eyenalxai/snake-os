#![no_std]
#![no_main]

use core::panic::PanicInfo;
use snake_os::println;

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello World{}", "!");

    snake_os::init();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    snake_os::hlt_loop();
}
