# This requires a hack so that we don't try to build bindgen 
# when being run by Travis (it errors)
ifdef TRAVIS_BRANCH

build:
test:
clean:

else

build:
	cd bzip2 && cargo build

test:
	cd bzip2 && cargo test

clean:
	cd bzip2 && cargo clean

endif

.PHONY: build test clean
