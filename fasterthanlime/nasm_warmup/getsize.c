#include <stddef.h>
#include <stdio.h>
#include <sys/mman.h>
#include <sys/stat.h>

int main() {
  printf("size of stat struct: %zu\n", sizeof(struct stat));
  printf("offset of st_size  : %zu\n", offsetof(struct stat, st_size));
  printf("PROT_READ   = 0x%x\n", PROT_READ);
  printf("MAP_PRIVATE = 0x%x\n", MAP_PRIVATE);
}
