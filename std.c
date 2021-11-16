#include <stdarg.h>
#include <stdio.h>

int print(char *format, ...) {
  printf("PRINT\n");
  va_list v;
  va_start(v, format);
  vprintf(format, v);
  va_end(v);
  return 5;
}
