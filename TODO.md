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

- [x] Add support for escape chars in string literals

- [x] Span is wrong for strings with UTF-8 chars like emoji

- [ ] Update error reporting with code snippet

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

- [ ] Research on how to read files effectively

- [ ] Optimizations

    - [ ] Read source code file in chunks

- [ ] Integration tests for CLI
