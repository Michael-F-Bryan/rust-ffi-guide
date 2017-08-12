LIBRARIES := libadder.so


build: $(LIBRARIES) loading

test: $(LIBRARIES)
	cd loading && cargo run -- ../libadder.so | grep '1 + 2 = 3'

loading:
	cd loading && cargo build

lib%.so: %.rs
	rustc --crate-type cdylib -O $<

clean:
	$(RM) $(LIBRARIES)
	cd loading && cargo clean

.PHONY: clean test loading
