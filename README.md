# elise-lang

Dynamically typed, functional programming language with ugly syntax.

## TODO

- [x] Add foundation for executing binary with arguments:
    - [x] --file=<file> - specify file to execute
    - [x] --print-bytecode - print bytecode of the executed file

- [x] Add library crate which exposes interpreter entry point

- [x] Use library crate in the binary crate

- [x] Add number parsing

- [ ] Add support for scientific number notation and prevent overflow

- [ ] Create parser for parsing simple `.add` and `.declare` expression and producing AST. Tokenization is skipped because of the language syntax which is already a valid AST (Code is Data)

- [ ] Add static analysis module

- [ ] Create bytecode generator for producing bytecode from AST

- [ ] Create virtual machine for executing bytecode
