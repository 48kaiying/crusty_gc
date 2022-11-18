use std::ffi::{CStr};
mod allocator;

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

#[no_mangle]
pub extern "C" fn rgc_init() {
    allocator::alloc_init();
}

#[no_mangle]
pub extern "C" fn rgc_malloc(size: isize) -> *mut u8 {
    if size <= 0 {
        return 0 as *mut u8; 
    }

    println!("rgc malloc requested {}", size);

    return allocator::malloc(size as usize);
}


#[no_mangle]
pub extern "C" fn rgc_free(ptr: *mut u8) {
    allocator::free(ptr);
}

