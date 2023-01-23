/*
 *   40101b:        48 31 ff            xor     %rdi,%rdi
 *   40101e:        b8 3c 00 00 00      mov     $0x3c,%eax
 *   401023:        0f 05               syscall
 */
const char *INSTRUCTIONS = "\x48\x31\xFF\xB8\x3C\x00\x00\x00\x0F\x05";
