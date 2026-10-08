# TODO

## Things to implement

NOTE: .elt schema file must contain :Data type definition. It can also define other types.
      All of them are injected into the global scope of the semanalyzer.

     For now, our parser handles all grammar that was designed. It also emits ast nodes
     that represent all possible data types such as Int, Float, List, Dict etc.
     Semantic analyzer narrows them down only to things that are currently supported.
     In our case we only support: Int, .add function, @data slot, .typedef function,
     .get function. Every other module after semanalyzer must only
     support these for now. Do not add anything that is not yet supported.

- [x] Parser

- [x] CSV Data Parser

- [x] TypeBinder

- [ ] Review TODOS (return own spans for span fns)

- [ ] Semantic analyzer

    - [x] .typedef

    - [x] @data slot

    - [ ] .add

        - [ ] Tests
    
    - [ ] .get

        - [ ] Tests

- [ ] DataBinder

    - [ ] For each data (.csv, .json etc) create an adapter first, that translates them
          into Elise code. And then use the same DataBinder for it as well as for
          source code data. Alternative: write it's own Binder for each data type +
          have a separate binder for Elise data types.

- [ ] Data validation

- [ ] Compiler

- [ ] VM

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

    - [ ] Maybe we don't want to store literal values in code structs like tokens etc
          but instead store Span and slice the source code on demand.
