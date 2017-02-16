
all: intro arrays


intro:
	$(MAKE) -C src/introduction

arrays:
	$(MAKE) -C src/arrays

clean:
	$(MAKE) -C src/introduction clean
	$(MAKE) -C src/arrays clean

.PHONY: clean
