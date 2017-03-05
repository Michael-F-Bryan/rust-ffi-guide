OPEN := xdg-open

all: book intro arrays structs pythonic strings bindgen

todo:
	grep -r --colour=auto 'TODO\|FIXME' src/ 

open:book
	$(OPEN) file:///`pwd`/book/index.html

book:
	mdbook build

intro:
	$(MAKE) -C src/introduction

arrays:
	$(MAKE) -C src/arrays

strings:
	$(MAKE) -C src/strings

structs:
	cd src/structs/get_usage/ && cargo build

pythonic:
	cd src/pythonic/primes/ && cargo build

bindgen:
	cd src/bindgen/bzip2/ && cargo build

clean:
	rm -rf ./book/*
	$(MAKE) -C src/introduction clean
	$(MAKE) -C src/arrays clean
	cd src/structs/get_usage/ && cargo clean
	cd src/pythonic/primes/ && cargo clean
	cd src/bindgen/bzip2/ && cargo clean

build: open
	bash build.sh

.PHONY: clean build book todo
