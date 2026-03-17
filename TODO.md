# TODO

- [x] Add foundation for executing binary with arguments:
  
    - [x] --file=<file> - specify file to execute
    
    - [x] --print-bytecode - print bytecode of the executed file

- [x] Add library crate which exposes interpreter entry point

- [x] Use library crate in the binary crate

- [ ] Create parser for lexing, building AST and enforcing grammar rules 

    - [x] Add number parsing (positive, negative, float, scientific notation)

    - [x] Add string parsing ("<UTF-8 string>")

    - [ ] Add parsing identifiers

        - [ ] Add parsing bool

        - [ ] Add parsing nil

        - [ ] Custom with identifier rule check

    - [ ] Add tests for identifiers 

    - [ ] Add parsing lists

    - [ ] Add tests for lists

    - [ ] Add parsing dictionaries

    - [ ] Add tests for dictionaries

    - [ ] Add parsing function calls

    - [ ] Add tests for function calls

    - [ ] Review parser for improvements

- [ ] Add semantic analysis module

    - [ ] ...

- [ ] Create a compiler for bytecode (IR) generation from AST

    - [ ] Apply optimizations

        - [ ] Constant folding
        
        - [ ] ...
    
    - [ ] ...

- [ ] Create a virtual machine for bytecode interpretation

    - [ ] ...
