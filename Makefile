build:
	cargo build

build\:release:
	cargo build --release

# DEV PURPOSE START
run:
	./target/debug/elise --file-path=sample.eli --print-bytecode

run\:release:
	./target/release/elise --file-path=sample.eli --print-bytecode
# DEV PURPOSE END

test:
	cargo test

test\:parser:
	cargo test -p elise-parser

check:
	cargo check

fmt\:check:
	cargo fmt -- --check

fmt:
	cargo fmt
