.PHONY: test check fmt clippy run-example help

help:
	@echo "Craken Framework Makefile"
	@echo ""
	@echo "Commands:"
	@echo "  make test         - Run workspace-wide tests"
	@echo "  make check        - Check workspace compilation"
	@echo "  make fmt          - Format workspace code"
	@echo "  make clippy       - Run clippy linting"
	@echo "  make run-example  - Run example application"

test:
	cargo test --workspace

check:
	cargo check --workspace

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace -- -D warnings

run-example:
	cargo run -p craken-app -- serve
