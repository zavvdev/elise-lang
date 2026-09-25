# TODO

## Things to implement

NOTE: .elt schema file must contain :Data type definition. It can also define other types.
      All of them are injected into the global scope of the semanalyzer.
      Add TypeBinder that takes TypeDef ast nodes and creates HashMap<BindingPath, TypeDesc>.
      We can use it for binding custom type definitions in the source code during semanalyzing
      and also bind type definitions in .elt file which is also the same Ast.
      We also need to have a DataBinder that takes ast of data expressions and produces
      HashMap<BindingPath, DataDesc>

     Each binding table must be associated with data/type it was bind to.
     For .elt file we'll return a HashMap where each key is a name of the defined
     type inside this file, and values are binding tables. Each table is a result
     of a TypeBinder. By doing so, we eliminate the need to attach type name directly.
     So we can bind type/data inside the source code and attach it to an arbitrary
     metadata like symbol or data descriptor.

     For now, our parser handles all grammar that was designed. It also emits ast nodes
     that represent all possible data types such as Int, Float, List, Dict etc.
     Semantic analyzer narrows them down only to things that are currently supported.
     In our case we only support: Int, .add function, @data slot, .let function,
     .typedef function, get function. Every other module after semanalyzer must only
     support these for now. Do not add anything that is not yet supported.

- [x] Parser

- [x] Data Parser (CSV)

- [ ] Semantic analyzer. It must accept a valid source code Ast + optional HashMap where key is a
      type name (alias) and value is a result of TypeBinder (hashmap where key is a binding path and
      value is a type descriptor). It's optional because we use the same semanalyzer for schema file
      and source code file, but when we analyze source code file, we inject type bindings into it
      so we can reference types defined in .elt file.

      NOTE: Continue from AastNode .get call

    - [ ] ScopeStack
    
    - [ ] Int

    - [ ] .typedef

    - [ ] .let

    - [ ] .add

    - [ ] .get

    - [ ] @data slot

- [ ] TypeBinder

- [ ] DataBinder

    - [ ] For each data (.csv, .json etc) create an adapter first, that translates them
          into Elise code. And then use the same DataBinder for it as well as for
          source code data. Alternative: write it's own Binder for each data type +
          have a separate binder for Elise data types.

- [ ] Data validation against schema

- [ ] Compiler

    - [ ] ...

- [ ] VM

    - [ ] ...

- [ ] Add support for comments

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

    - [ ] Maybe we don't want to store literal values in code structs like tokens etc
          but instead store Span and slice the source code on demand.
