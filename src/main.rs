use std::ffi::{c_char, c_int};

extern "C" {
    fn main_c(argc: c_int, argv: *const *const c_char) -> c_int;
}

fn main() {
    unsafe { main_c(1, std::ptr::null()) };
}
