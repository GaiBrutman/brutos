use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

const NEWLINE: u8 = b'\n';
const NON_PRINTABLE: u8 = 0xFE;
const SPACE: u8 = b' ';

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

static VGA_MEM_ADDR: u32 = 0xb8000;

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
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column: usize,
    row: usize,

    pub buffer: &'static mut Buffer,
    pub color_code: ColorCode,
}

impl Writer {
    pub fn new(buffer: &'static mut Buffer, color_code: ColorCode) -> Self {
        Self {
            column: 0,
            row: BUFFER_HEIGHT - 1,
            buffer,
            color_code,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            NEWLINE => self.new_line(),
            byte => {
                self.buffer.chars[self.row][self.column].write(ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                });
                self.column += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | NEWLINE => byte,
                // not part of printable ASCII range
                _ => NON_PRINTABLE,
            });
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: SPACE,
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }

    fn write_fmt(mut self: &mut Self, args: fmt::Arguments<'_>) -> fmt::Result {
        fmt::write(&mut self, args)
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(
        unsafe { &mut *(VGA_MEM_ADDR as *mut Buffer) },
        ColorCode::new(Color::Yellow, Color::Black),
    ));
}
