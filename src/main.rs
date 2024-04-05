#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustostutorial::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rustostutorial::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, again!");
    println!("A new number:\n{}", 42);

    #[cfg(test)]
    test_main();

    loop{}
}

#[test_case]
fn trivial_assertion(){
    assert_eq!(0, 0);
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rustostutorial::test_panic_handler(_info);
}