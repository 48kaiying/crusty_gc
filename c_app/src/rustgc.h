// Functions called in the Rust library
extern void rust_test();
extern int rust_test2(int);
extern void rust_string(char *);

extern void rgc_init();
extern char *rgc_malloc(unsigned long);