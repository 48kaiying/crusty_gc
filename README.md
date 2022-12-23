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

2. Writing the collector via the Mark Sweep algorithm. Garbage collection is two parts -- the mark phase which enumerates all the set of live objects and the sweep phase which removes the allocations that no longer have references to them. For the mark phase, we know all the live objects must have been allocated with RGC_malloc so all the live objects must be in the allocator linked list. Iterate over the heap (the blocks) word by word and treat each word as a pointer. If the pointer points within some allocated block in the linked list then we know there is a reference to an object in another object. Create a graph of heap references to other heap references. Next, iterate through root (user accessible) memory regions e.g. BSS, stack, global, registers and treat every word as a pointer. If the pointer points to a heap object, add a source to the node to the graph of heap references. For the sweep phase we will run DFS from every source node in the graph of heap references and note all the objects that the traversal touches -- these are non leaked objects. The remaining objects that are not touched by the DFS can’t be user accessed so they must be garbage collected (and also leaked). 

3. Lastly, I will be wrapping Rust functions with a C API using the following https://docs.rust-embedded.org/book/interoperability/rust-with-c.html

4. Include verbose logging that tells the user information about what has been garbage collected including the number of allocations and total bytes collected at the point of the call.

## Target Platform 

The project is intended only to run for single threaded c programs on x86-64 Linux. For the development of RGC, I used Windows Subsystem for Linux (WSL) - Ubuntu.

## Evaluation Criteria

Valgrind output of memory leaks running the same code with and without garbage collection. We should be able to compare the memory leak size difference and it should match the logging output of RGC. 

The following is an example: 

// TODO: add logging

// TODO: add valgrind output 

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

## Method 

### 1. The Dev Environment

The first step of the project was to create a development environment. The set up is two fold. In order to run Rust code in a C project, first a C-friendly Rust API is required. Secondly, the Rust project must be linked to by the external build system. To prioritize being able to test end to end (running Rust calls from a C project), stub Rust commands were first created for purpose of testing the C API and ensuring that the Rust project was linked correctly. First, I edited the `cargo.toml` to emit a dynamic system library `rustgc` instead of a standard Rust target. I chose GCC as the default compiler for the C application and used the linker flags to link to the library. Next I created the C API using the Rust ABI and can be viewed in `src/lib.rs`. For every function in the Rust API, a corresponding header function in C is required (see file `c_app/src/rustgc.h`). To stream line everything, I wrote a build script `build_run.sh` which compiles the Rust library and then the C application which links to said library. For source of truth on this I referred to the [Rust docs](https://docs.rust-embedded.org/book/interoperability/rust-with-c.html). 

### 2. The Naive Allocator 

Now that I was able to build a Rust project and call Rust functions from C, I began on writing the actual garbage collector. The first part of garbage collection is creating an allocator. The allocator manages allocations in containers and implements the the `rgc_malloc` call. The goal is such that a user can request memory in C in the same way they would use `malloc`. But instead of having the C program allocate memory, the request would go through the Rust allocator. The purpose of this is so we are able to keep track of every heap allocation made by the user.

Writing the allocator was the most difficult part for me and had many false starts. First, in order to create an allocator that would last the entire duration of the C project, I had to declare a global Allocator object. In Rust "global" variables are "static", however the Allocator isn't actually "static" it's mutable -- changes during execution time. This means a Rust type of `static mutable` was required. However, static mutable objects are not memory safe and the compiler through a big fuss. The solution was to use the `lazy_static` crate which allows initialization of static variable at runtime and wrapping the Allocator in a mutex since mutable static objects can not be safely passed between threads.

Now that I had an Allocator I needed to implement `rgc_malloc`. I needed to be able to declare a chunk if memory in Rust, some how store a reference to that point in memory in an internal Rust structure, and then and pass a pointer to that chunk of memory back to the C program. Initially in the project proposal, I wanted to write an allocator that used Buddy allocation, thus the internal data structure I used was a linked list of `Blocks` which stored a `vec u8` (a `vec` us a Rust heap allocated array and `u8` means unsigned 8-bit integer). It turned out this was not the right move. While implementing enqueue and dequeue to the linked list was not terrible, implementing deletion in the middle of the array was strongly discouraged in Rust as there is no real way to this safely. After fighting this for a long time, I pivoted for time sake and used a vec of Blocks and side stepped Buddy allocation algorithm to finish the project. 

Secondly, another big hurtle which I had to overcome was figuring out how to return the pointer back to C *and* store it in a Rust vec while making the compiler content. The vec memory which gets written to by the C program is referred to as the `payload` in my project. Thus, the internal data structure is a vec of vec pointers. There were two issues required resolution. First, a feature of Rust is that it de-allocates whenever a variable is no longer in scope, we do not want the `payload vec` to be deallocated after the `malloc` function finishes running. Secondly, the compiler does not allow us to store a `vec` pointer that is returned to the C program because the C program can modify the memory in unsafe ways (valid). The solution, after much research and discussing with a PhD student who knew rust was to use the Rust `Box` module which allowed me to turn the `vec` into a mutable pointer which could be returned (solving problem 2), and also calling `std::mem::forget` on said mutable pointer which tells the Rust compiler to not drop the payload memory when it goes out of scope. This process taught me a lot about Rust and also gave me a lot of pain since it felt like everything I was trying to do was not safe. At this point of the project, the progress had been slow and I wasn't sure if I would be able to achieve what I wanted using Rust.

The last part of the allocator is implementing `rgc_free` which, after deciding to simplify the implementation to a vec, I iterate over the allocated blocks and look a `payload` which matched the pointer that was passed in. Using Rust `Box` method `Box::from_raw` and passing the payload pointer in, I was able to successfully allowed Rust to de-allocate the payload memory. Calling `from_raw` is unsafe, but this is the most straight forward way to do it so I just wrapped the call in `unsafe` brackets. 

By the end of this section, I was able to allocate and de-allocate memory using the Rust allocator in C. 

### 3. Heap to Heap References 

Now that I am able to track and identify all heap allocations. 

### 4. Root Memory to Heap References


### 5. Finding Memory Leaks & Cleaning it Up

## Results

// TODO: add results

## Conclusion & Future Work

In the end, I was able to complete the goal I had in mind. 

