#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustostutorial::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustostutorial::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustostutorial::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("Testing println as integ test");
}