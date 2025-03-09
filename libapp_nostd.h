#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef const char *(*foo_callback)(int32_t);

const char *hello_lib(void);

const char *lib_foo_callback(int32_t a);
