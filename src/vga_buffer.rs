// repr will make rust represent the enum as a u8
// allow dead code allows us to specify variants that are unused

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8)) // each color takes up 4 bits
    }
}

// repr(C) guarantees field ordering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_positiion: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer, // shows that the lifetime of the buffer should always be available (VGA text buffer always there)
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_positiion > BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HEIGHT - 1;
                let column = self.column_positiion;
                let color_code = self.color_code;

                // ScreenChar is wrapped in volatile; means we use "write" instead of assign
                // Prevents compiler from optimizing away writes if we don't read
                self.buffer.chars[row][column].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                self.column_positiion += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            // omit first row because this moves offscreen
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_positiion = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // non-printable for VGA, print a block
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    // Mutex both grants threadsafe lock, but also makes this mutable
    // DON"T use mut static (this makes the static globally mutable, which is super unsafe)
    //
    // ref is needed to make sure that ownership of the WRITER static remains in the kernel
    // and doesn't get moved into other functions
    pub static ref WRITER: Mutex<Writer> = Mutex::new(
        Writer {
        column_positiion: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        // cannot use raw pointers in  statics
        // the compiler cannot evaluate them at compile time
        // need to use lazy-static in order to make this be evaluated at runtime
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
