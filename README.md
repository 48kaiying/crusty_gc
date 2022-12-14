# Rust Garbage Collector (RGC) for C Programs 

Kaiying Guo

Advised by Norm Badler and Adam Mally 

Fall 2022 CIS 4970 Senior Capstone Design Project 

The University of Pennsylvania 

# Abstract 

Rust Garbage Collector (RGC) is a garbage collector for C programming inspired by the Boehm garbage collector. Garbage collection is form of automatic memory management which attempts to reclaim garbage -- memory that is no longer referenced -- by the program. RGC provides API call `rgc_malloc` which allocates heap memory similar to the standard `malloc` call. RGC also provides a API call `rgc_garbage_collect` which determines garbage at the current program execution point and clears the memory thereby freeing developers from having to manually release memory (calling `free` explicitly). RGC only works for **single threaded C programs running on x86-64 Linux** since the code is reliant on the Linux kernel.

This project was created with the purpose of learning Rust and is an academic endeavor to demystify garbage collection for myself. RGC is not meant to be a robust or wholly correct garbage collector to be used for production/serious software as there are bugs that have not been hashed out.

Google slides for the final project presentation explaining the algorithm and implementation can be found [at this link here](https://docs.google.com/presentation/d/1FYoHnk3ZLhpRNy4YOdfhOIGkaC9evFwXEp_KMASxNkA/edit#slide=id.g180cc10c6d2_0_9.).

# Introduction 

Any running program takes up memory. Every variable declared takes up memory to store the variable value. Every running program also has its own isolated memory layout which is partitioned into four main regions:

- `text region` which stores the program code to be executed abd is a protected region which can not be written to. The amount of code does not change during execution time so this region size is fixed.
- `data region` which stores static and global variables which exist during the entire duration of the program. This region contains initialized data and uninitialized data (BSS region). The size of all global variables is determined at compile time so this region size is fixed. 
- `stack region` which contains local variables. Variables are allocated and de-allocated automatically - when a variable is declared and when it goes out of scope respectively. This is a relatively small region and grows from "backwards"-- from a high address to a low address.
- `heap region` or dynamic memory is the alternative to stack memory and requires the programmer to explicitly allocate. In C, this is done with `malloc` and its variations. Heap allocated memory remains allocated until the programmer explicitly de-allocates with `free` or when the program terminates. Memory leaks and garbage collection is interested in automatically `free`ing allocations that are no longer accessible by the programmer in this region.

The problem with manual memory management (requiring the programmer to explicitly de-allocate) is that people are notoriously bad at it. People often forget or simply do not know *when* memory is no longer referenced and can be de-allocated. Higher level languages like Java and C# contain automatic memory management to resolve this issue.

Garbage collection (GC) is expensive and adds program overhead, thus performance critical code written in lower level languages like C and C++ don't come with automatic GC. Therefore, members of the graphics/games/systems/UI community that heavily use C/C++ benefit from having a robust understanding of memory management which motivates my project as I am a member in this "frequently using C/C++" community. 

The solution to gaining better understanding is to write an garbage collector in Rust. Why garbage collection? Because it is the automatic solution to what is required of us manually. We will follow the implementation of the classic Boehm GC for C. Why Rust? Because Rust is one of the fastest growing languages with extraordinary memory safety features without GC and no performance cost (as fast as C/C++) and learning it developes even deeper understanding of memory usage.

The project will make the following contributions:
- Provide a Rust garbage collector for C programs
- Provide a public reference for writing a Rust garbage collector for C. 
- Motivate understanding of memory management

## Design Goals

This project is of interest to people who want to learn about Rust, garbage collection, memory leak detection, and memory management. The audience will benefit from being able to write their own RGC and to use this one to collect garbage in a naive single threaded C program. 

The users of my project are C programmers who want a garbage collector. 

## Project Proposed Features and Functionality

The project will provide a dynamic Rust library `lrustgc` (found in `target/debug/librustgc.d`) which can be linked to by the C program (reference `build_run.sh` as an example) and a C API for using RGC  which can be imported easily. 

The features of RGC are listed in the API found in `c_app/src/rustgc.h`. To build and run the example c project using RGC run the build script `build_run.sh`. To observe Valgrind memory leak output on a already created executable run `run_valgrind.sh`. 

## Related Work

The project is inspired by the Boehm GC and uses the Mark Sweep algorithm for garbage collection. For reference about Rust, I used the [Rust Lang Book](https://doc.rust-lang.org/book/). 

# Project Proposal 

By the end of the project the following test case should contain the same behavior. 

C example with no leaks and manual memory management: 

```c
void test()
{
    int size = 6000000;
    // heap allocate a giant int array 
    int* big_int_array = (int*) malloc (size * sizeof(int)); 
    // fill array
    int* temp = big_int_array;
    for (int i = 0; i < size; ++i) { *temp++ = i; }
    // explicit user free required
    free(big_int_array);
}

int main() 
{
    test();
    return 0;
}
```

C example using RGC should also have no leaks: 

```c 
#include "rustgc.h"

void test()
{
    int size = 6000000;
    int* big_int_array = (int*) rgc_malloc (size * sizeof(int)); 
    int* temp = big_int_array;
    for (int i = 0; i < size; ++i) { *temp++ = i; }
    // no user free required
    rgc_garbage_collect();
}

int main() 
{
    rgc_init();
    test();
    return 0;
    rgc_cleanup(); 
}
```

## Anticipated Approach

1. Implement a `rgc_malloc(unsigned int)` function and a memory allocator. The user can request a chunk of memory and a block of the correct size is returned. If no blocks of the right size exist the linked list is partitioned into smaller blocks or more memory is requested (Buddy System for memory partition). Each call to `rgc_malloc` will prepend a tagged `header struct` to the allocation which allows us to later identify that the object was allocated using `rgc_malloc` call. 

2. Writing the collector via the Mark Sweep algorithm. Garbage collection is two parts -- the mark phase which enumerates all the set of live objects and the sweep phase which removes the allocations that no longer have references to them. For the mark phase, we know all the live objects must have been allocated with RGC_malloc so all the live objects must be in the allocator linked list. Iterate over the heap (the blocks) word by word and treat each word as a pointer. If the pointer points within some allocated block in the linked list then we know there is a reference to an object in another object. Create a graph of heap references to other heap references. Next, iterate through root (user accessible) memory regions e.g. BSS, stack, global, registers and treat every word as a pointer. If the pointer points to a heap object, add a source to the node to the graph of heap references. For the sweep phase we will run DFS from every source node in the graph of heap references and note all the objects that the traversal touches -- these are non leaked objects. The remaining objects that are not touched by the DFS can???t be user accessed so they must be garbage collected (and also leaked). 

3. Lastly, I will be wrapping Rust functions with a C API using the following https://docs.rust-embedded.org/book/interoperability/rust-with-c.html

4. Include verbose logging that tells the user information about what has been garbage collected including the number of allocations and total bytes collected at the point of the call.

## Target Platform 

The project is intended only to run for single threaded c programs on x86-64 Linux. For the development of RGC, I used Windows Subsystem for Linux (WSL) - Ubuntu.

## Evaluation Criteria

Valgrind output of memory leaks running the same code with and without garbage collection. We should be able to compare the memory leak size difference and it should match the summary output of RGC. 

The following is an example of RGC output after it has identified `3126 bytes` of trash and collected it:

```
RGC: Heap object #1 leaked 0x556b9d356e80 and cleaned
RGC: Heap object #2 leaked 0x556b9d356a40 and cleaned
RGC: Heap object #3 leaked 0x556b9d3563e0 and cleaned
RGC: Heap object #4 leaked 0x556b9d356c60 and cleaned
RGC: Heap object #5 leaked 0x556b9d356820 and cleaned
RGC: Heap object #6 leaked 0x556b9d356600 and cleaned
RGC SUMMARY: Garbage collected 6 objects freeing 3126 bytes
```

When running Valgrind with and without `rgc_garbage_collect_nice()` call the leaked memory difference should match. 

Valgrind with gc:
```
==6303== LEAK SUMMARY:
==6303==    definitely lost: 0 bytes in 0 blocks
==6303==    indirectly lost: 0 bytes in 0 blocks
==6303==      possibly lost: 0 bytes in 0 blocks
==6303==    still reachable: 6,786 bytes in 13 blocks
==6303==         suppressed: 0 bytes in 0 blocks
```

Valgrind *without* gc should show an increase of the same size as RGC's summary. So 6,786 + 3,126 = 9,912. 
```
==6525== LEAK SUMMARY:
==6525==    definitely lost: 0 bytes in 0 blocks
==6525==    indirectly lost: 0 bytes in 0 blocks
==6525==      possibly lost: 0 bytes in 0 blocks
==6525==    still reachable: 9,912 bytes in 19 blocks
==6525==         suppressed: 0 bytes in 0 blocks
```


# Project Timeline 

The following is a speculated pace of the project. 

1. September: Research about garbage collection, write proposal, get familiar with Rust
2. October: Create a development env, develope methods for testing, implement the allocator fleshing calls `rgc_init`, `rgc_malloc`, `rgc_free`, and `rgc_cleanup`.
3. November: Implement leak detection and garbage collection fleshing the call `rgc_garbage_collect`.
4. December: Stretch goals, debugging, robust testing, presenting

## Project Future Tasks

- Implement parallel garbage collection by garbage collecting on another thread so that RGC does not stop the main project execution
- Flesh out garbage collection data by collecting runtime type information and displaying to user 
- Optimize the memory allocator

## Method & Results

### 1. The Dev Environment

The first step of the project was to create a development environment. The set up is two fold. In order to run Rust code in a C project, first a C-friendly Rust API is required. Secondly, the Rust project must be linked to by the external build system. To prioritize being able to test end to end (running Rust calls from a C project), stub Rust commands were first created for purpose of testing the C API and ensuring that the Rust project was linked correctly. First, I edited the `cargo.toml` to emit a dynamic system library `rustgc` instead of a standard Rust target. I chose GCC as the default compiler for the C application and used the linker flags to link to the library. Next I created the C API using the Rust ABI and can be viewed in `src/lib.rs`. For every function in the Rust API, a corresponding header function in C is required (see file `c_app/src/rustgc.h`). To stream line everything, I wrote a build script `build_run.sh` which compiles the Rust library and then the C application which links to said library. For source of truth on this I referred to the [Rust docs](https://docs.rust-embedded.org/book/interoperability/rust-with-c.html). 

### 2. The Naive Allocator 

Now that I was able to build a Rust project and call Rust functions from C, I began on writing the actual garbage collector. The first part of garbage collection is creating an allocator. The allocator manages allocations in containers and implements the the `rgc_malloc` call. The goal is such that a user can request memory in C in the same way they would use `malloc`. But instead of having the C program allocate memory, the request would go through the Rust allocator. The purpose of this is so we are able to keep track of every heap allocation made by the user.

Writing the allocator was the most difficult part for me and had many false starts. First, in order to create an allocator that would last the entire duration of the C project, I had to declare a global Allocator object. In Rust "global" variables are "static", however the Allocator isn't actually "static" it's mutable -- changes during execution time. This means a Rust type of `static mutable` was required. However, static mutable objects are not memory safe and the compiler through a big fuss. The solution was to use the `lazy_static` crate which allows initialization of static variable at runtime and wrapping the Allocator in a mutex since mutable static objects can not be safely passed between threads.

Now that I had an Allocator I needed to implement `rgc_malloc`. I needed to be able to declare a chunk if memory in Rust, some how store a reference to that point in memory in an internal Rust structure, and then and pass a pointer to that chunk of memory back to the C program. Initially in the project proposal, I wanted to write an allocator that used Buddy allocation, thus the internal data structure I used was a linked list of `Block` which stores the requested allocation size and a `vec u8` (a `vec` us a Rust heap allocated array and `u8` means unsigned 8-bit integer). It turned out this was not the right move. While implementing enqueue and dequeue to the linked list was not terrible, implementing deletion in the middle of the array was strongly discouraged in Rust as there is no real way to this safely. After fighting this for a long time, I pivoted for time sake and used a vec of Blocks and side stepped Buddy allocation algorithm to finish the project. 

Secondly, another big hurtle which I had to overcome was figuring out how to return the pointer back to C *and* store it in a Rust vec while making the compiler content. The vec memory which gets written to by the C program is referred to as the `payload` in my project. Thus, the internal data structure is a vec of vec pointers. There were two issues required resolution. First, a feature of Rust is that it de-allocates whenever a variable is no longer in scope, we do not want the `payload vec` to be deallocated after the `malloc` function finishes running. Secondly, the compiler does not allow us to store a `vec` pointer that is returned to the C program because the C program can modify the memory in unsafe ways (valid). The solution, after much research and discussing with a PhD student who knew rust was to use the Rust `Box` module which allowed me to turn the `vec` into a mutable pointer which could be returned (solving problem 2), and also calling `std::mem::forget` on said mutable pointer which tells the Rust compiler to not drop the payload memory when it goes out of scope. This process taught me a lot about Rust and also gave me a lot of pain since it felt like everything I was trying to do was not safe. At this point of the project, the progress had been slow and I wasn't sure if I would be able to achieve what I wanted using Rust.

The last part of the allocator is implementing `rgc_free` which, after deciding to simplify the implementation to a vec, I iterate over the allocated blocks and look a `payload` which matched the pointer that was passed in. Using Rust `Box` method `Box::from_raw` and passing the payload pointer in, I was able to successfully allowed Rust to de-allocate the payload memory. Calling `from_raw` is unsafe, but this is the most straight forward way to do it so I just wrapped the call in `unsafe` brackets. 

By the end of this section, I was able to allocate and de-allocate memory using the Rust allocator in C. 

### 3. Heap to Heap References 

Now that I am able to track and identify all heap allocations, the next step is identifying what is garbage. Garbage is any heap allocation that can not be reference to by the user during the program execution point. Thus, we need to be able to know what heap allocations have references to it.

What can point to a heap allocation? Well, another heap allocation! This is the case for instance, when we allocate a `struct A` which stores a pointer to another heap allocated `struct B`. See the following test case `test_heap_graph()` in `c_app/application.c` which contains a heap allocated `Point_container` which itself stores five pointers to heap allocated `Point` structs. See `test_heap_graph()` test case in `c_app/application.c`. 

```c
typedef struct Point_container
{
    struct Point *first;
    struct Point *sec;
    struct Point *third;
    struct Point *fourth;
    struct Point *fifth;
} Point_container;

void test_heap_graph()
{
    Point_container *pc = (Point_container *)rgc_malloc(sizeof(Point_container));
    ...
}

void fill_point_container(Point_container *pc)
{
    Point *first = (Point *)rgc_malloc(sizeof(Point));
    Point *sec = (Point *)rgc_malloc(sizeof(Point));
    Point *tri = (Point *)rgc_malloc(sizeof(Point));
    Point *four = (Point *)rgc_malloc(sizeof(Point));
    Point *fifth = (Point *)rgc_malloc(sizeof(Point));
    ...
}
```

The implementation for finding heap references is in the `create_heap_graph(...)` method in Rust allocator. The algorithm is as follows: for every allocated Block, iterate over the contents of the `payload vec` 8 bytes at a time and treat every 8 bytes as a pointer (a memory address). In 64-bit a pointer is quadword (8 bytes) which is why we iterate 8 bytes at a time. This pointer is called a potential reference. A potential reference is an *actual* reference when the address is between another heap object address and the heap object address + allocation size. This addresses the case where a heap object can reference the middle of another heap object. 

Implementing this was kind of meta since in Rust, I store the 8 byte contents in a local potential reference `pref` variable, so in order to actually read the right address, I needed to dereference pref due to the way Rust iterators work. This was a nuance that took some time to figure out and for a while the results were not matching up. 

For implementation, the heap graph is a `HashMap` which uses a heap object pointer as the key and a `HashSet` of other heap objects as the value. When a heap object `A` referred to heap object `B` then the pointer for `B` is added to the HashSet value at key `A`. 

By the end of this section, I had also implemented logging which printed the contents of the heap graph and saw the expected outcome - there is a heap allocation (the point container) which contains 5 other heap references (the points). 

The following is the test output. "Inspecting Allocator Blocks" prints all the blocks in the allocator. Here we see after "Printing heap graph with only heap to heap references", there are 6 nodes in the graph and only one contains 5 references. That makes sense since only `pc` contains references to `points`. The `H` in front of the memory address identifies that the address is a heap pointer, this will be relevant in the next section. 

```
#################### Test 5 ####################
Test heap graph point container
Inspecting allocator blocks
Alloc #1 - 0x55d246c05af0 | 40 bytes req | 521 bytes alloced
Alloc #2 - 0x55d246c05da0 | 16 bytes req | 521 bytes alloced
Alloc #3 - 0x55d246c05fc0 | 16 bytes req | 521 bytes alloced
Alloc #4 - 0x55d246c061e0 | 16 bytes req | 521 bytes alloced
Alloc #5 - 0x55d246c06400 | 16 bytes req | 521 bytes alloced
Alloc #6 - 0x55d246c06730 | 16 bytes req | 521 bytes alloced
Printing heap graph with only heap to heap references
H 0x55d246c06400 contains 0 heap reference(s): 
H 0x55d246c05da0 contains 0 heap reference(s): 
H 0x55d246c06730 contains 0 heap reference(s): 
H 0x55d246c05af0 contains 5 heap reference(s): 0x55d246c05da0, 0x55d246c061e0, 0x55d246c06400, 0x55d246c05fc0, 0x55d246c06730, 
H 0x55d246c061e0 contains 0 heap reference(s): 
H 0x55d246c05fc0 contains 0 heap reference(s): 
```

### 4. Root Memory to Heap References (Data Region)

Now that we know what heap objects point to other heap objects, we need to consider what else can point to a heap object? A global data structure or a local variable! By this point of the project, I could see that there was a trend. I needed to iterate over memory regions from a start address to and end address 8 bytes at a time looking for potential references modifying my heap graph. Thus, I refactored my code and wrote a `scan_region(...)` function which would do just that. 

The parts of memory that are accessible by the user during execution is called root memory. This contains globals, stack (local variables), and registers. In a multi-threaded program we would also include thread stack and thread registers, but that is beyond the scope of this project.

The following test case `test_scan_data_region()` contains two global variables `gcp_1` and `gcp_2` which both contain heap references.
```c
struct Point_container gpc_1 = {
    .first = NULL,
    .sec = NULL,
    .third = NULL,
    .fourth = NULL,
    .fifth = NULL,
};

struct Point_container gpc_2 = {
    .first = NULL,
    .sec = NULL,
    .third = NULL,
    .fourth = NULL,
    .fifth = NULL,
};

void test_scan_data_region()
{
    printf("Test scan data region\n");
    fill_point_container(&gpc_1);
    fill_point_container(&gpc_2);
    rgc_garbage_collect_nice();

    printf("Expected output is a new heap object %p with 5 references:\n", &gpc_1);
    printf("Global gpc_1:\n");
    print_point_container(&gpc_1);
    printf("Global gpc_2:\n");
    print_point_container(&gpc_2);
}
```

This was a tricky part of the project that required a lot of research, but not that much code. I needed to figure out how exactly the Linux kernel formats memory for a C program and how I could have access to the data and stack regions. Good thing there are a lot of manuals for this. 

Reading [the Linux man page](https://linux.die.net/man/3/etext), I discovered that Unix systems declare symbols `etext` and `end` which contains the first address past the text segment (which is the start of the initialized data segment) and the first address pass the BSS data segment respectively. These symbols needed to be explicitly declared in the C program, thus I declared them in the RGC API. RGC would be passed the `etext` and `end` addresses. While scanning this data region, initially no heap references were found. It wasn't until closer examination did I realize that `etext` was not always 8-byte aligned. Thus, I was incorrectly iterating over the region misaligned. I am not sure about the correctness of this part, but to fix the issue I made sure that `etext` was an 8 byte aligned address and rounded the address down. This seemed to fix the issue and I began to see root references to heap objects in the heap graph. 

The following is the heap graph after scanning the data regions for `test_scan_data_region()`. Here we see that the graph now contain root memory keys denoted by `R`. There are two global variable `gpc_1` and `gpc_2` and each of them contain 5 references, so we should see 10 unique heap references as values in the heap graph which we do.

```
#################### Test 6 ####################
...
Scan region from 0x55e1ce073b28 to 0x55e1ce0760c8 (range 9632) with step 8-bytes
Heap graph after sweeping root memory
Printing heap graph contains root to heap references
R 0x55e1ce076050 contains 1 heap reference(s): 0x55e1ce9eefc0, 
H 0x55e1ce9eeda0 contains 0 heap reference(s): 
R 0x55e1ce076060 contains 1 heap reference(s): 0x55e1ce9ef400, 
H 0x55e1ce9f01c0 contains 0 heap reference(s): 
H 0x55e1ce9ef1e0 contains 0 heap reference(s): 
R 0x55e1ce076058 contains 1 heap reference(s): 0x55e1ce9ef1e0, 
H 0x55e1ce9ef730 contains 0 heap reference(s): 
R 0x55e1ce076048 contains 1 heap reference(s): 0x55e1ce9eeda0, 
R 0x55e1ce076088 contains 1 heap reference(s): 0x55e1ce9ef950, 
H 0x55e1ce9eeaf0 contains 0 heap reference(s): 
R 0x55e1ce076080 contains 1 heap reference(s): 0x55e1ce9ef730, 
H 0x55e1ce9efd90 contains 0 heap reference(s): 
H 0x55e1ce9ef400 contains 0 heap reference(s): 
R 0x55e1ce076040 contains 1 heap reference(s): 0x55e1ce9eeaf0, 
H 0x55e1ce9eefc0 contains 0 heap reference(s): 
R 0x55e1ce076090 contains 1 heap reference(s): 0x55e1ce9efb70, 
H 0x55e1ce9efb70 contains 0 heap reference(s): 
H 0x55e1ce9ef950 contains 0 heap reference(s): 
R 0x55e1ce0760a0 contains 1 heap reference(s): 0x55e1ce9f01c0, 
R 0x55e1ce076098 contains 1 heap reference(s): 0x55e1ce9efd90, 
```

### 5. Root Memory to Heap References (Stack Region)

The next step was to scan the execution stack for heap pointers. First, I identified the stack bottom by reading the `proc/self/stat` file for the 28th value. According to [the Linux man page](https://man7.org/linux/man-pages/man5/proc.5.html), the Linux kernel logs a ton of information about a process in a `proc/[pid]/stat` file and the 28th value is the start of the stack. Next, I needed to figure out how to get the stack top address. This part was straight forward but took me a bit of time to figure out. In x86-64, the `rsp` register stores the stack pointer and the `rbp` register stores the stack frame pointer. Using GCC inline assembly, I was able to write the `rbp` value to a local variable. The outcome of this section is only three lines but took me a lot of research since it was the first time I was writing asm in C. It also took me some time to figure out the correct `mov` instruction to use and what the exact types of the operands should be. I did not even know there were different mov instructions. With the stack start and end addresses, I was able to find local variable references to heap objects. 

While scanning root memory regions, new keys needed to be added to the heap graph. These keys are root memory addresses that only contain a single heap reference in the HashSet.

The following test case stores an allocated `stack` object in a local variable `s`. Each element in the stack is heap allocated. 

```c
Stack *make_stack()
{
    Stack *s = (Stack *)rgc_malloc(sizeof(Stack));
    return s;
}

void test_scan_stack_region()
{
    printf("Test n");
    Stack *s = make_stack(); // local variable points to heap obj 
    s->head = NULL;
    stack_add(s, 1);
    stack_add(s, 2);
    stack_add(s, 3);

    rgc_garbage_collect_nice();

    printf("Expected output from c:\n");
    printf("Nothing is leaked.\n");
    printf("Heap graph should contain root stack pointer reference %p to heap obj %p\n", &s, s);
    stack_iterate(s);
}
```

The following is the output. It is as we expect, there contains one root pointer (the address local var `s`) which points to a heap reference and all but one heap reference should point to another heap reference (stack elements point to the next stack element). Here we see there is only one `R` root pointer. 
```
#################### Test 9 ####################
Test stack implementation
...
Sweep stack from end/top (low) 0x7ffe245f17e0 to start/bottom 0x7ffe245f1900
Scan region from 0x7ffe245f17e0 to 0x7ffe245f1900 (range 288) with step 8-bytes
Heap graph after sweeping root memory
Printing heap graph contains root to heap references
H 0x5653abe2ada0 contains 0 heap reference(s): 
R 0x7ffe245f17f0 contains 1 heap reference(s): 0x5653abe2aaf0, 
H 0x5653abe2b1e0 contains 1 heap reference(s): 0x5653abe2afc0, 
H 0x5653abe2aaf0 contains 1 heap reference(s): 0x5653abe2b1e0, 
H 0x5653abe2afc0 contains 1 heap reference(s): 0x5653abe2ada0, 
RGC SUMMARY: Garbage collected 0 objects freeing 0 bytes
...
Expected output from c:
Nothing is leaked.
Heap graph should contain root stack pointer reference 0x7ffe245f17f0 to heap obj 0x5653abe2aaf0
Iterating over stack obj 0x5653abe2aaf0
Stack Item Obj 0x5653abe2b1e0 #1 : 3
Stack Item Obj 0x5653abe2afc0 #2 : 2
Stack Item Obj 0x5653abe2ada0 #3 : 1
Stack Bottom
Cleaning RGC
```

### 6. Finding Memory Leaks & Cleaning it Up

With this graph of heap references, I was finally able to find memory leaks to clean. A heap object is leaked if there is no way the user can access the object. In reverse, a heap object is not leaked if it is reachable from some root memory. Thus, the algorithm for identifying leaks is just depth first search (DFS) graph traversal. Iterating over the keys in the heap graph, I identified keys that were a root memory addresses and not heap object address, and ran DFS from these nodes. To find what was leaked, I just took the set difference of all the heap objects and the heap objects that were visited in the traversal.

Since `rgc_free` was already implemented, cleaning up the garbage was iterating over all the leaked objects and calling the method on the payload. This also removed the allocator block to it.

For the following test case we should expect RGC to find and collect the leaked point container. 

```c
void leak_point_container()
{
    // Purposefully leak this point container and all elements inside
    Point_container *pc = (Point_container *)rgc_malloc(sizeof(Point_container));
    fill_point_container(pc);
    printf("Leaking this point container and elements: ");
    print_point_container(pc);
}

void test_find_mem_leaks()
{
    printf("Test leak point container and elements\n");
    fill_point_container(&gpc_1);
    fill_point_container(&gpc_2);
    leak_point_container();
    rgc_garbage_collect_nice();

    // expected output
    printf("Expected output is 6 heap objects are leaked and cleaned "the point container and all container elements \n");
}
```

The RGC output of the test checks out and shows all that was cleaned 
```
#################### Test 8 ####################
Test leak point container and elements
...
RGC: Heap object #1 leaked 0x56066d8c6c60 and cleaned
RGC: Heap object #2 leaked 0x56066d8c6600 and cleaned
RGC: Heap object #3 leaked 0x56066d8c6820 and cleaned
RGC: Heap object #4 leaked 0x56066d8c6a40 and cleaned
RGC: Heap object #5 leaked 0x56066d8c6e80 and cleaned
RGC: Heap object #6 leaked 0x56066d8c63e0 and cleaned
RGC SUMMARY: Garbage collected 6 objects freeing 3126 bytes
```

Checking valgrind with `rgc_garbage_collect_nice()`:
```
==6303== LEAK SUMMARY:
==6303==    definitely lost: 0 bytes in 0 blocks
==6303==    indirectly lost: 0 bytes in 0 blocks
==6303==      possibly lost: 0 bytes in 0 blocks
==6303==    still reachable: 6,786 bytes in 13 blocks
==6303==         suppressed: 0 bytes in 0 blocks
```

Checking valgrind without `rgc_garbage_collect_nice()`:
```
==10938==    definitely lost: 0 bytes in 0 blocks
==10938==    indirectly lost: 0 bytes in 0 blocks
==10938==      possibly lost: 0 bytes in 0 blocks
==10938==    still reachable: 9,912 bytes in 19 blocks
==10938==         suppressed: 0 bytes in 0 blocks
```

## Conclusion & Future Work

For the final RGC C-API, please see this file [rustgc.h](https://github.com/48kaiying/crusty_gc/blob/master/c_app/src/rustgc.h)

In the end, I was able to complete the goal I had in mind even though it was not easy and not exactly how I planned. I feel comfortable writing Rust now and have a good understanding of how to build my Rust expertise which was the project objective. I also had practice writing cross platform code and exposure writing an API and documenting the API. I learned more about the Linux kernel, x86 architecture and GCC inline asm. I also learned how to manage a project myself and how to unblock myself and pivot when necessary.

RGC future work include the following:
- Optimizing the allocator
- Getting better at Rust and bypassing the Rust allocator and directly mapping pages using system calls
- Implement Rust unit testing for RGC 
- Measuring RGC's performance
- Concurrent garbage collection