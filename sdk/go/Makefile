ROOT := $(shell git -C $(dir $(abspath $(lastword $(MAKEFILE_LIST)))) rev-parse --show-toplevel)
FFI_LIB := $(ROOT)/target/debug

.PHONY: build-ffi test clean

build-ffi:
	cargo build -p ancora-ffi --manifest-path $(ROOT)/Cargo.toml

test: build-ffi
	go test ./...

clean:
	go clean ./...
