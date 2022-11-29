#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include "rustgc.h"

typedef struct Point
{
    int a;
    int b;
    char name[4];
    bool t;
} Point;

int main()
{
    printf("Sample C Application!\n");
    rust_test();
    int x = rust_test2(10);
    printf("Expected 11 = %d\n", x);
    rust_string("Hello there");

    rgc_init();

    // Test Malloc
    printf("#################### Test 1 ####################\n");
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

    printf("cptr = %s\n", cptr);

    printf("#################### Test 2 ####################\n");
    int count = 400;
    int *int_arr = (int *)(rgc_malloc(sizeof(int) * count));

    if (int_arr)
    {
        printf("Prt not null - populating\n");
        int *temp = int_arr;
        for (int i = 0; i < count; ++i)
        {
            *temp = i;
            temp++;
        }

        printf("Print results\n");
        for (int i = 0; i < count; ++i)
        {
            printf("%d\n", int_arr[i]);
        }
    }

    printf("#################### Test 3 ####################\n");
    Point *p = (Point *)(rgc_malloc(sizeof(Point)));
    p->a = 5;
    p->b = 10;
    const char name[] = "JFK";
    strcpy(p->name, (char *)&name);
    p->t = true;

    Point *p2 = p;
    printf("Point.a = %d\n", p2->a);
    printf("Point.b = %d\n", p2->b);
    printf("Point.name = %s\n", p2->name);
    printf("Point.t = %s\n", p2->t ? "true" : "false");

    // Test Free
    printf("#################### Test 4 ####################\n");

    // rgc_free((char *)int_arr);

    printf("End of c prog\n");
    return 0;
}