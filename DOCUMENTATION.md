# Documentation

## Execution Pipeline

Steps 1–3 run in parallel:

1. `frontend/parser` parses source code `.eli` file → `AST`
2. `frontend/parser` parses schema `.elt` file → `AST`, then `frontend/csv/schema_resolver` walks it → `CsvResolvedSchema`
3. `frontend/csv/parser` reads data file → `CsvParserRecord`

Then sequentially:

4. `frontend/binder` validates `CsvParserRecord` against `CsvResolvedSchema` → `DataBindingTable` which is data agnostic IR 
5. `frontend/semantic-analyzer` walks source `AST` → `SemanticIR`
6. `compiler` takes `SemanticIR` + `DataBindingTable` and produces `bytecode` with serialized `DataBindingTable` (agnostic data representation for VM)
7. `runtime/vm` deserializes data and executes `bytecode`

Note: the same parser is used for both source and schema files. Schema syntax is identical to source syntax by design.

---

## Modules

### `shared/builtins`
Language builtin functionality. No dependencies.

### `shared/errors`
Centralized `LangErr` enum wrapping all subsystem error kinds. Depends on `shared/types`.

### `shared/types`
Shared types. No dependencies.

### `frontend`
Module that is responsible for syntax/grammar related manipulations.

### `frontend/ast`
`AstNode` definitions. Depends on `shared/types`. A frontend-internal artifact — never escapes into `compiler` or `runtime`.

### `frontend/parser`
Parses source `.eli` and schema `.elt` files into `AST`. Depends on `frontend/ast`, `shared/errors`, `shared/types`.

### `frontend/csv`
Two responsibilities:
- CSV parser: reads data file → `CsvParserRecord`
- Schema resolver: walks schema `AST` → `CsvResolvedSchema`

Depends on `frontend/ast`, `shared/errors`, `shared/builtins`, `shared/types`.

### `frontend/semantic-analyzer`
Walks source `AST`, resolves identifiers and types → `SemanticIR`. Depends on `frontend/ast`, `shared/errors`, `shared/builtins`

### `binder`
Validates `CsvParserRecord` against `CsvResolvedSchema` and produces `DataBindingTable`. Depends on `frontend/csv`.

### `compiler`
Takes `SemanticIR` + `DataBindingTable`, emits `bytecode`. Depends on `binder`, `frontend/semantic-analyzer`. Has no knowledge of `ast` or `runtime`.

### `bytecode`
Bytecode instruction definitions. No dependencies. A shared neutral contract between `compiler` (writes) and `runtime/vm` (reads) — owned by neither.

### `runtime/vm`
Executes bytecode. Depends only on `bytecode`. Has no knowledge of `compiler`, `AST`, or any frontend artifact.

### `cli`
Composition root. Orchestrates the pipeline, handles all user-facing error display.

---

## Dependency Placement Rules

**One consumer** — the data structure lives with the crate that produces it. The consumer declares a dependency on the producer.
- `SemanticIR` lives in `frontend/semantic-analyzer`, consumed only by `compiler`
- `DataBindingTable` lives in `binder`, consumed only by `compiler`
- `CsvResolvedSchema` and `CsvParserRecord` live in `frontend/csv`, consumed only by `binder`

**Two independent consumers** — the data structure lives at the root as a neutral contract. Neither consumer depends on the other.
- `bytecode` is written by `compiler` and read by `runtime/vm`. Placing it under either would create an incorrect dependency direction.

**Cross-cutting concerns** — live in `shared/`. No single subsystem owns them; pulling them into any one crate would force incorrect dependencies across the graph.
- `shared/builtins`, `shared/errors`, `shared/types`.

## Design decisions

### General execution pipeline

```
Source code
    -> Prelude (parser)
    -> Harmony (semantic-analyzer)
    -> Maestro (compiler)
    -> Score (bytecode)
    -> Sonata (VM)
```

### Lexing & Parsing

Elise syntax is designed to be Code as Data where source is already shaped like an AST. Given that, lexing and parsing are combined into a single Parser step in order to reduce number of iterations and build AST right away.

### Data Binding Stage

Lives in `src/binder`.

The data binding stage is responsible for building a data structure that can simplify accessing data.
For example, if we have `.csv` file, users will access data by mapping rows and accessing column names. Or, if we're talking about `.json`, it might also we a nested access since we can have arrays and objects in there.

So, binder takes structured data and its schema and produces `DataBindingTable` that has a hashmap:

```
(Index(0), Field(“name”)) → Descriptor
(Index(0), Field(“age”))  → Descriptor
```

This structure represents a mapping between **data access paths** and their corresponding metadata.

```rust
enum PathSegment {
    Index(usize),
    Field(String),
}

type Path = Vec<PathSegment>;

struct DataBindingTable {
    table: HashMap<Path, DataDescriptor>,
}
```

This data structure is data agnostic and can be used for `csv` and `json`.

### Compilation Stage

During compilation stage, we create some `ConstantPool` structure which is represented as a Vector, and a `ResolutionCache` (names can be different) HashMap.

When we walk AST and encounter data access procedure, we build a `Path` from `PathSegments` and resolve metadata from `DataBindingTable` using that `Path`. Now when we have a data, we push it into `ConstantPool` and obtain an index (last element). Then we cache that index into `ResolutionCache` (Path -> Index) so we don't need to construct it every time. Once we resolved the data, we can drop it from DataBindingTable which will be discarded eventually since it's not needed after compilation stage.

So our compiler now can emit bytecode where data load procedures point to a `ConstantPool` index.
And after the compilation we can also serialize that `ConstantPool` into a deserializable data for further usage in VM.
