OPEN := xdg-open

all: intro arrays structs book

book:
	mdbook build

intro:
	$(MAKE) -C src/introduction

arrays:
	$(MAKE) -C src/arrays

structs:
	cd src/structs/get_usage/ && cargo build

clean:
	rm -rf ./book/*
	$(MAKE) -C src/introduction clean
	$(MAKE) -C src/arrays clean
	cd src/structs/get_usage/ && cargo clean

build: 
	mdbook build
	$(OPEN) file:///`pwd`/book/index.html
	bash build.sh

.PHONY: clean build
