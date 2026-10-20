# TODO

- [x] Parser

- [x] Data Parser (CSV)

- [x] TypeBinder

- [ ] Review .span.clone() calls and move it under impl

- [ ] Semantic analyzer

    - [x] .typedef

    - [x] @data slot
    
    - [x] Int

    - [x] List

    - [x] Dicts
        
    - [ ] Create a type definition collector as a part of Harmony that
          can be used separately. It must accept a mutable AliasedTypeBindings
          struct that will be enriched with type definitions. It must return
          its own error types so we can use it only for .elt files
          with schema code for reports. Then, we can use it inside Harmony
          annotator as well for collecting local type definitions.
          In must only collect type definitions made from .typedef calls.

    - [ ] We need to make semantic analysis to have multiple passes.
          First pass is a type definition collection (.typedef), that uses
          type collector and produces an aliased type bindings. Second annotates AST
          nodes that represent data with types (infers type from data if needed).
          Third applies semantic rules and checks if types match. It can also
          change the structure of the resulting AAST if needed.

    - [ ] .get

        - [ ] Currently, when we do .get(.get(@data, 0), "id")
              we only get a final AAstNodeCall::Get with access path
              [Root, Field("id")] and return type [Root] => Int.
              This is correct behavior from technical perspective
              but we do not include any information what data we're
              accessing. We must include type information about data
              being accessed.

        - [ ] Tests

    - [ ] .add

        - [ ] Tests

- [ ] DataBinder

    - [ ] Takes AAst nodes. Before binding data, we adapt it depending on its type,
          analyze semantics for it since it's just a raw Elise code and then bind.
          We can use the same binder in Harmony if we want to add a custom data
          definitions. We would just get AAst nodes from that data and bind it.
          Additionaly check type match of course.

- [ ] Data adapter for CSV

- [ ] Data validation. Must be used for schema validation and also for a validation of the
      custom data inside Harmony

- [ ] Improve error reporting. Add more informative error messages.

- [ ] Compiler

- [ ] VM

- [ ] Optimizations
    
    - [ ] Research on how to read files effectively
    
        - [ ] Read data file in chunks (streaming parser)

    - [ ] Maybe we don't want to store literal values in code structs like tokens etc
          but instead store Span and slice the source code on demand.
