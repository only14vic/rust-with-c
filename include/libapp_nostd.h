/*
Fix implicit declaration of function ‘usleep’:
#define _XOPEN_SOURCE 500
*/


#pragma once

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <unistd.h>
#include <sched.h>


typedef char *(*foo_callback)(int32_t);

typedef struct foo_struct {
  char *foo;
} foo_struct;

extern pthread_mutex_t MUTEX;

void log_init(void);

void foo_init(void);

char *hello_lib(int32_t a);

void *hello_lib_pthread(void *arg);

char *lib_foo_callback(int32_t a);
