#include <stdio.h>
#include <stdint.h>
typedef struct {
  int first;
  int second;
} TwoNumbers;
extern TwoNumbers cheese__main();
extern int64_t cheese__first(TwoNumbers);
extern int64_t cheese__second(TwoNumbers);

int main() {
  TwoNumbers tn = {0, 1};
  TwoNumbers cheese = cheese__main();
  printf("Result: {first: %ld: second: %ld}\n", cheese__first(cheese), cheese__second(cheese));
}