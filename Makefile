release:
	cargo build -r

run-release:
	cargo run -r sample.el

test:
	cargo test

check:
	cargo check

run:
	cargo run sample.el
