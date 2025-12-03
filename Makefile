CLIPPY_FLAGS = -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used

detect-cargo:
	@which cargo || echo "Cargo(Rust package manager) is not installed"

i: install
install: detect-cargo
	cargo install --path .

f: format
format: detect-cargo
	cargo fmt

l: lint
lint: detect-cargo
	cargo clippy -- $(CLIPPY_FLAGS)

lf: lint-fix
lint-fix: detect-cargo
	cargo clippy --fix -- $(CLIPPY_FLAGS)

install-upgrade:
	cargo install cargo-edit

u: upgrade
upgrade: detect-cargo install-upgrade
	cargo upgrade --incompatible
