#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    blog_os::init();


    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // trigger a stack overflow
    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    blog_os::hlt_loop();
}

/// This function is called on panic.
// our existing panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
