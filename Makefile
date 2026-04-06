build:
	cargo build

build\:release:
	cargo build --release

# DEV PURPOSE START

elise-run:
	./target/debug/elise --mode=run --source-code=sample.eli --data=data.csv --data-schema=data.elt --print-bytecode

elise-run\:release:
	./target/debug/elise --mode=run --source-code=sample.eli --data=data.csv --data-schema=data.elt --print-bytecode

elise-build:
	./target/debug/elise --mode=build --source-code=sample.eli --data-schema=data.elt --output=sample.elc

elise-build\:release:
	./target/debug/elise --mode=build --source-code=sample.eli --data-schema=data.elt --output=sample.elc

elise-exec:
	./target/debug/elise --mode=exec --executable=sample.elc --data=data.csv

elise-exec\:release:
	./target/debug/elise --mode=exec --executable=sample.elc --data=data.csv

elise-validate:
	./target/debug/elise --mode=validate --data=data.csv --data-schema=sample.elt 

elise-validate\:release:
	./target/debug/elise --mode=validate --data=data.csv --data-schema=sample.elt 

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
