.PHONY: all parser wasm copy-wasm build-js example clean help

# Default target
all: parser wasm copy-wasm build-js

# Generate parser from ANTLR grammar
parser:
	@echo "=== Generating Rust parser from ANTLR grammar ==="
	@cd gadget-vm/rust-vm && $(MAKE) parser

# Compile Rust VM to WASM
wasm: parser
	@echo "=== Compiling Rust VM to WASM ==="
	@cd gadget-vm/rust-vm && $(MAKE) wasm

# Copy WASM files to js-bindings (build.sh already does this, but ensure it's done)
copy-wasm: wasm
	@echo "=== Ensuring WASM files are in js-bindings ==="
	@if [ ! -f "gadget-vm/js-bindings/wasm/gadget_vm.js" ]; then \
		echo "WASM files not found, checking build output..."; \
		if [ -f "gadget-vm/rust-vm/target/wasm32-unknown-unknown/release/gadget_vm.wasm" ]; then \
			mkdir -p gadget-vm/js-bindings/wasm && \
			cp gadget-vm/rust-vm/target/wasm32-unknown-unknown/release/gadget_vm.wasm gadget-vm/js-bindings/wasm/; \
		fi; \
	fi

# Build JavaScript bindings
build-js: copy-wasm
	@echo "=== Building JavaScript bindings ==="
	@cd gadget-vm/js-bindings && npm install && npm run build:js

# Create example (assumes build is complete)
example: build-js
	@echo "=== Example is ready at gadget-vm/examples/piano-player.html ==="

# Clean all generated files
clean:
	@echo "=== Cleaning all generated files ==="
	@cd gadget-vm/rust-vm && $(MAKE) clean
	@cd gadget-vm/js-bindings && rm -rf dist node_modules wasm
	@rm -f gadget-vm/examples/piano-player.html

# Help target
help:
	@echo "Available targets:"
	@echo "  all        - Build everything (parser, wasm, copy, js-bindings)"
	@echo "  parser     - Generate Rust parser from ANTLR grammar"
	@echo "  wasm       - Compile Rust VM to WASM"
	@echo "  copy-wasm  - Copy WASM files to js-bindings"
	@echo "  build-js   - Build JavaScript bindings"
	@echo "  example    - Create example (after build)"
	@echo "  clean      - Clean all generated files"
	@echo "  help       - Show this help message"
