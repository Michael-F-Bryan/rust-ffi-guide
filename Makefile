OPEN := xdg-open
DIRS := $(shell find ./src -maxdepth 1 -type d)
CC := clang


define run-in-sub-dirs
	for sub_dir in $(DIRS); do \
		$(MAKE) -C $$sub_dir $1; \
	done
endef


build:
	$(call run-in-sub-dirs,build)

test:
	$(call run-in-sub-dirs,test)

clean:
	$(call run-in-sub-dirs,clean)
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
