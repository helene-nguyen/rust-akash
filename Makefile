.PHONY: build test lint fmt check clean release help

# Default target
help:
	@echo "Usage: make <target>"
	@echo ""
	@echo "Development:"
	@echo "  build     Build in release mode"
	@echo "  test      Run tests"
	@echo "  lint      Run clippy"
	@echo "  fmt       Format code"
	@echo "  check     Run all checks (fmt, lint, test)"
	@echo "  clean     Clean build artifacts"
	@echo ""
	@echo "Release:"
	@echo "  release   Create a new release (maintainers only)"

build:
	cargo build --release

test:
	cargo test

lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all

check: fmt lint test
	@echo "✅ All checks passed"

clean:
	cargo clean

release:
	@./scripts/release.sh