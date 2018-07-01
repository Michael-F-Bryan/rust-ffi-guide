
test: build
	./arrays

build: arrays

arrays: libarray_utils.so main.rs
	rustc main.rs -o arrays -L. -larray_utils

libarray_utils.so: array_utils.c
	$(CC) $(CFLAGS) -shared array_utils.c -o $@

clean:
	$(RM) libarray_utils.so
	$(RM) arrays

.PHONY: clean