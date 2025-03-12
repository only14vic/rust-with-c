#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef char *(*foo_callback)(int32_t);

void log_init(void);

char *hello_lib(int32_t a);

void *hello_lib_pthread(void *arg);

char *lib_foo_callback(int32_t a);
