# TODO

- [x] Parser

- [x] Data Parser (CSV)

- [x] TypeBinder

- [x] Semantic analyzer

- [x] Data adapter for CSV

- [x] DataBinder

- [x] Data validation

- [ ] Improve error reporting. Add more informative error messages.

    - [x] ConfErr

    - [x] cli_run, cli_build, cli_exec, cli_validate in main.rs

    - [ ] Prelude

      - [x] UnexpTok

      - [x] UntermDelim

- [ ] Compiler

- [ ] VM

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

    - [ ] Maybe we don't want to store literal values in code structs like tokens etc
          but instead store Span and slice the source code on demand.
