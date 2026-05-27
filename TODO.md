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

- [x] Add csv schema resolution (take schema ast and build CsvResolvedSchema
      that maps each col to type descriptor)
  
- [ ] Provide source code for schema errors (we might need to improve error handling in general)
      Move source code slicer into shard; slice source in in cli where it gets handled

- [ ] Combine CsvResoledSchema with parsed csv records to produce TypedDataGraph
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
