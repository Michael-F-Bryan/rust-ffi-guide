
test: build
	./basic
	./stateful

build: basic stateful

stateful: stateful_main.c libstateful.so
	$(CC) $(CFLAGS) $< -L. -lstateful -o stateful

libstateful.so: stateful.rs
	rustc $(RUST_FLAGS) $< --crate-type=cdylib

basic: basic_main.rs libbasic.so
	rustc $(RUST_FLAGS) $< -L. -lbasic -o $@

libbasic.so: basic.c
	$(CC) $(CFLAGS) -shared $< -o $@

clean:
	$(RM) *.so
	$(RM) basic
	$(RM) stateful

.PHONY: clean
