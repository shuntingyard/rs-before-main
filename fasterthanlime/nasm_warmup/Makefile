.PHONY: all clean

all: readfile readfile-rs getsize

readfile: readfile.asm
	nasm -f elf64 -g -F dwarf $<
	ld $@.o -o $@

readfile-rs: readfile-rs.rs
	rustc $<

getsize: getsize.c
	gcc $< -o $@

clean:
	rm -f readfile readfile.o readfile-rs getsize
