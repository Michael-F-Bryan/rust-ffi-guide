bucket := s3://temp.michaelfbryan.com/
chapters := getting-started arrays wrap-libmagic pod objects dynamic-loading \
	        callbacks linking errors

export LD_LIBRARY_PATH := .:$(shell rustc --print sysroot)
export CFLAGS := -std=c11 -Wall -fPIC
export CXX_FLAGS := -std=c++11 -Wall
export RUST_FLAGS := 

test build clean: $(chapters)

test: TARGET=test
build: TARGET=build
clean: TARGET=clean

clean:
	$(RM) -r book

$(chapters): _force
	@ $(MAKE) -C src/$@ $(TARGET)

book:
	mdbook build

upload: clean book
	aws s3 sync book/html $(bucket) --size-only --exclude target

open:
	xdg-open https://s3.amazonaws.com/temp.michaelfbryan.com/getting-started/index.html >/dev/null 2>&1

_force:

.PHONY: book build test _force
