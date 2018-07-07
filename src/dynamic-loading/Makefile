MANIFEST := wordcount/Cargo.toml
LIBRARIES := -ldl

test: build
	./main libwordcount.so

build: main libwordcount.so

libwordcount.so:
	cargo build --manifest-path $(MANIFEST) 
	cp ./wordcount/target/debug/libwordcount.so .

main: plugin_manager.o main.cpp
	$(CXX) $(CXX_FLAGS) -o main main.cpp plugin_manager.o $(LIBRARIES)

plugin_manager.o: plugin_manager.h plugin_manager.cpp
	$(CXX) -c $(CXX_FLAGS) plugin_manager.cpp $(LIBRARIES)

clean:
	cargo clean --manifest-path $(MANIFEST) 
	$(RM) *.o
	$(RM) *.so
	$(RM) main

.PHONY: clean
