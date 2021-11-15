#include <stdarg.h>
#include <stdio.h>

int std__print(char *format, ...) {
  va_list v;
  va_start(v, format);
  vprintf(format, v);
  va_end(v);
  return 5;
}
