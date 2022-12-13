#include <stdio.h>
#include <stdbool.h>
#include <string.h>
#include "rustgc.h"

// This is the first address past the end of the text segment (the program code).
extern char etext;

// This is the first address past the end of the initialized data segment.
extern char edata;

// This is the first address past the end of the uninitialized data segment (also known as the BSS segment).
extern char end;

typedef struct Stack_elt
{
    int value;
    struct Stack_elt *next;
} Stack_elt;

typedef struct Stack
{
    struct Stack_elt *head;
} Stack;

void stack_add(Stack *s, int v)
{
    Stack_elt *elem = (Stack_elt *)rgc_malloc(sizeof(Stack_elt));
    elem->value = v;

    if (s->head == NULL)
    {
        s->head = elem;
    }
    else
    {
        elem->next = s->head;
        s->head = elem;
    }
}

void stack_iterate(const Stack *s)
{
    // Read values out
    if (s->head == NULL)
    {
        printf("Stack empty\n");
        return;
    }

    int i = 1;
    for (Stack_elt *t = s->head; t != NULL; t = t->next)
    {
        printf("Stack Item #%d : %d\n", i, t->value);
        ++i;
    }
    printf("Stack Bottom\n");
}

void unused_test_heap_graph_stack()
{
    printf("Test heap graph stack\n");
    Stack *s = (Stack *)rgc_malloc(sizeof(Stack));
    s->head = NULL;
    stack_add(s, 1);
    stack_add(s, 2);
    stack_add(s, 3);
    stack_iterate(s);
    printf("Stack object in C:  %p\n", s);
    rgc_garbage_collect(&etext, &end);
}

typedef struct Point
{
    int a;
    int b;
    char name[4];
    bool t;
} Point;

typedef struct Point_container
{
    struct Point *first;
    struct Point *sec;
    struct Point *third;
    struct Point *fourth;
    struct Point *fifth;
} Point_container;

void fill_point_container(Point_container *pc)
{
    if (pc == NULL)
    {
        return;
    }

    Point *first = (Point *)rgc_malloc(sizeof(Point));
    first->a = 1;
    first->b = 2;
    const char name[] = "JFK";
    strcpy(first->name, (char *)&name);
    first->t = false;
    pc->first = first;

    Point *sec = (Point *)rgc_malloc(sizeof(Point));
    sec->a = 2;
    sec->b = 3;
    const char name1[] = "PHL";
    strcpy(sec->name, (char *)&name1);
    sec->t = false;
    pc->sec = sec;

    Point *tri = (Point *)rgc_malloc(sizeof(Point));
    tri->a = 4;
    tri->b = 5;
    const char name2[] = "ALT";
    strcpy(tri->name, (char *)&name2);
    tri->t = true;
    pc->third = tri;

    Point *four = (Point *)rgc_malloc(sizeof(Point));
    four->a = 6;
    four->b = 7;
    const char name3[] = "SJO";
    strcpy(four->name, (char *)&name3);
    four->t = true;
    pc->fourth = four;

    Point *fifth = (Point *)rgc_malloc(sizeof(Point));
    fifth->a = 8;
    fifth->b = 9;
    const char name4[] = "SEA";
    strcpy(fifth->name, (char *)&name4);
    fifth->t = true;
    pc->fifth = fifth;
}

void print_point_container(Point_container *pc)
{
    printf("Object point container %p contains 5 references: %p, %p, %p, %p, %p\n",
           pc,
           pc->first,
           pc->sec,
           pc->third,
           pc->fourth,
           pc->fifth);
}

void test_heap_graph()
{
    printf("Test heap graph point container\n");
    Point_container *pc = (Point_container *)rgc_malloc(sizeof(Point_container));
    fill_point_container(pc);
    rgc_garbage_collect(&etext, &end);

    // expected output
    printf("Expected output from c:\n");
    print_point_container(pc);
}

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
    rgc_garbage_collect(&etext, &end);
    printf("Expected output is a new heap object %p with 5 references:\n", &gpc_1);
    printf("Global gpc_1:\n");
    print_point_container(&gpc_1);
}

void print_root_mem_regions()
{
    printf("First address past:\n");
    printf("    program text (etext)      %10p\n", &etext);
    printf("    initialized data (edata)  %10p\n", &edata);
    printf("    uninitialized data (end)  %10p\n", &end);
}

// #def NOT_TESTING

int main()
{
    printf("Sample C Application!\n");
    // rust_test();
    // int x = rust_test2(10);
    // printf("Expected 11 = %d\n", x);
    // rust_string("Hello there");

    rgc_init();
    print_root_mem_regions();

#ifdef NOT_TESTING
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

    rgc_free((char *)int_arr);
    rgc_free((char *)p);
    rgc_free((char *)cptr);

    rgc_free((char *)(++int_arr));
#endif

    // Test points to something else
    // printf("#################### Test 5 ####################\n");
    // test_heap_graph();

    printf("#################### Test 6 ####################\n");
    test_scan_data_region();

    printf("Cleaning RGC\n");
    // Clean up
    rgc_cleanup();

    printf("End of c prog\n");
    return 0;
}