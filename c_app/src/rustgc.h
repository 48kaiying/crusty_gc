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

static __always_inline unsigned long rgc_stack_top()
{
    // Get the top of the stack by moving the ebp register value to sp
    // %%rsp is stack pointer and %%rbp is stack frame pointer

    // 64-bit system use %%rbp and movq (move quad word aka 8 bytes)
    // 32-bit system use %%ebp use movl (move double word aka 4 bytes)

    // Linux is at&t syntax so OPCODE SRC DEST
    // The `=` in `=r` means that the operand is an output.
    // The `r` is an operand contraint and says the operand may be in a register
    // as long as it is in a general register.

    // GCC inline asm: https://gcc.gnu.org/onlinedocs/gcc-4.4.7/gcc/Extended-Asm.html
    // Constraints https://gcc.gnu.org/onlinedocs/gcc-4.4.7/gcc/Simple-Constraints.html#Simple-Constraints

    unsigned long stack_top = 0;
    asm volatile("movq %%rbp, %0"
                 : "=r"(stack_top));
    return stack_top;
}

static __always_inline void rgc_stack_bottom(unsigned long *stack_bottom)
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

static __attribute_noinline__ void rgc_garbage_collect_nice()
{
    // Size of long is 8 bytes
    unsigned long stack_bottom = 0;
    unsigned long stack_top = rgc_stack_top();
    rgc_stack_bottom(&stack_bottom);
    printf("STACK TOP IS %p\n", (char *)stack_top);
    printf("STACK BOTTOM IS %p\n", (char *)stack_bottom);
    rgc_garbage_collect(&etext, &end, (char *)stack_top, (char *)stack_bottom);
}