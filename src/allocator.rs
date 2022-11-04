use std::sync::Mutex;

// 1 KB block = 1024 / word
const SIZE_1KB : usize = 1024; 
// static mut BASE_PTR : *mut Block = 0 as *mut Block;

// Questions to ask
// 1. is this even a good idea to do in rust and how?

// Box makes the assumption that there is no alias raw pointer
// want rust to not assume box::into_raw and that will allow us to alias 

// lazy_static::lazy_static! {
//     static ref ALLOCATOR : Mutex<Allocator> = 
//         Mutex::new(Allocator { 
//             num_used: 0, 
//             num_free: 0, 
//             // base: Box::new(Block {size: SIZE_1KB, payload: vec![0; SIZE_1KB], next : None}), 
//             // base: Box::new(Block {size: SIZE_1KB, payload: Box::new([0; SIZE_1KB]), next : None}), 
//             base: get_base(), 
//             used: None, 
//             free: None
//         });
// }

lazy_static::lazy_static! {
    static ref ALLOCATOR : Mutex<Allocator> = 
        Mutex::new(Allocator::new());
}

struct Allocator {
    num_used: usize, 
    num_free: usize, 
    base: Box<Block>,
    used: Option<Box<Block>>,
    free: Option<Box<Block>>
}

impl Allocator {
    fn new() -> Allocator {
        Allocator {
            num_used : 0, 
            num_free : 0, 
            base : Box::new(
                Block {
                    size : 0,
                    payload : 0 as *mut i8,
                    next : None
                }
            ), 
            used : None,
            free : None
        }
    }
}

struct Block {
    size: usize,
    // payload: Vec<u8>, // byte array 
    // payload: Box<[u8]>,
    payload: *mut i8,
    next: Option<Box<Block>>
}

unsafe impl Send for Block {}
unsafe impl Sync for Block {}

fn inc() -> usize {
    let mut guard = ALLOCATOR.lock().unwrap();
    (*guard).num_used += 1;
    let copy = (*guard).num_used;
    return copy;
}


// fn malloc(size: usize) -> *mut u8 {
//     let mut guard = ALLOCATOR.lock().unwrap();
//     (*guard).base.next = Some(Box::new(Block {size: size, payload: Box::new([0; SIZE_1KB]), next : None}));
//     // call boxinto raw and then store that pointer 
//     let ref mut block = (*guard).base.next.as_ref().unwrap();
//     let ref payload = block.payload;
//     let mut buff = payload.into_boxed_slice();
//     let data = block.payload.as_mut_ptr();
//     std::mem::forget(block.payload);
//     return data;
//     // return 0 as *mut u8; 
// }

fn free(ptr: *mut u8) {
    // find associated block 
    // drop
    // mark as free 
}


pub fn alloc_okay() -> bool {
    return false;
}

pub fn alloc_init() {
    // Assert for 64bit arch
    assert_eq!(usize::MAX, 18446744073709551615, "Expected arch 64");
    println!("Initializing  Allocator");

    let mut x = inc();
    x = inc();
    x = inc();
    x = inc();
    println!("val {}", x);

    // ALLOCATOR.num_used = 1;
    // ALLOCATOR.base = Some(Box::new(Block {size: SIZE_1KB, payload: vec![0; SIZE_1KB], next : None}));
    // ALLOCATOR.free = ALLOCATOR.base; 

    // unsafe {

        // BASE_PTR = Box::into_raw(Box::new(Block {size: SIZE_1KB, payload: vec![0; SIZE_1KB], next : None})); 
        // (*BASE_PTR).payload[0] = 10;
        // println!("val {}", (*BASE_PTR).payload[1]);
    // }
}

