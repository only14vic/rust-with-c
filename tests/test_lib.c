#define _XOPEN_SOURCE 500

#include <assert.h>
#include <malloc.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <sched.h>
#include <json-c/json.h>
#include <json-c/json_object.h>
#include "../include/libapp_nostd.h"

void foo(foo_callback callback, int a);

int main() {
    log_init();
    foo_init();

    char *last_ptr = NULL;

    for (int i = 0; i < 5; i++) {
        // disable compiler optimization with "volatile"
        volatile char *ptr = hello_lib(i);

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

    char *value = (char *)malloc(100 * sizeof(char));
    strcpy(value, "Data from Main.");

    pthread_t thread;
    pthread_create(&thread, NULL, &hello_lib_pthread, value);

    for (int i = 0; i < 5; i++) {
        pthread_mutex_lock(&MUTEX);
        printf("Main thread\n");
        pthread_mutex_unlock(&MUTEX);

        sched_yield();
        usleep(1);
    }

    pthread_join(thread, (void **)&value);

    printf("Thread value: %s\n", value);
    free(value);

    foo_callback callback = &lib_foo_callback;

    for (int i = 0; i < 5; i++) {
        foo(callback, i);
    }

    json_object *json = json_object_new_object();
    json_object_object_add(json, "hello", json_object_new_string("World!"));
    json_object_object_add(json, "foo", json_object_new_int(123));
    json_object_object_add(json, "bar", json_object_new_object());

    printf(
        "JSON: %s\n",
        json_object_to_json_string_ext(json, JSON_C_TO_STRING_PRETTY)
    );

    json_object_put(json);

    foo_struct *foo = foo_create("Hello", NULL);
    printf("FooStruct { foo: '%s', bar: '%s' }\n", foo->foo, foo->bar);
    foo_drop(foo);

    malloc_stats();

    // Waits for key pressing
    // getchar();

    return 0;
}

void foo(foo_callback callback, int a) {
    // enable compiler optimization with "restrict"
    char *restrict ptr = (*callback)(a);

    if (ptr == NULL) {
        fprintf(stderr, "Error: callback returned NULL for a=%d\n", a);
        exit(1);
    }

    printf("[%p] %s (strlen=%ld)\n", ptr, ptr, strlen(ptr));

    free(ptr);
}
