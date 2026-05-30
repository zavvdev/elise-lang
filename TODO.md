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

- [x] Review how we work with source code in Prelude. We need to convert into bytes right after
      file has been read and always pass a vector of bytes.

- [x] Optimize regex usage in Prelude. It executes every time we construct identifier.
      We might also can replace it with raw byte checks.

- [x] Provide source code for schema errors (we might need to improve error handling in general)
      Move source code slicer into shared; slice source in in cli where it gets handled

- [ ] Parser doesn't know which kind of code he's parsing. For example, if we parse source code
      then error prints source code slice of the source code. But when we parse schema and
      get ParserErr, error reports row/col of the schema source code but slice is created
      from source code.

- [ ] Combine CsvResolvedSchema with parsed csv records to produce TypedDataGraph
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
