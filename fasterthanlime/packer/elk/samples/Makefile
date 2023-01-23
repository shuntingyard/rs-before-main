.PHONY: all clean

all: hello-pie nodata hello entry_point

hello-pie: hello-pie.asm
	nasm -f elf64 -g -F dwarf $<
	ld $@.o -o $@

nodata: nodata.asm
	nasm -f elf64 -g -F dwarf $<
	ld $@.o -o $@

hello: hello.asm
	nasm -f elf64 -g -F dwarf $<
	ld $@.o -o $@

entry_point: entry_point.c
	gcc $< -o $@

clean:
	rm -f  entry_point hello hello.o nodata nodata.o hello-pie hello-pie.o
