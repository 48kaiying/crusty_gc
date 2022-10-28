#include <stdio.h>
#include "rustgc.h"

typedef struct Point
{
    int a;
    int b;
    char name[4];
} Point;

int main()
{
    printf("Sample C Application!\n");
    rust_test();
    int x = rust_test2(10);
    printf("Expected 11 = %d\n", x);
    rust_string("Hello there");
    struct Point p = {8, 10, "LAX"};
    printf("Size of struct = %ld\n", sizeof(p));
    return 0;
}