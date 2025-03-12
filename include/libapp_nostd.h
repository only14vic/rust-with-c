#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>

typedef char *(*foo_callback)(int32_t);

extern pthread_mutex_t MUTEX;

void log_init(void);

void foo_init(void);

char *hello_lib(int32_t a);

void *hello_lib_pthread(void *arg);

char *lib_foo_callback(int32_t a);
