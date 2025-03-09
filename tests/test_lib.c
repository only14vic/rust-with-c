#include <stdio.h>
#include "../libapp_nostd.h"

void foo(foo_callback callback, int a);

int main()
{
    const char* res = hello_lib();
    printf("%s\n", res);

    foo_callback callback = &lib_foo_callback;
    foo(callback, 333);

    return 0;
}

void foo(foo_callback callback, int a)
{
    const char* res = (*callback)(a);
    printf("Callback: %s\n", res);
}
