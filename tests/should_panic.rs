#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rustostutorial::{QemuExitCode, exit_qemu, serial_println, serial_print};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("OK");
    exit_qemu(QemuExitCode::Success);
    loop{}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("Test did not panic!");
    exit_qemu(QemuExitCode::Failed);
    loop{}
}

fn should_fail(){
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0,1);
}

