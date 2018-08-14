
test: build
	./basic_static
	./cargo_static/target/debug/cargo_static

build: basic_static cargo_static

basic_static: basic_static.rs libstatic.a 
	rustc $(RUST_FLAGS) $< -L. -lstatic

libstatic.a: static.c
	$(CC) $(CFLAGS) -c $< -o $@

cargo_static: main.rs cargo_static/* libstatic.a
	cp libstatic.a cargo_static/libstatic.a
	cargo build --manifest-path cargo_static/Cargo.toml

clean:
	$(RM) *.a
	$(RM) basic_static
	cd cargo_static && cargo clean

.PHONY: clean cargo_static

