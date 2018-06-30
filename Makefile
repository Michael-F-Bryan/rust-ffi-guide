chapters := "src/the-basics"

build: book
	for dir in $(chapters); do \
		$(MAKE) build -C $$dir; \
	done

test: build
	for dir in $(chapters); do \
		$(MAKE) test -C $$dir; \
	done

clean: 
	mdbook clean
	for dir in $(chapters); do \
		$(MAKE) clean -C $$dir; \
	done

book:
	mdbook build

.PHONY: book build test