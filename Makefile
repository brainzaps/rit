help:
	cargo run --manifest-path git_impl/Cargo.toml -- --help

helpc:
	cargo run --manifest-path git_impl/Cargo.toml -- $(COMMAND) --help

ipath:
	cargo run --manifest-path git_impl/Cargo.toml -- init --path $(REPO)