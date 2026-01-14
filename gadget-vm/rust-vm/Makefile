.PHONY: all clean parser wasm test check-java

# Configuration
# Note: Standard ANTLR doesn't support Rust. We use the Rust-compatible version from antlr4rust
# See: https://github.com/rrevenantt/antlr4rust
ANTLR_JAR := antlr4-4.8-2-SNAPSHOT-complete.jar
GRAMMAR_DIR := ../grammar
GRAMMAR_FILE := $(GRAMMAR_DIR)/GadgetLanguage.g4
PARSER_DIR := src/parser
WASM_TARGET := wasm32-unknown-unknown

all: parser wasm

# Check if Java is available, install if missing on macOS
check-java:
	@if ! command -v java >/dev/null 2>&1 || ! java -version >/dev/null 2>&1; then \
		echo "Java not found. Checking system..."; \
		if [ "$$(uname)" = "Darwin" ]; then \
			echo "Detected macOS. Attempting to install Java via Homebrew..."; \
			if command -v brew >/dev/null 2>&1; then \
				echo "Installing openjdk via Homebrew (this may take a few minutes)..."; \
				brew install openjdk || { \
					echo ""; \
					echo "Homebrew installation failed. Please install Java manually:"; \
					echo "  brew install openjdk"; \
					echo ""; \
					echo "Or download from: https://www.java.com/download/"; \
					exit 1; \
				}; \
				echo "Java installed successfully!"; \
				echo "Setting up Java environment..."; \
				if [ -d "/opt/homebrew/opt/openjdk" ]; then \
					echo "Found Java at /opt/homebrew/opt/openjdk"; \
					echo "You may need to add to your shell profile:"; \
					echo "  export JAVA_HOME=\"/opt/homebrew/opt/openjdk\""; \
					echo "  export PATH=\"\$$JAVA_HOME/bin:\$$PATH\""; \
				elif [ -d "/usr/local/opt/openjdk" ]; then \
					echo "Found Java at /usr/local/opt/openjdk"; \
					echo "You may need to add to your shell profile:"; \
					echo "  export JAVA_HOME=\"/usr/local/opt/openjdk\""; \
					echo "  export PATH=\"\$$JAVA_HOME/bin:\$$PATH\""; \
				fi; \
				echo ""; \
				echo "Trying to use Java from Homebrew location..."; \
				if [ -f "/opt/homebrew/opt/openjdk/bin/java" ]; then \
					/opt/homebrew/opt/openjdk/bin/java -version 2>&1 | head -1 || true; \
				elif [ -f "/usr/local/opt/openjdk/bin/java" ]; then \
					/usr/local/opt/openjdk/bin/java -version 2>&1 | head -1 || true; \
				fi; \
			else \
				echo ""; \
				echo "Homebrew not found. Please install Java manually:"; \
				echo "  1. Install Homebrew: /bin/bash -c \"\$$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""; \
				echo "  2. Then run: brew install openjdk"; \
				echo ""; \
				echo "Or download Java from: https://www.java.com/download/"; \
				exit 1; \
			fi; \
		else \
			echo ""; \
			echo "Error: Java is required to generate the parser."; \
			echo "ANTLR's code generator is Java-based."; \
			echo ""; \
			echo "Please install Java for your system:"; \
			echo "  - macOS: brew install openjdk"; \
			echo "  - Linux: sudo apt-get install default-jdk (Debian/Ubuntu)"; \
			echo "  - Or download from: https://www.java.com/download/"; \
			exit 1; \
		fi; \
	else \
		echo "Java is available: $$(java -version 2>&1 | head -1)"; \
	fi

# Download ANTLR JAR with Rust support if not present
$(ANTLR_JAR): check-java
	@if [ ! -f "$(ANTLR_JAR)" ]; then \
		echo "ANTLR JAR with Rust support not found."; \
		echo ""; \
		echo "Standard ANTLR doesn't support Rust code generation."; \
		echo "This project requires the Rust-compatible version from antlr4rust."; \
		echo ""; \
		echo "Please download the JAR file manually:"; \
		echo "  1. Visit: https://github.com/rrevenantt/antlr4rust/releases"; \
		echo "  2. Look for the SNAPSHOT release or 'antlr4-4.8-2-Rust-0.2' release"; \
		echo "  3. Download the JAR file (antlr4-4.8-2-SNAPSHOT-complete.jar) from the Assets section"; \
		echo "  4. Save it as: $(ANTLR_JAR)"; \
		echo ""; \
		echo "Automatic download is not available - please download manually from:"; \
		echo "  https://github.com/rrevenantt/antlr4rust/releases"; \
		exit 1; \
	fi

# Generate Rust parser from ANTLR grammar
parser: check-java $(ANTLR_JAR) $(GRAMMAR_FILE)
	@echo "Generating Rust parser from ANTLR grammar..."
	@mkdir -p $(PARSER_DIR)
	@JAVA_CMD=$$(if command -v java >/dev/null 2>&1 && java -version >/dev/null 2>&1; then \
		echo "java"; \
	elif [ -f "/opt/homebrew/opt/openjdk/bin/java" ]; then \
		echo "/opt/homebrew/opt/openjdk/bin/java"; \
	elif [ -f "/usr/local/opt/openjdk/bin/java" ]; then \
		echo "/usr/local/opt/openjdk/bin/java"; \
	else \
		echo ""; \
	fi); \
	if [ -z "$$JAVA_CMD" ]; then \
		echo "Error: Could not find Java executable."; \
		echo "Please ensure Java is installed and in your PATH."; \
		exit 1; \
	fi; \
	echo "Using Java: $$JAVA_CMD"; \
	$$JAVA_CMD -jar $(ANTLR_JAR) -Dlanguage=Rust -visitor -o $(PARSER_DIR) $(GRAMMAR_FILE)
	@echo "Parser generated in $(PARSER_DIR)"

# Compile Rust code to WASM
wasm: parser
	@echo "Compiling Rust to WASM..."
	@./build.sh
	@echo "WASM build complete: target/$(WASM_TARGET)/release/gadget_vm.wasm"

# Run tests
test:
	@cargo test

# Clean generated files
clean:
	@echo "Cleaning generated files..."
	@rm -rf $(PARSER_DIR)
	@cargo clean

# Clean everything including ANTLR JAR (optional)
clean-all: clean
	@echo "Cleaning ANTLR JAR..."
	@rm -f $(ANTLR_JAR)
