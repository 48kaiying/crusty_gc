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
    printf("Size of struct = %lu\n", sizeof(p));
    rgc_init();

    char *cptr;
    char test[] = {'d', 'u', 'c', 'k', 'y', '\0'};
    printf("Size of test = %lu\n", sizeof(test));

    cptr = rgc_malloc(sizeof(test));

    printf("Value:  %p\n", cptr);

    char *temp = cptr;
    for (int i = 0; i < 8; ++i)
    {
        *temp = test[i];
        temp++;
    }

    rgc_free(cptr);

    printf("cptr = %s\n", cptr);

    printf("End of c prog\n");
    return 0;
}