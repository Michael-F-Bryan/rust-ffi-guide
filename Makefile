bucket := s3://temp.michaelfbryan.com/
chapters := getting-started arrays wrap-libmagic pod

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

.PHONY: book build test
