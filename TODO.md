# TODO

## Things to implement

- [ ] Validation of data binding against resolved schema

- [ ] Semantic analyzer (preserve as much types as you can)

    - [x] Add lang data types

        - [x] Int, Float, String, Bool, Null

        - [x] Tests

    - [x] Add ScopeStack for Harmony

    - [x] Add tests for SymbolTable

    - [x] Add tests for ScopeStack

    - [x] Add semantics for .define

        - [x] Tests

    - [ ] Add semantics for .let

        - [ ] Tests

    - [ ] Add semantics for .mul

        - [ ] Tests

    - [ ] Add semantics for .add

        - [ ] Tests

- [ ] Compiler

    - [ ] ...

- [ ] VM

    - [ ] Add support for IEEE-754 numbers format

    - [ ] ...

- [ ] Try to add .union for schema resolver

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

    - [ ] Maybe we don't want to store literal values in code structs like tokens etc
          but instead store Span and slice the source code on demand.
