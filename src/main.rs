#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use core::sync::atomic::Ordering;
use snake_os::interrupts::NEEDS_UPDATE;
use snake_os::println;
use snake_os::snake_game::SNAKE_GAME;

#[no_mangle]
pub extern "C" fn _start() {
    println!("Starting Snake Game!");

    snake_os::init();

    loop {
        if NEEDS_UPDATE.load(Ordering::SeqCst) {
            NEEDS_UPDATE.store(false, Ordering::SeqCst);

            let mut snake_game = SNAKE_GAME.lock();
            snake_game.update();
            snake_game.render();
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    snake_os::hlt_loop();
}
