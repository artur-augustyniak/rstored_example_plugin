use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub fn run_probe() -> *const c_char {
    let s = CString::new("Hello Rust dynlib plugin!").unwrap();
    return s.into_raw();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
