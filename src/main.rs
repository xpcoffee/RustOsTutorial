#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            let line_offset: isize = 160 * 2;
            let char_offset_within_line: isize = i as isize * 2;
            let color_offset_within_line: isize = i as isize * 2 + 1;
            let char_offset = char_offset_within_line + line_offset;
            let color_offset = color_offset_within_line + line_offset;

            *vga_buffer.offset(i as isize * 2) - byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
