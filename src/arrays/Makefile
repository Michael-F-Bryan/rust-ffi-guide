
test: build
	./arrays
	./more_arrays

build: arrays more_arrays

librusty_array_utils.so: rusty_array_utils.rs
	rustc $(RUST_FLAGS) rusty_array_utils.rs --crate-type cdylib

more_arrays: main.c librusty_array_utils.so
	$(CC) $(CFLAGS) -o more_arrays main.c -L. -lrusty_array_utils

arrays: libarray_utils.so main.rs
	rustc $(RUST_FLAGS) main.rs -o arrays -L. -larray_utils

libarray_utils.so: array_utils.c
	$(CC) $(CFLAGS) -shared array_utils.c -o $@

clean:
	$(RM) *.so
	$(RM) arrays
	$(RM) more_arrays

.PHONY: clean
