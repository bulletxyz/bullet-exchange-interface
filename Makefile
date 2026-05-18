.PHONY: fmt-fix

fmt-fix:
	cargo +nightly fmt --all -- --config-path rustfmt.nightly.toml

