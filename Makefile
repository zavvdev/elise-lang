build:
	cargo build

release-build:
	cargo build --release

# DEV PURPOSE START
run:
	./target/debug/elise --file-path=sample.eli --print-bytecode

release-run:
	./target/release/elise --file-path=sample.eli --print-bytecode
# DEV PURPOSE END

test:
	cargo test

test\:parser:
	cargo test -p elise-parser

check:
	cargo check

fmt-check:
	cargo fmt -- --check

fmt:
	cargo fmt
