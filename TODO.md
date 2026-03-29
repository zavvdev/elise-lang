# TODO

- [x] Add foundation for executing binary with arguments:
  
    - [x] --file=<file> - specify file to execute
    
    - [x] --print-bytecode - print bytecode of the executed file

- [x] Add library crate which exposes interpreter entry point

- [x] Use library crate in the binary crate

- [ ] Create parser for lexing, building AST and enforcing grammar rules 

    - [x] Add number parsing (positive, negative, float, scientific notation)

    - [x] Add string parsing ("<UTF-8 string>")

    - [x] Add parsing identifiers

        - [x] Add parsing bool

        - [x] Add parsing null

        - [x] Custom with identifier rule check

        - [x] Add tests for identifiers

    - [x] Add parsing lists

        - [x] Add tests for lists

    - [x] Add parsing dictionaries

        - [x] Add tests for dictionaries

    - [ ] Add parsing function calls

        - [ ] Add tests for function calls

    - [ ] Review tests and add more test cases where possible

    - [ ] Review parser for improvements (including messages)

- [ ] Add semantic analysis module

    - [ ] ...

- [ ] Create a compiler for bytecode (IR) generation from AST

    - [ ] Apply optimizations

        - [ ] Constant folding
        
        - [ ] ...
    
    - [ ] ...

- [ ] Create a virtual machine for bytecode interpretation

    - [ ] ...
