# TODO

- [x] Parser

- [x] Data Parser (CSV)

- [x] TypeBinder

- [ ] Semantic analyzer

    - [x] .typedef

    - [x] @data slot
    
    - [x] Int
    
    - [x] Str

    - [x] List

    - [x] Dicts
        
    - [x] .get

    - [x] .add

- [ ] DataBinder

    - [ ] Takes AAst nodes. Before binding data, we adapt it depending on its type,
          analyze semantics for it since it's just a raw Elise code and then bind.
          We can use the same binder in Harmony if we want to add a custom data
          definitions. We would just get AAst nodes from that data and bind it.
          Additionaly check type match of course.

- [ ] Data adapter for CSV

- [ ] Data validation. Must be used for schema validation and also for a validation of the
      custom data inside Harmony

- [ ] Improve error reporting. Add more informative error messages.

- [ ] Compiler

- [ ] VM

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

    - [ ] Maybe we don't want to store literal values in code structs like tokens etc
          but instead store Span and slice the source code on demand.
