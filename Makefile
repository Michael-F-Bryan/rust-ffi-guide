bucket := s3://temp.michaelfbryan.com/
chapters := "src/getting-started" "src/wrap-libmagic"

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

upload: clean book
	aws s3 sync book $(bucket) --size-only --exclude target

.PHONY: book build test