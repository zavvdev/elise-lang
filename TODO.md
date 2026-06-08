# TODO

## Things to implement
 
- [x] CLI
  
- [x] Parser

- [x] Schema file parsing

- [x] Csv data file parsing

- [x] Csv schema resolution (take schema AST and build CsvResolvedSchema
      that maps each col to type descriptor)

- [x] Combine CsvResolvedSchema with parsed csv records to produce DataBindingTable 
      IR that describes data in agnostic way.

- [ ] Semantic analyzer

    - [ ] Source code

    - [ ] Schema

- [ ] Compiler

    - [ ] ...

- [ ] VM

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
