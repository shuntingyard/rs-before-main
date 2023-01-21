.PHONY: all clean

all: hello entry_point

hello: hello.asm
	nasm -f elf64 -g -F dwarf $<
	ld $@.o -o $@

entry_point: entry_point.c
	gcc $< -o $@

clean:
	rm -f  entry_point hello hello.o
