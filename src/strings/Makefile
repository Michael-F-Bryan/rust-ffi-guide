
build: main_chars main_versions

test: build
	LD_LIBRARY_PATH=. ./main_chars | grep '12 characters'
	LD_LIBRARY_PATH=. ./main_versions | grep '0.1.0'


main_chars: main_chars.c libchars.so
	$(CC) $(CFLAGS) -l chars -L . -o $@ $<

main_versions: main_versions.c libversions.so
	$(CC) $(CFLAGS) -l versions -L . -o $@ $<


lib%.so: %.rs
	rustc --crate-type cdylib -o $@ $<

clean:
	$(RM) *.so
	$(RM) main_versions
	$(RM) main_chars

.PHONY: clean test
