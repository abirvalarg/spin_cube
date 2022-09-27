use std::ffi::{c_uint, c_char};

#[repr(C)]
#[derive(Debug)]
pub struct TermSize {
    pub width: c_uint,
    pub height: c_uint,
}

impl TermSize {
    pub fn get() -> Self {
        unsafe {
            get_term_size()
        }
    }
}

pub fn put(ch: char) {
    unsafe {
        putchar(ch as i8);
    }
}

pub fn flush() {
    unsafe {
        flush_stdout();
    }
}

extern "C" {
    fn get_term_size() -> TermSize;
    fn putchar(c: c_char);
    fn flush_stdout();
}
