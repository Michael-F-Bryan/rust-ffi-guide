bucket := s3://temp.michaelfbryan.com/
chapters := getting-started arrays wrap-libmagic pod

export LD_LIBRARY_PATH := .
export CFLAGS := -g
export RUST_FLAGS := -g

build: 
	for dir in $(chapters); do \
		$(MAKE) build -C src/$$dir; \
	done

test: build
	for dir in $(chapters); do \
		$(MAKE) test -C src/$$dir; \
	done

clean: 
	$(RM) -r book
	for dir in $(chapters); do \
		$(MAKE) clean -C src/$$dir; \
	done

book:
	mdbook build

upload: clean book
	aws s3 sync book $(bucket) --size-only --exclude target

open:
	xdg-open https://s3.amazonaws.com/temp.michaelfbryan.com/getting-started/index.html >/dev/null 2>&1

.PHONY: book build test
