/* example.i */
%module libenc
%{
/* Put header files here or function declarations like below */
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
extern const char *c_hello_world(void);
extern void c_hello_world_free(char *ptr);
extern int c_meaning_of_life(void);
%}
extern const char *c_hello_world(void);
extern void c_hello_world_free(char *ptr);
extern int c_meaning_of_life(void);
