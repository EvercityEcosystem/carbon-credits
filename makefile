run-tmp:
	SKIP_WASM_BUILD= cargo run --release -- --dev --tmp

run:
	SKIP_WASM_BUILD= cargo run --release

build:
	SKIP_WASM_BUILD= cargo build --release

test:
	SKIP_WASM_BUILD= cargo test

check:
	cargo check

check-tests:
	cargo check --tests

lint:
	cargo clippy --all-targets