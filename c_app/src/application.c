#include <stdio.h>
#include "rustgc.h"

int main()
{
    printf("Sample C Application!\n");
    rust_test();
    int x = rust_test2(10);
    printf("Expected 11 = %d\n", x);
    rust_string("Hello there");
    return 0;
}