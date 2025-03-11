#include <stdio.h>
#include <assert.h>
#include "../include/libapp_nostd.h"

void foo(foo_callback callback, int a);

int main()
{
    log_init();

    char * last_ptr = NULL;

    for (int i = 0; i < 5; i++) {
        char * ptr = hello_lib(i);

        if (!last_ptr) {
            last_ptr = ptr;
        }
        assert(last_ptr == ptr);

        printf("[%p] %s\n", ptr, ptr);

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
    printf("[%p] %s\n", ptr, ptr);
    free(ptr);
}
