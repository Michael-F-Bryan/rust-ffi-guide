RUST_FLAGS := -O
CFLAGS := -O2

test: build
	./main

build: main

main: libmy_vec.so main.c my_vec.h
	$(CC) $(CFLAGS) main.c -L. -lmy_vec -o main

libmy_vec.so: my_vec.rs
	rustc my_vec.rs $(RUST_FLAGS) --crate-type cdylib

clean:
	$(RM) libmy_vec.so
	$(RM) main

.PHONY: clean
