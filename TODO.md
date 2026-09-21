# TODO

## Things to implement

NOTE: .elt schema file must contain :Data type definition. It can also define other types.
      All of them are injected into the global scope of the semanalyzer.
      Add TypeBinder that takes TypeDef ast nodes and creates HashMap<BindingPath, TypeDesc>.
      We can use it for binding custom type definitions in the source code during semanalyzing
      and also bind type definitions in .elt file which is also the same Ast.
      We also need to have a DataBinder that takes ast of data expressions and produces
      HashMap<BindingPath, DataDesc>

- [ ] TypeBinder

- [ ] DataBinder

- [ ] Data validation against schema

- [ ] Semantic analyzer (preserve as much types as you can)

    - [ ] Add lang data types

        - [ ] Int

        - [ ] Tests

    - [ ] Add ScopeStack for Harmony

    - [ ] Add tests for SymbolTable

    - [ ] Add tests for ScopeStack

    - [ ] Add semantics for .let (former .define)

        - [ ] Tests

    - [ ] Add semantics for function definition

    - [ ] Add semantics for .add

        - [ ] Tests

- [ ] Compiler

    - [ ] ...

- [ ] VM

    - [ ] ...

- [ ] Add support for comments

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

    - [ ] Maybe we don't want to store literal values in code structs like tokens etc
          but instead store Span and slice the source code on demand.
