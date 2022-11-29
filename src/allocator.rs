use std::sync::Mutex;
use std::mem; 

// 1 KB block = 1024 / word
// const MIN_BLOCK_SIZE : usize = 512;
// const 1b : usize = 1024;

lazy_static::lazy_static! {
    static ref ALLOCATOR : Mutex<Allocator> = 
        Mutex::new(Allocator::new());
}

struct Block {
    // header size = 24
    size: usize,
    request_size: usize,  
    used: bool,
    payload: *mut u8,
}

unsafe impl Send for Block {}
unsafe impl Sync for Block {}

struct Allocator {
    num_used: usize, 
    num_free: usize, 
    blocks: Vec<Block>
}

fn get_block_size(size : usize) -> usize {
    let min_block_size = 521;  // 0.5 KB
    let s1kb = min_block_size * 2; 
    let s2kb = s1kb * 2; 
    let s4kb = s2kb * 2; 
    let s8kb = s4kb * 2; 
    let s32kb = s8kb * 2; 
    let s64kb = s32kb * 2;
    let s128kb = s64kb * 2;
    let sizes : [usize; 8] = 
        [min_block_size, s1kb, s2kb, s4kb, s8kb, s32kb, s64kb, s128kb]; 

    for s in sizes {
        if size < s {
            println!("Returning size {}", s);
            return s;
        }
    }

    unimplemented!("Requested size is too large");
}


impl Allocator {

    pub fn new() -> Allocator {
        Allocator {
            num_used : 0, 
            num_free : 0, 
            blocks : Vec::new(),
        }
    }

    fn malloc(&mut self, size : usize) -> *mut u8 {
        println!("alloc malloc");
        let m_size : usize = get_block_size(size); 
        let mem : Vec<u8> = vec![0; m_size];
        let mut payload = mem.into_boxed_slice();
        let payload_ptr = payload.as_mut_ptr();

        // don't drop memory when var out of scope 
        std::mem::forget(payload);

        // TODO: move block info onto payload return block - offset
        let new_block = 
            Block {
                size : m_size, 
                request_size: size,
                used : true,
                payload : payload_ptr,
            };

        self.blocks.push(new_block);
        println!("num blocks = {}", self.blocks.len());
        
        return payload_ptr;
    }

    fn free(&mut self, ptr : *mut u8) { 
        println!("alloc free");

        let mut rm_idx : Option<usize> = None; 
        for i in 0..self.blocks.len() {
            self.blocks.get(i).map(|b| {
                if b.payload == ptr {
                    rm_idx = Some(i);
                    println!("Found pointer match req size was = {}", b.request_size);
                    unsafe {
                        println!("Dropping ptr");
                        Box::from_raw(ptr); 
                    }
                }
            });
        }
     
        match rm_idx {
            None => println!("Invalid free call"), 
            Some(i) => {
                println!("Removing block at index {}", i);
                let size_prev = self.blocks.len();
                self.blocks.swap_remove(i);
                assert_eq!(self.blocks.len(), size_prev - 1);
            }
        }
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        println!("Allocator dropped");
        self.blocks.clear();
    }
}


pub fn malloc(size: usize) -> *mut u8 {
    if size == 0 {
        return 0 as *mut u8; 
    }

    let mut guard = ALLOCATOR.lock().unwrap();
    return guard.malloc(size);
}

pub fn free(ptr: *mut u8) {
    if !ptr.is_null() {
        let mut guard = ALLOCATOR.lock().unwrap();
        println!("calling free");
        return guard.free(ptr);
    }
}

pub fn alloc_clean() {
    // TODO: fix me, allocator doesn't clean up
    println!("size of mutex allocator {}", std::mem::size_of::<Mutex<Allocator>>());
    let guard = ALLOCATOR.lock().unwrap();
    // drop(guard); 
    std::mem::drop(guard);
    // std::mem::drop(ALLOCATOR);
}

pub fn alloc_init() {
    // Assert for 64bit arch
    assert_eq!(usize::MAX, 18446744073709551615, "Expected arch 64");
    println!("Initializing  Allocator");
}


