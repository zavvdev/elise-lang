# TODO

## Things to implement

- [ ] Add static types

    - [ ] New data type called TypeDef
          (:Int, :String?, :Float|Null, :List<String>, Dict{"a" :Int})

    - [ ] Update schema binding

    - [ ] Remove support for .let (all we need for now is .define, .fn and builtin functions)

- [ ] Disallow multiple types in List

- [ ] Add support for comments

- [x] Data validation against schema

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

    - [ ] Add semantics for function definition (named and anon)

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
