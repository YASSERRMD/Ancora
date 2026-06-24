ROOT    := $(shell git -C $(dir $(abspath $(lastword $(MAKEFILE_LIST)))) rev-parse --show-toplevel)
FFI_LIB := $(ROOT)/target/debug
BIN_DIR := $(ROOT)/target/go-bin

.PHONY: build-ffi build-release build-agent test clean

build-ffi:
	cargo build -p ancora-ffi --manifest-path $(ROOT)/Cargo.toml

# build-release builds ancora-agent with the release FFI staticlib.
build-release:
	cargo build -p ancora-ffi --release --manifest-path $(ROOT)/Cargo.toml
	mkdir -p $(BIN_DIR)
	CGO_ENABLED=1 \
	CGO_LDFLAGS="-L$(ROOT)/target/release -lancora_ffi" \
	go build -o $(BIN_DIR)/ancora-agent ./cmd/ancora-agent

# build-agent builds ancora-agent against the debug FFI staticlib.
build-agent: build-ffi
	mkdir -p $(BIN_DIR)
	CGO_ENABLED=1 \
	CGO_LDFLAGS="-L$(FFI_LIB) -lancora_ffi" \
	go build -o $(BIN_DIR)/ancora-agent-debug ./cmd/ancora-agent

test: build-ffi
	go test ./...

clean:
	go clean ./...
	rm -rf $(BIN_DIR)
