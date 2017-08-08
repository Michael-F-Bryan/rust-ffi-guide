

all: libhello.so main


libhello.so: hello.c
	clang -shared -fPIC -o libhello.so hello.c

main: libhello.so main.rs
	rustc -l hello -L . main.rs

clean:
	$(RM) libhello.so
	$(RM) main

run: all
	LD_LIBRARY_PATH=. ./main


.PHONY: clean run
