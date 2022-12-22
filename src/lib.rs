use std::ffi::CStr;
mod allocator;

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

#[no_mangle]
pub extern "C" fn rgc_cleanup() {
    allocator::alloc_clean();
}

#[no_mangle]
pub extern "C" fn rgc_garbage_collect(
    etext: *const u8,
    end: *const u8,
    stack_top: *const u8,
    stack_bottom: *const u8,
) {
    allocator::garbage_collect(etext, end, stack_top, stack_bottom);
}
