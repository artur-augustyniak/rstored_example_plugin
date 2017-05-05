extern crate libc;

use libc::c_char;
use libc::puts;
use std::ffi::CString;

#[no_mangle]
pub extern fn ctor() {
    let s = CString::new("Loading Rust plugin!").unwrap();
    unsafe {
        let _ = puts(s.as_ptr());
    }
}

#[no_mangle]
pub extern fn dtor() {
    let s = CString::new("Unloading Rust plugin!").unwrap();
    unsafe {
        let _ = puts(s.as_ptr());
    }
}

#[link_section = ".ctors"]
pub static CTOR: extern fn() = ctor;

#[link_section = ".dtors"]
pub static DTOR: extern fn() = dtor;

static STR: &'static str = "Hello Rust plugin!";

#[no_mangle]
pub fn run_probe() -> *const c_char {
    let s = CString::new(STR).unwrap();
    return s.into_raw();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_run_probe() {
        unsafe {
            let expected = STR;
            let extern_str = CStr::from_ptr(run_probe()).to_str().unwrap();
            assert_eq!(expected, extern_str);
        }
    }
}
