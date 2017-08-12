
build: app main

test: main
	cd app && cargo run | grep 'aborted early'
	LD_LIBRARY_PATH=. ./main 2>&1 | grep 'index out of bounds' 

app:
	cd app && cargo build

main: main.c libfoo.so
	$(CC) -L. -lfoo main.c -o main

lib%.so: %.rs
	rustc --crate-type cdylib -o $@ $<

clean:
	$(RM) main libfoo.so

.PHONY: clean
