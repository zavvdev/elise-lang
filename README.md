# elise-lang

Dynamically typed, functional programming language with ugly syntax.

## TODO

- [x] Add foundation for executing binary with arguments:
    - [x] --file=<file> - specify file to execute
    - [x] --print-bytecode - print bytecode of the executed file

- [x] Add library crate which exposes interpreter entry point

- [x] Use library crate in the binary crate

- [x] Add number parsing (positive, negative, float, scientific notation)

- [x] Add string parsing ("<UTF-8 string>")

- [ ] Add parsing function calls

- [ ] Add static analysis module

- [ ] Create bytecode generator for producing bytecode from AST

- [ ] Create virtual machine for executing bytecode
