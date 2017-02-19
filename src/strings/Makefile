
all: main_chars main_versions

run: all
	LD_LIBRARY_PATH=. ./main_chars
	LD_LIBRARY_PATH=. ./main_versions


main_chars: main_chars.c libchars.so
	clang -l chars -L . -o $@ $<

main_versions: main_versions.c libversions.so
	clang -l versions -L . -o $@ $<


lib%.so: %.rs
	rustc --crate-type cdylib -o $@ $<

clean:
	$(RM) *.so
	$(RM) main_versions
	$(RM) main_chars

.PHONY: clean run
