
all: libaverages.a main

libaverages.a: averages.rs
	rustc --crate-type staticlib averages.rs

main: main.c libaverages.a
	clang -l dl \
	-l rt \
	-l pthread \
	-l gcc_s \
	-l c \
	-l m \
	-l util \
	-o main \
	main.c \
	libaverages.a

clean:
	$(RM) libaverages.a
	$(RM) main

.PHONY: clean
