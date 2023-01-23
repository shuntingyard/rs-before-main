/*
 *   4000c4:        d2800f60        mov     x0, #0x7b
 *   4000c8:        d2800ba8        mov     x8, #0x5d
 *   4000cc:        d4000001        svc     #0x0
 *
 *   in memory:
 *   ** ** ** ** 60 0f 80 d2  a8 0b 80 d2 01 00 00 d4  |****`...........|
 */
const char *INSTRUCTIONS = "\x60\x0f\x80\xd2\xa8\x0b\x80\xd2\x01\x00\x00\xd4";
