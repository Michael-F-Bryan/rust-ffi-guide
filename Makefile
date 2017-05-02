OPEN := xdg-open

all: book intro arrays structs pythonic strings bindgen dynamic_loading


todo:
	grep -r --colour=auto 'TODO\|FIXME' src/ 

open: book
	$(OPEN) file:///`pwd`/book/index.html

book:
	mdbook build

intro:
	$(MAKE) -C src/introduction

arrays:
	$(MAKE) -C src/arrays

strings:
	$(MAKE) -C src/strings

dynamic_loading:
	$(MAKE) -C src/dynamic_loading

structs:
	cd src/structs/get_usage/ && cargo build

pythonic:
	cd src/pythonic/primes/ && cargo build

# This requires a hack so that we don't try to build bindgen 
# when being run by Travis (it errors)
bindgen:
	if [ -z "$(TRAVIS_BRANCH)" ]; then \
		cd src/bindgen/bzip2/ && cargo build; \
	fi


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
