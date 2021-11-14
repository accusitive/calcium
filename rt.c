#include <stdio.h>
typedef struct {
  int first;
  int second;
} TwoNumbers;
extern TwoNumbers cheese__main(TwoNumbers, TwoNumbers);
int main() {
  TwoNumbers tn = {0, 1};
  TwoNumbers cheese = cheese__main(tn, tn);
  printf("Result: {first: %i: second: %i}\n", cheese.first, cheese.second);
}