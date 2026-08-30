build\:dev:
	cargo build

build\:release:
	cargo build --release

docker\:dev:
	docker compose run --rm elise

# LOCAL DEV TEST COMMANDS START

elise\:run:
	./target/debug/elise --mode=run --source-code=sample.eli --data=sample.csv --data-schema=sample.elt --print-bytecode

elise\:run\:release:
	./target/debug/elise --mode=run --source-code=sample.eli --data=sample.csv --data-schema=sample.elt --print-bytecode

elise\:build:
	./target/debug/elise --mode=build --source-code=sample.eli --data-schema=sample.elt --output=sample.elb

elise\:build\:release:
	./target/debug/elise --mode=build --source-code=sample.eli --data-schema=sample.elt --output=sample.elb

elise\:exec:
	./target/debug/elise --mode=exec --executable=sample.elb --data=sample.csv

elise\:exec\:release:
	./target/debug/elise --mode=exec --executable=sample.elb --data=sample.csv

elise\:validate:
	./target/debug/elise --mode=validate --data=sample.csv --data-schema=sample.elt 

elise\:validate\:release:
	./target/debug/elise --mode=validate --data=sample.csv --data-schema=sample.elt 

# LOCAL DEV TEST COMMANDS END

test:
	cargo test

test\:bytecode:
	cargo test -p elise-bytecode

test\:cli:
	cargo test -p elise

test\:compiler:
	cargo test -p elise-compiler

test\:frontend\:data:
	cargo test -p elise-data

test\:frontend\:parser:
	cargo test -p elise-parser

test\:frontend\:semanalyzer:
	cargo test -p elise-semanalyzer

test\:runtime\:vm:
	cargo test -p elise-vm

check:
	cargo check

format\:check:
	cargo fmt -- --check && cargo clippy -- -D warnings

format:
	cargo fmt && cargo clippy --fix

format\:force:
	cargo fmt && cargo clippy --fix --allow-dirty

validate:
	make check && make format:check && make test

doc:
	cargo doc

doc\:open:
	cargo doc --open
