
test: build
	./dummy_rs
	./dummy_c

build: dummy_rs dummy_c

dummy_rs: main.rs libdummy_c.so
	rustc $(RUSTFLAGS) main.rs -L. -ldummy_c -o dummy_rs

libdummy_c.so: dummy.c
	$(CC) $(CFLAGS) -shared dummy.c -o libdummy_c.so

dummy_c: main.c libdummy_rs.so
	$(CC) $(CFLAGS) main.c -L. -ldummy_rs -o dummy_c

libdummy_rs.so: dummy.rs
	rustc $(RUSTFLAGS) dummy.rs --crate-type cdylib -o libdummy_rs.so

clean:
	$(RM) libdummy_rs.so dummy_c
	$(RM) libdummy_c.so dummy_rs

.PHONY: clean
