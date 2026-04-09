# TODO

Source code
    -> Prelude (parser)
    -> Harmony (semantic-analyzer)
    -> Maestro (compiler)
    -> Score (bytecode)
    -> Sonata (VM)

- [ ] CLI:
  
    - [x] Add support for different exec modes

    - [x] Rewrite error/messaging handling. Errors and messages should only be handled inside
      main.rs as a library consumer.
    
    - [ ] fsys tests

    - [ ] conf tests
    
- [ ] Parser

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

    - [ ] Remove out module

    - [ ] Add support for anonymous functions

- [ ] ?Data file parsing, schema file parsing

- [ ] Semantic analyzer

    - [ ] ...

- [ ] Compiler

    - [ ] Apply optimizations

        - [ ] Constant folding
        
        - [ ] ...
    
    - [ ] ...

- [ ] VM

    - [ ] ...

- [ ] Optimizations

    - [ ] Read source code file in chunks
