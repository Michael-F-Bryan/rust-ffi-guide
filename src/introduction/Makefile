build: libhello.so main

test: build
	LD_LIBRARY_PATH=. ./main | grep 'Hello World'

libhello.so: hello.c
	$(CC) $(CFLAGS) -shared -fPIC -o libhello.so hello.c

main: libhello.so main.rs
	rustc -l hello -L . main.rs

clean:
	$(RM) libhello.so
	$(RM) main

.PHONY: clean run
