CLIPPY_FLAGS = -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used

detect-cargo:
	@which cargo || echo "Cargo(Rust package manager) is not installed"

install: detect-cargo
	cargo install --path .

fmt: detect-cargo
	cargo fmt

lint: detect-cargo
	cargo clippy -- $(CLIPPY_FLAGS)

lint-fix: detect-cargo
	cargo clippy --fix -- $(CLIPPY_FLAGS)
