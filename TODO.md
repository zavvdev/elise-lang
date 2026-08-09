# TODO

## Things to implement

- [x] Refactor schema resolver (?Maybe we need to create a global type resolver instead of this
                                so we can use this resolution algorithm for any data structs in
                                the source code as well that user creates)

    - [x] Make it data agnostic that works only on schema ast.
          Add .list and .dict types which can be used for JSON.

    - [x] Use path segments instead of strings for hash table keys,
          so this data structure can be used during compilation.
          Check sample.elt for more details

    - [x] Docs
    
    - [x] Apply .nullable modifier only for the direct child
    
    - [x] Add doc comments for modifiers

    - [x] Add .optional modifier (deep=false)
    
    - [x] Tests

- [ ] Csv data file parsing

    - [ ] Do not coerce empty strings to Null

- [ ] Refactor Data Binder (data agnostic repr. that uses the same path segment as resolved schema)

    - [ ] Csv binder

- [ ] Validation of data binding against resolved schema (must be a separate stage. See DOCUMENTAITON)

- [x] CLI
  
- [x] Parser

- [ ] Semantic analyzer (preserve as much types as you can)

    - [x] Add lang data types

        - [x] Int, Float, String, Bool, Null

        - [x] Tests

    - [x] Add ScopeStack for Harmony

    - [x] Add tests for SymbolTable

    - [x] Add tests for ScopeStack

    - [x] Add semantics for .define

        - [x] Tests

    - [ ] Add semantics for .let

        - [ ] Tests

    - [ ] Add semantics for .mul

        - [ ] Tests

    - [ ] Add semantics for .add

        - [ ] Tests

- [ ] Compiler

    - [ ] ...

- [ ] VM

    - [ ] Add support for IEEE-754 numbers format

    - [ ] ...

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

## Things to learn

1. - [ ] [Compilers](https://pgrandinetti.github.io/compilers/)

### Deterministic Finite Automata theory

1. - [x] [Basics](https://cs.stanford.edu/people/eroberts/courses/soco/projects/2004-05/automata-theory/basics.html#fsm)

2. - [ ] [Theory of computation](https://www.geeksforgeeks.org/theory-of-computation/theory-of-computation-automata-tutorials/)

3. - [ ] [Automata Theory](https://www.tutorialspoint.com/automata_theory/index.htm)

4. - [ ] [Introduction to automata theory](https://medium.com/@shehanikavishkarg/introduction-to-automata-theory-the-foundation-of-computational-science-90a038b074fe).

5. - [ ] [Theory of computation & Automata theory](https://www.youtube.com/playlist?list=PLBlnK6fEyqRgp46KUv4ZY69yXmpwKOIev)

### Parsing theory

1. - [ ] [Recursive descent parser](https://www.geeksforgeeks.org/compiler-design/recursive-descent-parser/)

2. - [ ] [A recursive descent parser from zero](https://medium.com/@curtmatthewgarcia/a-recursive-descent-into-enlightenment-65fd2b567d6d)

3. - [ ] [Recursive descent parsing](https://www.cs.rochester.edu/u/nelson/courses/csc_173/grammars/parsing.html)

4. - [ ] [Why I'm using a recursive descent parser](https://poly.substack.com/p/why-im-using-a-recursive-descent)

5. - [ ] [The art of writing recursive descent parsers](https://arielortiz.info/pycon2025/)

6. - [ ] [Parsing theory](https://www.geeksforgeeks.org/compiler-design/introduction-of-parsing-ambiguity-and-parsers-set-1/)

7. - [ ] [Intro to parsing theory](https://aiju.de/misc/parsing)

8. - [x] [LL(1) Grammar](https://www.tutorialspoint.com/compiler_design/compiler_design_ll1_grammar.htm) 

9. - [x] [Left-recursive PEG Grammars](https://medium.com/@gvanrossum_83706/left-recursive-peg-grammars-65dab3c580e1) 

### Streamable Parser (for parsing large files)

1. - [ ] [Tree parser vs Stream parser](https://stackoverflow.com/questions/18382957/tree-parser-vs-stream-parser)

2. - [ ] [How to write a streaming parser](https://jsoneditoronline.org/indepth/parse/streaming-parser/)

...
