
test: build
	./basic_add
	cd my_real_project && cargo run

build: basic_add my_real_project

basic_add: basic_add.rs libadd.a 
	rustc $(RUST_FLAGS) $< -L. -ladd

libadd.a: add.c
	$(CC) $(CFLAGS) -c $< -o $@

my_real_project: basic_add.rs my_real_project/* libadd.a
	cp libadd.a my_real_project/libadd.a
	cargo build --manifest-path my_real_project/Cargo.toml

clean:
	$(RM) *.so *.a
	$(RM) my_real_project/*.a my_real_project/*.so
	$(RM) basic_add
	cd my_real_project && cargo clean

.PHONY: clean my_real_project

