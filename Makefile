.PHONY: build run release clean test check fmt lint doc install help

# Default target
all: build

# Build debug version
build:
	cargo build

# Run in debug mode
run:
	cargo run

# Build release version (optimized)
release:
	cargo build --release

# Run release version
run-release:
	cargo run --release

# Run tests
test:
	cargo test

# Run tests with output
test-verbose:
	cargo test -- --nocapture

# Check code without building
check:
	cargo check

# Format code
fmt:
	cargo fmt

# Check formatting
fmt-check:
	cargo fmt --check

# Run clippy linter
lint:
	cargo clippy -- -W clippy::all

# Generate documentation
doc:
	cargo doc --open

# Clean build artifacts
clean:
	cargo clean

# Install to system
install:
	cargo install --path .

# Update dependencies
update:
	cargo update

# Show dependency tree
deps:
	cargo tree

# Build and run with debug logging
debug:
	RUST_LOG=debug cargo run

# Build and run with trace logging
trace:
	RUST_LOG=trace cargo run

# Check for security vulnerabilities
audit:
	cargo audit

# Show binary size
size: release
	@ls -lh target/release/browser

# Help
help:
	@echo "Browser Makefile Commands:"
	@echo ""
	@echo "  make build       - Build debug version"
	@echo "  make run         - Run debug version"
	@echo "  make release     - Build optimized release"
	@echo "  make run-release - Run release version"
	@echo "  make test        - Run tests"
	@echo "  make check       - Check code (no build)"
	@echo "  make fmt         - Format code"
	@echo "  make lint        - Run clippy linter"
	@echo "  make doc         - Generate documentation"
	@echo "  make clean       - Clean build artifacts"
	@echo "  make debug       - Run with debug logging"
	@echo "  make help        - Show this help"
