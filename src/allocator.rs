use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;
use std::sync::Mutex;

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
    blocks: Vec<Block>,
    // etext: *const u8,
    // end: *const u8,
}

fn get_block_size(size: usize) -> usize {
    let min_block_size = 521; // 0.5 KB
    let s1kb = min_block_size * 2;
    let s2kb = s1kb * 2;
    let s4kb = s2kb * 2;
    let s8kb = s4kb * 2;
    let s32kb = s8kb * 2;
    let s64kb = s32kb * 2;
    let s128kb = s64kb * 2;
    let sizes: [usize; 8] = [min_block_size, s1kb, s2kb, s4kb, s8kb, s32kb, s64kb, s128kb];

    for s in sizes {
        if size < s {
            println!("Returning size {}", s);
            return s;
        }
    }

    unimplemented!("Requested size is too large");
}

impl Allocator {
    // pub fn new(etext: *const u8, end: *const u8) -> Allocator {
    pub fn new() -> Allocator {
        Allocator {
            num_used: 0,
            num_free: 0,
            blocks: Vec::new(),
            // etext: 0 as *const u8,
            // end: 0 as *const u8,
        }
    }

    fn malloc(&mut self, size: usize) -> *mut u8 {
        println!("alloc malloc");
        let m_size: usize = get_block_size(size);
        let mem: Vec<u8> = vec![0; m_size];
        let mut payload = mem.into_boxed_slice();
        let payload_ptr = payload.as_mut_ptr();

        // don't drop memory when var out of scope
        std::mem::forget(payload);

        // TODO: move block info onto payload return block - offset
        let new_block = Block {
            size: m_size,
            request_size: size,
            used: true,
            payload: payload_ptr,
        };

        self.blocks.push(new_block);
        println!("num blocks = {}", self.blocks.len());

        return payload_ptr;
    }

    fn free(&mut self, ptr: *mut u8) {
        println!("alloc free");

        let mut rm_idx: Option<usize> = None;
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

    // Create new graph every sweep
    // TODO: optimize, only update graph with changes and don't make new
    pub fn create_heap_graph(&self) {
        // Iterate through the blocks and find pointers from heap to heap
        // Key is allocation pointer
        // Value is if the alloc contains pointers in it
        let mut hg: HashMap<*mut u8, HashSet<*mut u8>> = HashMap::new();
        // Object pointers and their size
        let mut objs: HashMap<*mut u8, usize> = HashMap::new();

        // Sweep for objects
        for i in 0..self.blocks.len() {
            self.blocks.get(i).map(|b| {
                println!("Inserting key in hg {:p}", b.payload);
                hg.insert(b.payload, HashSet::new());
                objs.insert(b.payload, b.size);
            });
        }

        // Sweep for pointers
        let step = mem::size_of::<usize>() as usize;
        for i in 0..self.blocks.len() {
            self.blocks.get(i).map(|b| {
                println!("Sweeping new heap object {:p}", b.payload);
                // j is a potential pointer to another heap ref if there's a match
                let mut offset: usize = 0;
                while offset < b.size {
                    // unsafe {
                    //     println!(
                    //         "potential pointer 0x{:02x}",
                    //         *(b.payload.offset(offset as isize) as *const usize)
                    //     );
                    // }
                    for (obj_ptr, obj_size) in &objs {
                        println!("Checking to see if there are references to {:p}", *obj_ptr);
                        if *obj_ptr == b.payload {
                            // don't check for self
                            println!("Skip self reference");
                            continue;
                        }
                        unsafe {
                            // let pref = *obj_ptr.offset(offset as isize);
                            let pref = *(b.payload.offset(offset as isize) as *const usize);
                            println!("Potential pointer 0x{:02x}", pref);

                            if (pref as usize) >= (*obj_ptr as usize)
                                && (pref as usize) < (*obj_ptr as usize) + obj_size
                            {
                                // b.payload contains reference to this block
                                println!("Found ref!");
                                hg.entry(b.payload)
                                    .and_modify(|edges: &mut HashSet<*mut u8>| {
                                        edges.insert(*obj_ptr);
                                        // println!("obj at {:p} contains {} heap references",
                                        //     // *obj_ptr,
                                        //     b.payload,
                                        //     edges.len()
                                        // );
                                    });
                            }
                        }
                    }
                    offset += step;
                }
            });
        }

        // Iterate over hg
        for (obj, references) in &hg {
            print!(
                "Heap Object {:p} contains {} references: ",
                *obj,
                references.len()
            );
            for r in references {
                print!("{:p}, ", *r);
            }
            print!("\n");
        }
    }

    // etext is the last address past the text segment
    // end is the address of the start of the heap and last address pass the BSS
    // These variables are provided via the linux linker
    // TODO: move these variables to allocator initailizer since they don't change
    pub fn sweep_root_mem(&self, etext: *const u8, end: *const u8) {
        println!(
            "Sweep Initialized Data & BSS Regions from {:p} to {:p}",
            etext, end
        );
        // Scan through global memory region (initialized and uninitialized - BSS)
        // Scan etext (low address) --> end (high address)

        // Scan through stack which grows high to low
        // Start from stack bottom (high address) --> end / stack top (low address)
    }

    pub fn find_mem_leaks() {
        // find garbage = leaked objects
        // dfs on hg
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

pub fn garbage_collect(etext: *const u8, end: *const u8) {
    let mut guard = ALLOCATOR.lock().unwrap();
    println!("Garbage collecting");
    guard.create_heap_graph();
    guard.sweep_root_mem(etext, end);
}

pub fn alloc_clean() {
    // TODO: fix me, allocator doesn't clean up
    println!(
        "size of mutex allocator {}",
        std::mem::size_of::<Mutex<Allocator>>()
    );
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
