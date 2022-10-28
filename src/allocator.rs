use std::sync::Mutex;

// 1 KB block = 1024 / word
const SIZE_1KB : usize = 1024; 
// static mut BASE_PTR : *mut Block = 0 as *mut Block;

lazy_static::lazy_static! {
    static ref ALLOCATOR : Mutex<Allocator> = 
        Mutex::new(Allocator { 
            num_used: 0, 
            num_free: 0, 
            base: Box::new(Block {size: SIZE_1KB, payload: vec![0; SIZE_1KB], next : None})
            // , 
            // used: base, 
            // free: None
        });
}

struct Allocator {
    num_used: usize, 
    num_free: usize, 
    base: Box<Block>
    // ,
    // mut &used: <Box<Block>,
    // mut free: Option<Box<Block>
}

// impl Allocator {
//     fn inc(&mut self) {
//         self.num_used += 1; 
//     }

//     // pub fn malloc(&mut self, size: usize) -> mut u8* {

//     // }
// }

fn inc() -> usize {
    let mut guard = ALLOCATOR.lock().unwrap();
    (*guard).num_used += 1;
    let copy = (*guard).num_used;
    return copy;
}

struct Block {
    size: usize,
    payload: Vec<u8>, // byte array 
    next: Option<Box<Block>>
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

