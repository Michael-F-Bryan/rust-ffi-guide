test: build
	./points

build: points

libadd_points.so: add_points.rs
	rustc add_points.rs --crate-type cdylib

points: main.c libadd_points.so
	$(CC)  main.c -L. -ladd_points -o points

clean:
	$(RM) points
	$(RM) libadd_points.so

.PHONY: clean
