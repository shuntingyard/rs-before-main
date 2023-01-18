          global _start           ; start is our entry point - this is the declaration...

          section .text           ; this is where we'll put our executable code
_start:   mov     rax, 2          ; "open" syscall
          mov     rdi, path       ; arg1: path
          xor     rsi, rsi        ; arg2: flags (0 = O_RDONLY)
          syscall

          mov     rdi, rax        ; fd (returned from open)
          sub     rsp, 144        ; allocate stat struct (we know 144 from getsize.c)
          mov     rsi, rsp        ; address of 'struct stat'
          mov     rax, 5          ; "fstat" syscall
          syscall

          mov     rsi, [rsp+48]   ; len = file size (from 'struct stat')
          add     rsp, 144        ; free 'struct stat'
          mov     r8, rdi         ; fd (still in rdi from last syscall)
          xor     rdi, rdi        ; address = 0
          mov     rdx, 0x1        ; protection = PROT_READ
          mov     r10, 0x2        ; flags = MAP_PRIVATE
          xor     r9, r9          ; offset = 0
          mov     rax, 9          ; "mmap" syscall
          syscall

          mov     rdx, rsi        ; count (file size from last call)
          mov     rsi, rax        ; buffer address (returned from mmap)
          mov     rdi, 1          ; fd = stdout
          mov     rax, 1          ; "write" syscall
          syscall

          mov     rax, 60         ; "exit" syscall
          syscall

          section .data
path:     db      "/etc/hosts", 0 ; null-terminated
