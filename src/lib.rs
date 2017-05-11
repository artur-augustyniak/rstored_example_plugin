extern crate libc;

use libc::c_char;
use libc::puts;
use std::ffi::CString;


#[allow(dead_code)]
extern "C"
fn ctor() {
    let s = CString::new("Loading Rust plugin!").unwrap();
    unsafe {
        let _ = puts(s.as_ptr());
    }
    println!("Rust plugin println! macro in ctor.");
}

#[allow(dead_code)]
extern "C"
fn dtor() {
    let s = CString::new("Unloading Rust plugin!").unwrap();
    unsafe {
        let _ = puts(s.as_ptr());
    }
    println!("Rust plugin println! macro in dtor.");
}


#[cfg(not(test))]
#[allow(dead_code)]
#[link_section = ".ctors"]
static CTOR: extern "C" fn() = ctor;

#[cfg(not(test))]
#[allow(dead_code)]
#[link_section = ".dtors"]
static DTOR: extern "C" fn() = dtor;

static STR: &'static str = "Hello Rust plugin!w";

#[no_mangle]
pub  extern "C" fn run_probe() -> *const c_char {

    println!("Rust plugin println! macro in run_probe.");
    let local_s = CString::new("Rust plugin clib::puts in run_probe.").unwrap();
    unsafe {
        let _ = puts(local_s.as_ptr());
    }
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
            let actual = CStr::from_ptr(run_probe()).to_str().unwrap();
            assert_eq!(expected, actual);
        }
    }
}
