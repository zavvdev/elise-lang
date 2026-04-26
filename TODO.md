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

- [ ] Data file parsing

    - [ ] Add data parser dispatcher to correct parser according to data type

    - [ ] Add error info for data parser + error handling in main.rs
    
    - [ ] Add csv parser that will parse csv into raw string records

    - [ ] Add raw csv strings coercion to data types defined in .elt
        
        - [ ] Number

        - [ ] String

        - [ ] Bool (true/false, 1/0, yes/no, y/n, on/off)

        - [ ] Date (2024-01-15, ISO 8601 is common, but formats vary wildly)
        
        - [ ] Datetime (2024-01-15T14:30:00)

        - [ ] Datetime with timezone (2024-01-15T14:30:00Z)

        - [ ] Time (14:30:00)

        - [ ] Empty (NULL, N/A, \N, -, ,,)

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
