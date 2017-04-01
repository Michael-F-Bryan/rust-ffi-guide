LIBRARIES := libadder.so


all: $(LIBRARIES) loading

run: $(LIBRARIES)
	cd loading && cargo run -- ../libadder.so

loading:
	cd loading && cargo build

lib%.so: %.rs
	rustc --crate-type cdylib -O $<

clean:
	$(RM) $(LIBRARIES)
	cd loading && cargo clean

.PHONY: clean loading
