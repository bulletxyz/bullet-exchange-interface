.PHONY: fmt-fix

fmt-fix:
	cargo +nightly fmt --all -- --config-path rustfmt.nightly.toml

clippy-fix:
	cargo +nightly clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all-features

ci:
	make fmt-fix
	make clippy-fix
	make test