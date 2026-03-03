build:
	cargo build

release-build:
	cargo build --release

# DEV PURPOSE START
run:
	./target/debug/elise --file-path=test.eli --print-bytecode

release-run:
	./target/release/elise --file-path=test.eli --print-bytecode
# DEV PURPOSE END

test:
	cargo test

check:
	cargo check
