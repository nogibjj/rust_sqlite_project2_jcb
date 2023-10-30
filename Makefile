format:
	cargo fmt --quiet

lint:
	cd rust_yahoo_finance && cargo clippy --quiet

test:
	cd rust_yahoo_finance && cargo test --quiet

run:
	cargo run 

release:
	cargo build --release

all: format lint test run
