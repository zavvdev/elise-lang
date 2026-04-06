# TODO

Source code
    -> Prelude (parser)
    -> Harmony (semantic-analyzer)
    -> Maestro (compiler)
    -> Score (bytecode)
    -> Sonata (VM)

- [ ] Build config from CLI arguments:
  
    - [ ] Add support for different exec modes
    
    - [ ] tests
    
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

    - [x] Add parsing function calls

        - [x] Add tests for function calls

    - [x] Review tests and add more test cases where possible

    - [x] Review parser for improvements (including messages)

    - [ ] Add support for anonymous functions

- [x] Create new module system (bytecode, parser, compiler, vm, cli)

- [ ] Add semantic analysis module (analyze each known function semantics etc)

    - [ ] ...

- [ ] Create a compiler for bytecode (IR) generation from AST

    - [ ] Apply optimizations

        - [ ] Constant folding
        
        - [ ] ...
    
    - [ ] ...

- [ ] Create a virtual machine for bytecode interpretation

    - [ ] ...

- [ ] Optimizations

    - [ ] Read source code file in chunks
