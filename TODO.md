# TODO

- [x] Add foundation for executing binary with arguments:
  - [x] --file=<file> - specify file to execute
  - [x] --print-bytecode - print bytecode of the executed file

- [x] Add library crate which exposes interpreter entry point

- [x] Use library crate in the binary crate

- [x] Add number parsing (positive, negative, float, scientific notation)

- [x] Add string parsing ("<UTF-8 string>")

- [ ] Add parsing lists -- IN PROGRESS

- [ ] Add tests for lists

- [ ] Add parsing dictionaries

- [ ] Add tests for dicts

- [ ] Add parsing identifiers

    - [ ] Add parsing bool

    - [ ] Add parsing nil

- [ ] Add tests for identifiers

- [ ] Add parsing function calls

- [ ] Add tests for function calls

- [ ] Add static analysis module

- [ ] Create bytecode generator for producing bytecode from AST

- [ ] Create virtual machine for executing bytecode
