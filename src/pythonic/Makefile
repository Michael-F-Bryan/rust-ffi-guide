build:
	cd primes && cargo build

test: build
	py.test

clean: 
	cd primes && cargo clean
	$(RM) -r ./__pycache__

.PHONY: clean
