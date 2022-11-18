use std::sync::Mutex;
// use std::mem; 

// 1 KB block = 1024 / word
// const MIN_BLOCK_SIZE : usize = 512;
// const 1b : usize = 1024; 

lazy_static::lazy_static! {
    static ref ALLOCATOR : Mutex<Allocator> = 
        Mutex::new(Allocator::new());
}

struct Allocator {
    num_used: usize, 
    num_free: usize, 
    head: Option<Box<Block>>,
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
            head : None
        }
    }

    fn malloc(&mut self, size : usize) -> *mut u8 {
        // TODO: figure out block sizes
        // 0.5KB, 1KB, 2KB, etc.. 
        let m_size : usize = get_block_size(size); 

        let mem : Vec<u8> = vec![0; m_size];
        let mut payload = mem.into_boxed_slice();
        let payload_ptr = payload.as_mut_ptr();

        // don't drop memory when var out of scope 
        std::mem::forget(payload);

        let new_block = 
            Box::new(
                Block {
                    size : m_size, 
                    request_size: size,
                    used : true,
                    payload : payload_ptr,
                    next : self.head.take(),
                });
    
        self.head = Some(new_block);
        self.num_used += 1; 
        return payload_ptr;
    }

    // Function does not release payload 
    fn pop(&mut self) {
        match self.head.take() {
            None => {} // do nothing,
            Some(node) => {
                self.head = node.next;
            }
        }
    }

    fn free(&mut self, ptr : *mut u8) {
        println!("alloc free");

        // let mut found_match = false; 

        // find associated block 
        let mut temp = &self.head;
        // let mut temp = &mut self.head;
        // let mut prev = &mut self.head; 
        // let mut is_first = true;
        while temp.is_some() {
            let ref t = temp.as_ref().unwrap();
            // println!("Prev req size was = {}", prev.as_ref().unwrap().request_size);
            // println!("Temp req size was = {}", t.request_size);
            if t.payload == ptr {
                // found_match = true;
                // drop payload
                println!("Found pointer match req size was = {}", t.request_size);
                unsafe {
                    println!("Dropping ptr");
                    Box::from_raw(ptr); 
                }

                // match temp {
                //     None => panic!("temp is null"),
                //     Some(ref mut temp) => 
                //         // I think this is where you create a new block..
                // };

                // TODO: merge algo 

                // uh doesn't work probably wont work --> drop block, relink
                // if is_first { 
                //     // first block 
                //     self.pop(); 
                // } else if t.next.is_none() { 
                //     // last block 
                //     prev.as_ref().unwrap().next = None;
                // } else { // middle block
                //     prev.as_ref().unwrap().next = t.next.take();
                // }

                break; 
            } 
            // prev = temp; 
            temp = &t.next; 
            // is_first = false;
        }

        // if !found_match {
        //     println!("Invalid rgc_free");
        // }
    }
}

struct Block {
    size: usize,
    request_size: usize,  
    used: bool,
    payload: *mut u8,
    next: Option<Box<Block>>
}

unsafe impl Send for Block {}
unsafe impl Sync for Block {}

// fn inc() -> usize {
//     let mut guard = ALLOCATOR.lock().unwrap();
//     (*guard).num_used += 1;
//     let copy = (*guard).num_used;
//     return copy;
// }

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

pub fn alloc_init() {
    // Assert for 64bit arch
    assert_eq!(usize::MAX, 18446744073709551615, "Expected arch 64");
    println!("Initializing  Allocator");

    // let mut x = inc();
    // x = inc();
    // x = inc();
    // x = inc();
    // println!("val {}", x);

    // add block to allocator
    // let mut p = &(*guard).base.next;
    // let mut q = &(*guard).base.next;
    // while q.is_some() && q.as_ref().unwrap().next.is_some() {
    //     p = &(p.as_ref().unwrap().next);
    //     q = &(q.as_ref().unwrap().next.as_ref().unwrap().next);
    // }
}

