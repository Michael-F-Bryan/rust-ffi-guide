
build: libaverages.a main

test: build
	./main | grep '123.5'

libaverages.a: averages.rs
	rustc --crate-type staticlib averages.rs

main: main.c libaverages.a
		$(CC) $(CFLAGS) -l dl \
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
