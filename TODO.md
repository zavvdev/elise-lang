# TODO

- [x] Parser

- [x] Data Parser (CSV)

- [x] TypeBinder

- [x] Semantic analyzer

- [x] Data adapter for CSV

- [x] DataBinder

- [x] Data validation

- [ ] Improve error reporting. Add more informative error messages. Consider collecting as much
  errors as possible instead of failing fast.

- [ ] Compiler

- [ ] VM

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

    - [ ] Maybe we don't want to store literal values in code structs like tokens etc
          but instead store Span and slice the source code on demand.
