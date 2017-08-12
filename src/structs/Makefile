build:
	cd get_usage && cargo build

test: build
	cd get_usage && cargo run | grep 'Usage { utime: Timeval { sec: '

clean: 
	cd get_usage && cargo clean

.PHONY: clean test

