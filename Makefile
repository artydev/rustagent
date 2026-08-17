# RustAgent — local development commands
#
# Common targets:
#   make build    — compile the project (debug)
#   make run      — build and launch the app
#   make test     — run the full test suite
#   make lint     — run clippy with warnings-as-errors
#   make fmt      — auto-format the code
#   make fmt-check— verify formatting without modifying files
#   make check    — run fmt-check + lint + test (mirrors CI)
#   make release  — build an optimized release binary
#   make clean    — remove build artifacts

.PHONY: build run test lint fmt fmt-check check release clean

# Compile the project in debug mode.
build:
	cargo build

# Build and launch the application.
run:
	cargo run

# Run the full test suite.
test:
	cargo test

# Run clippy with warnings treated as errors (matches CI).
lint:
	cargo clippy --all-targets --all-features -- -D warnings

# Auto-format the codebase.
fmt:
	cargo fmt --all

# Verify formatting without modifying files (matches CI).
fmt-check:
	cargo fmt --all -- --check

# Run the same checks as the CI pipeline: format, lint, then test.
check: fmt-check lint test

# Build an optimized release binary.
release:
	cargo build --release

# Remove build artifacts.
clean:
	cargo clean
