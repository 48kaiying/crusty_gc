use std::collections::HashMap;
use std::collections::HashSet;
use std::error;
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
            // println!("Returning size {}", s);
            return s;
        }
    }

    unimplemented!("Requested size is too large");
}

pub fn is_aligned_as_eight(addr: usize) -> bool {
    return addr % 8 == 0;
}

// Makes sure an address is 8-byte aligned
// Will round up unless round_down is true
pub fn align_as_eight(addr: usize, round_down: bool) -> usize {
    if !is_aligned_as_eight(addr) {
        if round_down {
            return addr - (8 - addr % 8);
        } else {
            return addr + (8 - addr % 8);
        }
    } else {
        return addr;
    }
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
        // println!("alloc malloc");
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
        // println!("num blocks = {}", self.blocks.len());

        return payload_ptr;
    }

    // Returns a tuple of the alloc request size and the actual alloc size
    // for the block that has been freed.
    fn free_verbose(&mut self, ptr: *mut u8) -> (usize, usize) {
        // println!("alloc free");
        let mut rm_idx: Option<usize> = None;
        let mut req_size = 0;
        let mut size = 0;
        for i in 0..self.blocks.len() {
            self.blocks.get(i).map(|b| {
                if b.payload == ptr {
                    rm_idx = Some(i);
                    req_size = b.request_size;
                    size = b.size;
                    // println!("Found pointer match req size was = {}", b.request_size);
                    unsafe {
                        // println!("Dropping ptr");
                        Box::from_raw(ptr);
                    }
                }
            });
        }

        match rm_idx {
            None => {
                println!("Invalid free call");
                return (0, 0);
            }
            Some(i) => {
                // println!("Removing block at index {}", i);
                let size_prev = self.blocks.len();
                self.blocks.swap_remove(i);
                assert_eq!(self.blocks.len(), size_prev - 1);
                return (req_size, size);
            }
        }
    }

    fn free(&mut self, ptr: *mut u8) {
        self.free_verbose(ptr);
    }

    // Print all blocks with verbose info
    pub fn inspect_blocks(&self) {
        println!("Inspecting allocator blocks");
        let mut count = 1;
        for b in self.blocks.iter() {
            println!(
                "Alloc #{} - {:p} | {} bytes req | {} bytes alloced",
                count, b.payload, b.request_size, b.size
            );
            count += 1;
        }
    }

    // todo: remove self
    pub fn print_heap_graph(&self, hg: &HashMap<*mut u8, HashSet<*mut u8>>, msg: &'static str) {
        println!("Printing heap graph {}", msg);

        // Get list of pure heap objects
        let mut heap_objs = HashSet::new();
        for i in 0..self.blocks.len() {
            self.blocks.get(i).map(|b| {
                // println!("Inserting key in hg {:p}", b.payload);
                heap_objs.insert(b.payload);
            });
        }

        // Iterate over heap graph
        for (obj, references) in hg {
            print!(
                "{} {:p} contains {} heap reference(s): ",
                if heap_objs.contains(obj) {
                    "H" // heap pointer
                } else {
                    "R" // root pointer
                },
                *obj,
                references.len()
            );
            for r in references {
                print!("{:p}, ", *r);
            }
            print!("\n");
        }
    }

    pub fn print_pointer_set(hs: &HashSet<*mut u8>, msg: &'static str) {
        println!("Printing pointer set: {}", msg);
        let mut count = 1;
        for item in hs {
            println!("Pointer #{} is {:p}", count, *item);
            count += 1;
        }
    }

    // Create new graph every sweep
    // TODO: optimize, only update graph with changes and don't make new
    pub fn create_heap_graph(
        &mut self, // TODO: remove mut here
        etext: *const u8,
        end: *const u8,
        stack_top: *const u8,
        stack_bottom: *const u8,
    ) {
        // Iterate through the blocks and find pointers from heap to heap
        // Key is allocation pointer
        // Value is if the alloc contains pointers in it
        let mut hg: HashMap<*mut u8, HashSet<*mut u8>> = HashMap::new();
        // Object pointers and their size
        let mut objs: HashMap<*mut u8, usize> = HashMap::new();

        // Sweep for objects
        for i in 0..self.blocks.len() {
            self.blocks.get(i).map(|b| {
                // println!("Inserting key in hg {:p}", b.payload);
                hg.insert(b.payload, HashSet::new());
                objs.insert(b.payload, b.size);
            });
        }

        // Sweep for pointers
        let step = mem::size_of::<usize>() as usize;
        for i in 0..self.blocks.len() {
            self.blocks.get(i).map(|b| {
                // println!("Sweeping new heap object {:p}", b.payload);
                // j is a potential pointer to another heap ref if there's a match
                let mut offset: usize = 0;
                while offset < b.size {
                    for (obj_ptr, obj_size) in &objs {
                        // println!("Checking to see if there are references to {:p}", *obj_ptr);
                        if *obj_ptr == b.payload {
                            // don't check for self
                            // println!("Skip self reference");
                            continue;
                        }
                        unsafe {
                            let pref = *(b.payload.offset(offset as isize) as *const usize);
                            // println!("Potential pointer 0x{:02x}", pref);

                            if (pref as usize) >= (*obj_ptr as usize)
                                && (pref as usize) < (*obj_ptr as usize) + obj_size
                            {
                                // b.payload contains reference to this block
                                // println!("Found ref!");
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
        self.print_heap_graph(&hg, "with only heap to heap references");
        self.sweep_root_mem(etext, end, stack_top, stack_bottom, &mut hg, &objs);
    }

    pub fn scan_region(
        start: *const u8,
        end: *const u8,
        step: usize,
        hg: &mut HashMap<*mut u8, HashSet<*mut u8>>,
        objs: &HashMap<*mut u8, usize>,
    ) {
        let scan_range = end as usize - start as usize;
        let mut offset: usize = 0;

        println!(
            "Scan region from {:p} to {:p} (range {}) with step {}-bytes",
            start, end, scan_range, step
        );

        while offset < scan_range {
            // unsafe {
            //     println!(
            //         "Checking value at address {:p} for heap ref",
            //         start.offset(offset as isize)
            //     );
            // }

            for (obj_ptr, obj_size) in objs {
                // println!("Checking to see if there are references to {:p}", *obj_ptr);
                unsafe {
                    let root_address = start.offset(offset as isize) as *const usize;
                    let pref = *root_address;
                    if pref == 0 {
                        continue;
                    }
                    // println!("Potential pointer 0x{:02x}", pref);

                    if (pref as usize) >= (*obj_ptr as usize)
                        && (pref as usize) < (*obj_ptr as usize) + obj_size
                    {
                        // Found root memory reference to this block
                        // println!("Found ref in root!");
                        hg.entry(root_address as *mut u8)
                            .and_modify(|edges: &mut HashSet<*mut u8>| {
                                edges.insert(*obj_ptr);
                            })
                            .or_insert(HashSet::from([*obj_ptr]));
                    }
                }
            }
            offset += step;
        }
    }

    // etext is the last address past the text segment
    // end is the address of the start of the heap and last address pass the BSS
    // These variables are provided via the linux linker
    // TODO: move these variables to allocator initailizer since they don't change
    pub fn sweep_root_mem(
        &mut self, // TODO: remove mut here
        etext: *const u8,
        end: *const u8,
        stack_top: *const u8,
        stack_bottom: *const u8,
        hg: &mut HashMap<*mut u8, HashSet<*mut u8>>,
        objs: &HashMap<*mut u8, usize>,
    ) {
        println!(
            "Sweep Initialized Data & BSS Regions from etext {:p} to end {:p}",
            etext, end
        );
        // Scan through global memory region (initialized and uninitialized - BSS)
        // Scan etext (low address) --> end (high address)

        // Make sure start and end addresses are 8-byte aligned
        let etext_aligned = align_as_eight(etext as usize, false);
        let end_aligned = align_as_eight(end as usize, true);

        // Warn if end is not eight byte aligned
        if end_aligned != end as usize {
            println!(
                "Warning: end address is not 8-byte aligned was 
                {:p} but evaluating as {:p}",
                end, end_aligned as *const usize
            );
        }

        let step = mem::size_of::<usize>() as usize;
        Allocator::scan_region(
            etext_aligned as *const u8,
            end_aligned as *const u8,
            step,
            hg,
            objs,
        );

        // Scan through stack which grows high to low
        // Start from stack bottom (high address) --> stack top (low address)
        assert!(is_aligned_as_eight(stack_bottom as usize));
        assert!(is_aligned_as_eight(stack_top as usize));
        println!(
            "Sweep stack from end/top (low) {:p} to start/bottom {:p}",
            stack_top, stack_bottom
        );
        Allocator::scan_region(stack_top, stack_bottom, step, hg, objs);

        println!("Heap graph after sweeping root memory");
        self.print_heap_graph(&hg, "contains root to heap references");

        self.find_mem_leaks(&hg);
    }

    pub fn graph_DFS(
        start_node: *mut u8,
        visited: &mut HashSet<*mut u8>,
        hg: &HashMap<*mut u8, HashSet<*mut u8>>,
    ) {
        visited.insert(start_node);

        let ref start = start_node;

        match hg.get(start) {
            Some(adj) => {
                for n in adj {
                    if !visited.contains(n) {
                        visited.insert(*n);
                        Allocator::graph_DFS(*n, visited, hg);
                    }
                }
            }
            None => println!("Error. DFS start node does not exist in the heap graph"),
        }
    }

    // Find leaked objects and garbage collect them
    pub fn find_mem_leaks(&mut self, hg: &HashMap<*mut u8, HashSet<*mut u8>>) {
        // println!("Finding memory leaks");

        // Get list of pure heap objects
        let mut heap_objs: HashSet<*mut u8> = HashSet::new();
        for i in 0..self.blocks.len() {
            self.blocks.get(i).map(|b| {
                // println!("Inserting key in hg {:p}", b.payload);
                heap_objs.insert(b.payload);
            });
        }

        // Allocator::print_pointer_set(&heap_objs, "heap objects");

        // First find all non-leaked objects. Run DFS from all root pointer entries in
        // the hg. Track all visited heap objects. Those that are not visited are leaked.
        let mut visited: HashSet<*mut u8> = HashSet::new();
        for ptr in hg.keys() {
            // Run DFS on pointers that are root memory (are not heap objects)
            if !heap_objs.contains(ptr) {
                Allocator::graph_DFS(*ptr, &mut visited, hg);
            }
        }

        // Allocator::print_pointer_set(&visited, "visited pointers");

        // Find leaked objects -- values in heap_objs but not visited.
        let mut leak_count = 1;
        let mut garbage_size = 0;
        for leaked in heap_objs.difference(&visited) {
            println!(
                "RGC: Heap object #{} leaked {:p} and cleaned",
                leak_count, *leaked
            );
            // Free leaked block
            let (_req_size, size) = self.free_verbose(*leaked);
            garbage_size += size;
            leak_count += 1;
        }

        // Print gc collect summary
        println!(
            "RGC SUMMARY: Garbage collected {} objects freeing {} bytes",
            leak_count - 1,
            garbage_size
        );
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        // println!("Allocator dropped");
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
        // println!("calling free");
        return guard.free(ptr);
    }
}

pub fn garbage_collect(
    etext: *const u8,
    end: *const u8,
    stack_top: *const u8,
    stack_bottom: *const u8,
) {
    let mut guard = ALLOCATOR.lock().unwrap();
    // println!("Garbage collecting");
    guard.inspect_blocks();
    guard.create_heap_graph(etext, end, stack_top, stack_bottom);
    guard.inspect_blocks();
}

pub fn alloc_clean() {
    // TODO: fix me, allocator doesn't clean up
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
