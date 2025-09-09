.PHONY: build-release install clean test fmt clippy help

# Default target
help:
	@echo "Available targets:"
	@echo "  build-release  Build optimized release binary"
	@echo "  install        Install the binary globally using cargo install"
	@echo "  test          Run test suite"
	@echo "  fmt           Format Rust code"
	@echo "  clippy        Run Rust linter"
	@echo "  clean         Clean build artifacts"
	@echo "  help          Show this help message"

# Build optimized release binary
build-release:
	cargo build --release

# Install the binary globally
install:
	cargo install --path .

# Run test suite
test:
	cargo test

# Format Rust code
fmt:
	cargo fmt

# Run Rust linter
clippy:
	cargo clippy

# Clean build artifacts
clean:
	cargo clean