# TODO

- [x] Parser

- [x] Data Parser (CSV)

- [x] TypeBinder

- [x] Semantic analyzer

    - [x] .typedef

    - [x] @data slot
    
    - [x] Int
    
    - [x] Str

    - [x] List

    - [x] Dicts
        
    - [x] .get

    - [x] .add

- [x] Data adapter for CSV

- [x] DataBinder

- [ ] Data validation

    - [ ] Impl

    - [ ] Tests

- [ ] Improve error reporting. Add more informative error messages

- [ ] Compiler

- [ ] VM

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

    - [ ] Maybe we don't want to store literal values in code structs like tokens etc
          but instead store Span and slice the source code on demand.
