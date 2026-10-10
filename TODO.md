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

- [ ] DataBinder

    - [ ] Takes AAst nodes. Before binding data, we adapt it depending on its type,
          analyze semantics for it since it's just a raw Elise code and then bind.
          We can use the same binder in Harmony if we want to add a custom data
          definitions. We would just get AAst nodes from that data and bind it.
          Additionaly check type match of course.

- [ ] Semantic analyzer

    - [x] .typedef

    - [x] @data slot

    - [ ] List

        - [ ] Must contain only static data

        - [ ] Tests

    - [ ] Dicts

        - [ ] Must contain only static data

        - [ ] Tests

    - [ ] .add

        - [ ] Tests
    
    - [ ] .get

        - [ ] Tests

- [ ] Data adapter for CSV

- [ ] Data validation

- [ ] Compiler

- [ ] VM

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

    - [ ] Maybe we don't want to store literal values in code structs like tokens etc
          but instead store Span and slice the source code on demand.
