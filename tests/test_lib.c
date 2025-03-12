#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "../include/libapp_nostd.h"

void foo(foo_callback callback, int a);

int main()
{
    log_init();

    volatile char * last_ptr = NULL;

    for (int i = 0; i < 5; i++) {
        volatile char * ptr = hello_lib(i);

        if (ptr == NULL) {
            fprintf(stderr, "Error: hello_lib returned NULL for i=%d\n", i);
            return EXIT_FAILURE;
        }

        if (!last_ptr) {
            last_ptr = ptr;
        }
        assert(last_ptr == ptr);

        printf("[%p] %s (strlen=%ld)\n", ptr, ptr, strlen(ptr));

        free(ptr);
    }

    foo_callback callback = &lib_foo_callback;

    for (int i = 0; i < 5; i++) {
        foo(callback, i);
    }

    // Waits for key pressing
    // getchar();

    return 0;
}

void foo(foo_callback callback, int a)
{
    char * ptr = (*callback)(a);

    if (ptr == NULL) {
        fprintf(stderr, "Error: callback returned NULL for a=%d\n", a);
        exit(1);
    }

    printf("[%p] %s (strlen=%ld)\n", ptr, ptr, strlen(ptr));

    free(ptr);
}
