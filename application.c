

#include <stdio.h>

// Functions called in the Rust library
extern void rust_test();

int main()
{
    printf("hello!\n");
    rust_test();
    return 0;
}