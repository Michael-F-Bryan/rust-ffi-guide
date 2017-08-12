OPEN := xdg-open
DIRS := $(shell find ./src -mindepth 1 -maxdepth 1 -type d)

# Export some default variables that all sub-makefiles should use
export OUTPUT_DIR := ${CURDIR}/target/debug
export CC := clang
export CFLAGS := -std=c99


build:
	$(MAKE) -C src/arrays build
	$(MAKE) -C src/bindgen build
	$(MAKE) -C src/callbacks build
	$(MAKE) -C src/dynamic_loading build
	$(MAKE) -C src/introduction build
	$(MAKE) -C src/pythonic build
	$(MAKE) -C src/strings build
	$(MAKE) -C src/structs build

test:
	$(MAKE) -C src/arrays test
	$(MAKE) -C src/bindgen test
	$(MAKE) -C src/callbacks test
	$(MAKE) -C src/dynamic_loading test
	$(MAKE) -C src/introduction test
	$(MAKE) -C src/pythonic test
	$(MAKE) -C src/strings test
	$(MAKE) -C src/structs test

clean:
	$(MAKE) -C src/arrays clean
	$(MAKE) -C src/bindgen clean
	$(MAKE) -C src/callbacks clean
	$(MAKE) -C src/dynamic_loading clean
	$(MAKE) -C src/introduction clean
	$(MAKE) -C src/pythonic clean
	$(MAKE) -C src/strings clean
	$(MAKE) -C src/structs clean
	cargo clean

word_count:
	@find -name '*.md' -print0 | wc --files0-from=-

todo:
	grep -r --colour=auto 'TODO\|FIXME' src/ 

open: book
	$(OPEN) file:///`pwd`/book/index.html

book:
	mdbook build

.PHONY: clean build test book todo word_count open
