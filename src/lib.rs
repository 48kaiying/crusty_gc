use std::ffi::{CStr};

#[no_mangle]
pub extern "C" fn rust_test() {
    println!("running rust function");
}

#[no_mangle]
pub extern "C" fn rust_test2(a: i32) -> i32 {
    println!("C app passed value {}", a);
    return a + 1; 
}

#[no_mangle]
pub extern "C" fn rust_string(cstr: *const i8) {
    let s = unsafe { CStr::from_ptr(cstr).to_string_lossy().into_owned() };
    println!("rust_string() is called, value passed = <{:?}>", s);
}
