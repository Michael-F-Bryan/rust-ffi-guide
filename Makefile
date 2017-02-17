
all: intro arrays book

book:
	mdbook build

intro:
	$(MAKE) -C src/introduction

arrays:
	$(MAKE) -C src/arrays

clean:
	rm -rf ./book/*
	$(MAKE) -C src/introduction clean
	$(MAKE) -C src/arrays clean

build:
	xdg-open file:///`pwd`/book/index.html
	bash build.sh

.PHONY: clean build
