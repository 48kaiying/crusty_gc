# Rust Garbage Collector (RGC) for C Programs 

Kaiying Guo

Advised by Norm Badler and Adam Mally 

Fall 2022 CIS 4970 Senior Capstone Design Project 

The University of Pennsylvania 

# Abstract 

Rust Garbage Collector (RGC) is a garbage collector for C programming inspired by the Boehm garbage collector. Garbage collection is form of automatic memory management which attempts to reclaim garbage -- memory that is no longer referenced -- by the program. RGC provides API call `rgc_malloc` which allocates heap memory similar to the standard `malloc` call. RGC also provides a API call `rgc_garbage_collect` which determines garbage at the current program execution point and clears the memory thereby freeing developers from having to manually release memory (calling `free` explicitly). 

Furthermore, RGC only works for **single threaded C programs running on x86-64 Linux**. The project is not meant to be a robust or wholly correct garbage collector to be used for actual garbage collection as there are bugs that have not been hashed out. It is more of an academic endeavor for myself to demystify garbage collection and an opportunity to learn Rust. 

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

#
