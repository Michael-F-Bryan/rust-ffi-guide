test: build
	./main

build: main

libadd_points.so: add_points.rs
	rustc $(RUST_FLAGS) add_points.rs --crate-type cdylib

main: main.c libadd_points.so
	$(CC) $(CFLAGS) main.c -L. -ladd_points -o main

clean:
	$(RM) main
	$(RM) libadd_points.so

.PHONY: clean
