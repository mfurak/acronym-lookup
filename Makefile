detect-cargo:
	@which cargo || echo "Cargo(Rust package manager) is not installed"

install: detect-cargo
	cargo install --path .

fmt: detect-cargo
	cargo fmt

lint: detect-cargo
	cargo clippy -- \
	-W clippy::pedantic \
	-W clippy::nursery \
	-W clippy::unwrap_used \
	-W clippy::expect_used

lint-fix: detect-cargo
	cargo clippy --fix -- \
	-W clippy::pedantic \
	-W clippy::nursery \
	-W clippy::unwrap_used \
	-W clippy::expect_used
