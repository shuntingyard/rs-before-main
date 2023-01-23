#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include <errno.h>
#include <sys/mman.h>

#include "instructions.h"

int main() {
  printf("        main @ %p\n", &main);
  printf("instructions @ %p\n", INSTRUCTIONS);

  size_t region = (size_t)INSTRUCTIONS;
  region = region & (~0xFFF);
  printf("        page @ %p\n", region);

  printf("making page executable...\n");
  int ret = mprotect((void *)region, // addr
                     0x1000,         // len - now the size of a page (4KiB)
                     PROT_READ | PROT_EXEC // prot
  );
  if (ret != 0) {
    printf("mprotect failed: error %d\n", errno);
    return 1;
  }

  void (*f)(void) = (void *)INSTRUCTIONS;
  printf("jumping...\n");
  f();
  printf("after jump\b");
}
