# Name of your Rust binary
BIN_NAME = data_processing

# Default target
default: build

# Install the project dependencies (Cargo handles this automatically)
install:
	cargo build --release

# Build the project
build:
	cargo build --release

# Run the project
run:
	./target/release/$(BIN_NAME)

# Lint the code (using clippy)
lint:
	cargo clippy -- -D warnings

# Format the code
format:
	cargo fmt --all -- --check

# Test the project (if you have tests written)
test:
	cargo test

# Clean up the build
clean:
	cargo clean