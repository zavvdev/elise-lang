# TODO

Source code
    -> Prelude (parser)
    -> Harmony (semantic-analyzer)
    -> Maestro (compiler)
    -> Score (bytecode)
    -> Sonata (VM)

-----

1. Parse source code, data schema and data (csv) file in separate threads.

2. Run semantic analysis on source code and schema file.

3. Run type coercion for data file against schema and produce typed data structure that represents data.

4. Compile

5. Run VM

-----
 
- [x] CLI
  
- [x] Parser

- [x] Schema file parsing

- [x] Csv data file parsing
 
- [ ] Add csv schema resolution (take schema ast and build CsvResolvedSchema
      that maps each col to type descriptor)
    
    - [ ] Number

    - [ ] String

    - [ ] Bool (true/false, 1/0, yes/no, y/n, on/off)

    - [ ] Empty (NULL, N/A, -, ,,)

- [ ] Add TypedDataGraph types for generic data representation (shared/ir)

- [ ] Combine CsvResoledSchema with raw csv records to produce TypedDataGraph
      IR that describes data in agnostic (binder)

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
