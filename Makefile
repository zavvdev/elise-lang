build:
	cargo build

release-build:
	cargo build --release

# DEV PURPOSE START
run:
	./target/debug/elise-lang --file=sample.eli --print-bytecode

release-run:
	./target/release/elise-lang --file=sample.eli --print-bytecode
# DEV PURPOSE END

test:
	cargo test

check:
	cargo check
