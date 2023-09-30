#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void no() {
  puts("Nope.");
  exit(1);
}

void ok() { puts("Good Job."); }

int main() {
  int counter = 0;
  char userInput[24] = {0};
  char comparisonString[9] = {0};

  printf("Please enter key: ");

  if (scanf("%23s", userInput) != 1) {
    no();
  }

  if (userInput[1] != '0') {
    no();
    return 1;
  }
  if (userInput[0] != '0') {
    no();
    return 1;
  }
  fflush(stdin);

  comparisonString[0] = 'd';
  int inputIndex = 2;
  int comparisonIndex = 1;

  while (1) {
    if (strlen(comparisonString) >= 8 || inputIndex >= strlen(userInput)) {
      break;
    }
    char currentInputChar = userInput[inputIndex];
    comparisonString[comparisonIndex] = atoi(&currentInputChar);
    inputIndex += 3;
    comparisonIndex++;
  }

  comparisonString[comparisonIndex] = 0;
  if (strcmp(comparisonString, "delabere") != 0) {
    no();
  }
  ok();
  return 0;
}

// original decompiled code (binaryninja)
/* int32_t main(int32_t argc, char** argv, char** envp) */
/**/
/*     int32_t var_c = 0 */
/*     printf(format: "Please enter key: ") */
/*     char var_39 */
/*     if (1 != __isoc99_scanf(format: "%23s", &var_39)) */
/*         no() */
/*         noreturn */
/*     char var_38 */
/*     if (0x30 != sx.d(var_38)) */
/*         no() */
/*         noreturn */
/*     if (0x30 != sx.d(var_39)) */
/*         no() */
/*         noreturn */
/*     fflush(fp: *stdin) */
/*     char var_21 */
/*     memset(&var_21, 0, 9) */
/*     var_21 = 0x64 */
/*     char var_3a = 0 */
/*     int32_t var_18 = 2 */
/*     int32_t var_14 = 1 */
/*     while (true) */
/*         char var_45_1 = 0 */
/*         int32_t eax_3 */
/*         if (strlen(&var_21) u< 8) */
/*             eax_3.b = var_18 u< strlen(&var_39) */
/*             var_45_1 = eax_3.b */
/*         eax_3.b = var_45_1 */
/*         if ((eax_3.b & 1) == 0) */
/*             break */
/*         int32_t eax_6 */
/*         eax_6.b = (&var_39)[var_18] */
/*         char nptr = eax_6.b */
/*         int32_t eax_7 */
/*         eax_7.b = (&var_38)[var_18] */
/*         char var_3c_1 = eax_7.b */
/*         void var_37 */
/*         int32_t eax_8 */
/*         eax_8.b = *(&var_37 + var_18) */
/*         char var_3b_1 = eax_8.b */
/*         (&var_21)[var_14] = atoi(nptr: &nptr) */
/*         var_18 = var_18 + 3 */
/*         var_14 = var_14 + 1 */
/*     (&var_21)[var_14] = 0 */
/*     if (strcmp(&var_21, "delabere") != 0) */
/*         no() */
/*         noreturn */
/*     ok() */
/*     return 0 */
