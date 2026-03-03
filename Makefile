.PHONY: test check fmt clippy run-example publish publish-dry help

help:
	@echo "Craken Framework Makefile"
	@echo ""
	@echo "Commands:"
	@echo "  make test         - Run workspace-wide tests"
	@echo "  make check        - Check workspace compilation"
	@echo "  make fmt          - Format workspace code"
	@echo "  make clippy       - Run clippy linting"
	@echo "  make run-example  - Run example application"
	@echo "  make publish-dry  - Run cargo publish dry-run on all crates"
	@echo "  make publish      - Publish all crates to crates.io in order"

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

publish-dry:
	@chmod +x scripts/publish.sh
	./scripts/publish.sh --dry-run

publish:
	@chmod +x scripts/publish.sh
	./scripts/publish.sh
