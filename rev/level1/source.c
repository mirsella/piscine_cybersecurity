#include <stdio.h>
#include <string.h>
int main(void) {
  char pass[] = "__stack_check";
  char input[100];
  printf("Please enter key: ");
  scanf("%s", input);
  if (strcmp(pass, input) == 0)
    printf("Good Job.\n");
  else
    printf("Nope.\n");
}
