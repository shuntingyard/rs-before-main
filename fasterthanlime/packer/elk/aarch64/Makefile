.PHONY: all clean

all: hello entry_point

hello: hello.o
	ld -o $@ $<

hello.o: hello.s
	as -o $@ $<

entry_point: entry_point.c instructions.h
	gcc $< -o $@

clean:
	rm -f  entry_point hello hello.o
