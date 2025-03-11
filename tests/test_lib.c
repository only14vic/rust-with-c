#include <stdio.h>
#include "../include/libapp_nostd.h"

void foo(foo_callback callback, int a);

int main()
{
    log_init();

    char* res = hello_lib();
    printf("%s\n", res);
    free(res);

    foo_callback callback = &lib_foo_callback;
    foo(callback, 333);

    return 0;
}

void foo(foo_callback callback, int a)
{
    char* res = (*callback)(a);
    printf("%s\n", res);
    free(res);
}
