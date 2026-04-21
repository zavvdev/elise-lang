# TODO

Source code
    -> Prelude (parser)
    -> Harmony (semantic-analyzer)
    -> Maestro (compiler)
    -> Score (bytecode)
    -> Sonata (VM)

- [x] CLI:
  
    - [x] Add support for different exec modes

    - [x] Rewrite error/messaging handling. Errors and messages should only be handled inside
      main.rs as a library consumer.
    
    - [x] fsys tests

    - [x] conf tests

    - [x] Add support for output (run mode)
    
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

    - [x] Remove out module
        
        - [x] Remove human readable messages from everywhere except cli/out. Use enums.

    - [x] Refactor tests 

    - [x] Remove should panic crate 

    - [x] Add support for anonymous functions
    
    - [ ] Add support for slots (@data)

    - [ ] Improve source code slice function

- [ ] Schema file parsing

- [ ] Data file parsing

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

- [ ] Integration tests for CLI
