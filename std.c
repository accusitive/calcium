#include <stdarg.h>
#include <stdio.h>

extern void cheese__main();

int main() { cheese__main(); }
int std__print(char *format, ...) {
  va_list v;
  va_start(v, format);
  vprintf(format, v);
  va_end(v);
  return 5;
}