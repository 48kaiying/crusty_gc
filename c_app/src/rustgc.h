#include <stdio.h>

// Functions called in the Rust library
extern void rust_test();
extern int rust_test2(int);
extern void rust_string(char *);

extern void rgc_init();
extern void rgc_cleanup();
extern char *rgc_malloc(unsigned long);
extern void rgc_free(char *);
extern void rgc_garbage_collect(char *, char *, char *, char *);

// This is the first address past the end of the text segment (the program code).
extern char etext;

// This is the first address past the end of the initialized data segment.
extern char edata;

// This is the first address past the end of the uninitialized data segment (also known as the BSS segment).
extern char end;

static __always_inline void rgc_stack_top(long unsigned *stack_top)
{
    // Get the top of the stack by moving the ebp register value to sp
    // %%ebp contains stack frame pointer (we do not use %esp)
    __asm__ volatile("movl %ebp, %0"
                     : "=r"((unsigned int *)stack_top));
}

static __always_inline void rgc_stack_bottom(long unsigned *stack_bottom)
{
    // Stack pointer is the 28th value in linux /proc/self/stat
    // See /proc/[pid]/stat section in https://man7.org/linux/man-pages/man5/proc.5.html
    // for full list of values.
    FILE *fp = fopen("/proc/self/stat", "r");
    if (fp != NULL)
    {
        // The * in %*d indicates the field is read but not written to a variable
        fscanf(fp,
               "%*d %*s %*c %*d %*d %*d %*d %*d %*u "
               "%*lu %*lu %*lu %*lu %*lu %*lu %*ld %*ld "
               "%*ld %*ld %*ld %*ld %*llu %*lu %*ld "
               "%*lu %*lu %*lu %lu",
               stack_bottom);
    }
    else
    {
        printf("ERROR in getting stack bottom\n");
    }
    fclose(fp);
}

static void __inline__ rgc_garbage_collect_nice()
{
    long unsigned stack_top;
    long unsigned stack_bottom;
    rgc_stack_top(&stack_top);
    rgc_stack_bottom(&stack_bottom);
    rgc_garbage_collect(&etext, &end, (char *)&stack_top, (char *)&stack_bottom);
}