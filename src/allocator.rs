use std::sync::Mutex;
use std::mem; 

// 1 KB block = 1024 / word
const MIN_BLOCK_SIZE : usize = 512; 
// const 1b : usize = 1024; 


pub struct List {
    head: Option<Box<Node>>
}

struct Node {
    elem: i32, 
    next: Option<Box<Node>>
}

impl List {
    pub fn new() -> Self {
        List { head : None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem, 
            next: mem::replace(&mut self.head, None)
        }); 

        self.head = Some(new_node)
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, None) {
            None => {
                return None;
            }
            Some(node) => {
                self.head = node.next;
                return Some(node.elem);
            }
        }
    }
}

lazy_static::lazy_static! {
    static ref ALLOCATOR : Mutex<Allocator> = 
        Mutex::new(Allocator::new());
}

struct Allocator {
    num_used: usize, 
    num_free: usize, 
    base: Box<Block>,

}

impl Allocator {
    fn new() -> Allocator {
        Allocator {
            num_used : 0, 
            num_free : 0, 
            base : Box::new(
                Block {
                    size : 0,
                    used : true, 
                    payload : 0 as *mut u8,
                    next : None
                }
            ), 
        }
    }
}

struct BlockList {
    block_size: usize,
    head: Block
}

struct Block {
    size: usize,
    used: bool, 
    payload: *mut u8,
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

pub fn malloc(size: usize) -> *mut u8 {
    if size == 0 {
        return 0 as *mut u8; 
    }

    let mut guard = ALLOCATOR.lock().unwrap();

    // TODO: figure out block sizes
    // 0.5KB, 1KB, 2KB, etc.. 
    let m_size : usize = MIN_BLOCK_SIZE; 

    let mut mem : Vec<u8> = vec![0; m_size];
    let mut payload = mem.into_boxed_slice();
    let payload_ptr = payload.as_mut_ptr();

    // don't drop memory when var out of scope 
    std::mem::forget(payload);

    let mut new_block = 
        Box::new(
            Block {
                size : m_size, 
                used : true,
                payload : payload_ptr,
                next : None
            });

    

    // add block to allocator
    let mut p = &(*guard).base.next;
    let mut q = &(*guard).base.next;
    while q.is_some() && q.as_ref().unwrap().next.is_some() {
        p = &(p.as_ref().unwrap().next);
        q = &(q.as_ref().unwrap().next.as_ref().unwrap().next);
    }

    // p = &Some(new_block);

    // match &(*guard).used {
    //     None => {
    //         println!("malloc first");
    //         (*guard).base.next = Some(new_block);
    //         (*guard).num_used += 1;
    //     },
    //     Some(x) => 
    //     {
    //         println!("malloc +1");

    //         (*guard).num_used += 1;
    //     }
    // }

    return payload_ptr;
}

fn free(ptr: *mut u8) {
    // find associated block 
    // drop
    // mark as free 

    // if !ptr.is_null() {
    //     unsafe { Box::from_raw(ptr); }
    // }
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

