# TODO

## Things to implement

- [ ] Support only these for now:
      String, Int, List, Dict, Identifier, Slot, Call
      TInt, TList, TRecord,
      .define, .add, .get

- [ ] New data type called TypeDef
      (:Int, :Optional<:Str>, :Nullable<:Float>, :List<:Str>, :Record<{"a" :Int}>)
      AstNode enum must have 2 branches: TypeDef and Expr where TypeDef is an enum of a
      type definition related nodes and Expr is everything else. Parser must guaratee
      that TypeDef ast nodes do not contain any Expr nodes.

- [ ] .elt schema file must contain :Data type definition. It can also define other types.
      All of them are injected into the global scope of the semanalyzer.
      Add TypeBinder that takes TypeDef ast nodes and creates HashMap<BindingPath, TypeDesc>.
      We can use it for binding custom type definitions in the source code during semanalyzing
      and also bind type definitions in .elt file which is also the same Ast.
      We also need to have a DataBinder that takes ast of data expressions and produces
      HashMap<BindingPath, DataDesc>

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
