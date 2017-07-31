CC := clang


all: app main

app:
	cd app && cargo build

main: main.c libfoo.so
	$(CC) -L. -lfoo main.c -o main

run: main
	cd app && cargo run
	LD_LIBRARY_PATH=. ./main

lib%.so: %.rs
	rustc --crate-type cdylib -o $@ $<

clean:
	$(RM) main libfoo.so

.PHONY: clean