CFLAGS := $(CFLAGS) -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lpthread -lutil -lutil

test: build
	./hello_world
	./print

build: hello_world print

print: libprint.a main.c 
	$(CC) $(CFLAGS) -o print main.c libprint.a 

libprint.a: print.rs
	rustc $(RUST_FLAGS) print.rs --crate-type staticlib

hello_world: hello_world.rs
	rustc $(RUST_FLAGS) hello_world.rs -o hello_world

clean:
	$(RM) hello_world
	$(RM) libprint.a
	$(RM) print

.PHONY: test clean
