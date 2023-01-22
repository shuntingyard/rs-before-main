.PHONY: all clean

all: nodata hello entry_point

nodata: nodata.asm
	nasm -f elf64 -g -F dwarf $<
	ld $@.o -o $@

hello: hello.asm
	nasm -f elf64 -g -F dwarf $<
	ld $@.o -o $@

entry_point: entry_point.c
	gcc $< -o $@

clean:
	rm -f  entry_point hello hello.o nodata nodata.o
