detect-cargo:
	@which cargo || echo "Cargo(Rust package manager) is not installed"

install: detect-cargo
	cargo install --path .

fmt: detect-cargo
	cargo fmt
