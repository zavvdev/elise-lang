# elise-lang

Dynamically typed, functional programming language with ugly syntax.

## TODO

- [ ] Add foundation for executing binary with arguments:
    - [ ] --file=<file> - specify file to execute
    - [ ] --print-bytecode - print bytecode of the executed file

- [ ] Add library crate which exposes interpreter entry point

- [ ] Use library crate in the binary crate

- [ ] Create parser for parsing simple `.add` and `.declare` expression and producing AST. Tokenization is skipped because of the language syntax which is already a valid AST (Code is Data)

- [ ] Add static analysis module

- [ ] Create bytecode generator for producing bytecode from AST

- [ ] Create virtual machine for executing bytecode
