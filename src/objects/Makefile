test: build
	./main

build: main

main: libmy_vec.so main.c my_vec.h
	$(CC) $(CFLAGS) main.c -L. -lmy_vec -o main

libmy_vec.so: my_vec.rs
	rustc $(RUST_FLAGS) my_vec.rs --crate-type cdylib

clean:
	$(RM) libmy_vec.so
	$(RM) main

.PHONY: clean
