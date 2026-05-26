# TODO

Source code
    -> Prelude (parser)
    -> Harmony (semantic-analyzer)
    -> Maestro (compiler)
    -> Score (bytecode)
    -> Sonata (VM)

-----
 
- [x] CLI
  
- [x] Parser

- [x] Schema file parsing

- [x] Csv data file parsing

    - [x] Tests
 
- [x] Add csv schema resolution (take schema ast and build CsvResolvedSchema
      that maps each col to type descriptor)
    
    - [x] Number

    - [x] String

    - [x] Bool (true/false, 1/0, yes/no, y/n, on/off)

    - [x] Optional (NULL, N/A, -, ,,)

    - [x] Tests

- [ ] Extend parsed csv data with type annotations

- [ ] Combine CsvResoledSchema with raw csv records to produce TypedDataGraph
      IR that describes data in agnostic way.

- [ ] Semantic analyzer

    - [ ] ...

- [ ] Compiler

    - [ ] Apply optimizations

        - [ ] Constant folding
        
        - [ ] ...
    
    - [ ] ...

- [ ] VM

    - [ ] ...

- [ ] Optimizations

    - [ ] Read source code file in chunks

- [ ] Integration tests for CLI
