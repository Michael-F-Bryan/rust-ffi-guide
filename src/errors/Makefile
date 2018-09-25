CRATE := tomlreader
LIB := lib$(CRATE).so
HEADER := $(CRATE)/$(CRATE).h

test: build
	./main

build: main

main: main.c $(LIB) $(HEADER)
	$(CC) -o main main.c $(LIB) -I $(CRATE)

$(LIB): $(HEADER)
	cd $(CRATE) && cargo build $(RUST_FLAGS) && cp target/debug/$(LIB) ..

clean:
	cd $(CRATE) && cargo clean
	$(RM) $(LIB)

.PHONY: clean
