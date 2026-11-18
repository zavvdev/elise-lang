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

    - [x] Prelude

    - [ ] Harmony

      - [x] ArityMismatch 

      - [x] UnknFnCall

      - [x] UnknSlot 
    
      - [x] UnknType

      - [x] UnexpNode
      
      - [x] UnexpExpr

      - [x] UnexpType 

      - [x] MissingGeneric 
      
      - [ ] UnsupListItem 

      - [ ] UnsupDictItem 

      - [ ] InvalGeneric 

      - [ ] InvalDataAccessArg 

      - [ ] UnresolvableTypedef 

      - [ ] TypedefMode 

      - [ ] ListInconsistentItemTypes 

      - [ ] CannotInferType 

      - [ ] NoTypeDefinition 

      - [ ] UnresolvablePath 

    - [ ] Remove Parser Error and Semantic Error titles from the error reports and use meaningful
          titles like "Syntax error: ...". Improve messages as well.

    - [ ] Add error codes like E201, E202

- [ ] Compiler

- [ ] VM

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

    - [ ] Maybe we don't want to store literal values in code structs like tokens etc
          but instead store Span and slice the source code on demand.
