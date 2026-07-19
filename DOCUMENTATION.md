# Documentation

## General execution pipeline

```
Source code
    -> Prelude (parser)
    -> Harmony (semanalyzer)
    -> Maestro (compiler)
    -> Score (bytecode)
    -> Sonata (VM)
```

Steps 1–3 run in parallel:

1. `Prelude` (parser) parses source code `.eli` file and produces `AST`
2. `Prelude` parses schema `.elt` file and produces `AST` 
3. `CsvParser` reads data and produces parsed data representation `Vec<CsvRow>`

Then sequentially:

4. `CsvSchemaResolver` takes schema AST and produces `CsvResolvedSchema` which is a convenient representation of schema types.
5. `CsvDataBinder` validates `CsvParserRecord` against `CsvResolvedSchema` → `DataBindingTable` which is data agnostic IR 
6. `Harmony` (semantic analyzer) takes source code `AST` and `DataBindingTable` → `HIR` with `SymbolTable` + `AAST` (optimized, annotated)
7. `compiler` takes `HIR` + `DataBindingTable` and produces `bytecode` with serialized `DataBindingTable` (agnostic data representation for VM)
8. `runtime/vm` deserializes data and executes `bytecode`

Note: the same parser (Prelude) is used for both source and schema files. Schema syntax is identical to source syntax by design.

---

## Modules

### `shared`
Centralized shared crates such as errors and types. Depends on nothing.

### `frontend`
Module that is responsible for syntax/grammar related manipulations.

### `frontend/ast`
`AstNode` definitions. Depends on `shared`. A frontend-internal artifact — never escapes into `compiler` or `runtime`.

### `frontend/data`
Contains data binder and files that are related to data being processed (csv, json). Depends on `shared` and `frontend/ast`.

### `frontend/parser`
Parses source `.eli` and schema `.elt` files into `AST`. Depends on `frontend/ast`, `shared`.

### `frontend/semanalyzer`
Takes source `AST` and `DataBindingTable` → `HIR`. Depends on `frontend/ast`, `frontend/data`, `shared`.

### `compiler`
Takes `HIR` + `DataBindingTable`, emits `bytecode`. Depends on `frontend/data`, `frontend/semanalyzer`. Has no knowledge of `ast` or `runtime`.

### `bytecode`
Bytecode instruction definitions. No dependencies. A shared neutral contract between `compiler` (writes) and `runtime/vm` (reads) — owned by neither.

### `runtime/vm`
Executes bytecode. Depends only on `bytecode`. Has no knowledge of `compiler`, `AST`, or any frontend artifact.

### `cli`
Composition root. Orchestrates the pipeline, handles all user-facing error display.

---

## Design decisions

### Lexing & Parsing

Elise syntax is designed to be _Code as Data_ where source is already shaped like an _AST_. Given
that, lexing and parsing are combined into a single Parser step in order to reduce number of
iterations and build _AST_ right away.

### Data Binding Stage

The data binding stage is responsible for building a data structure that can simplify accessing data.
For example, if we have `.csv` file, users will access data by mapping rows and accessing column names. Or, if we're talking about `.json`, it might also we a nested access since we can have arrays and objects in there.

So, binder takes structured data and its schema and produces `DataBindingTable` that has a hashmap:

```
(Index(0), Field(“name”)) → Descriptor
(Index(0), Field(“age”))  → Descriptor
```

This structure represents a mapping between **data access paths** and their corresponding metadata.

This data structure is data agnostic and can be used for `csv` and `json`.

### Semantic Analysis Stage

Semantic analyzer takes `AST` and `DataBindingTable`. The result is `HIR` (High-level Intermediate
Representation) which includes `SymbolTable` and `AAST` (Annotated Abstract Syntax Tree).

#### Why do we need SymbolTable

Consider this:

```
.let([x 1]
  .let([x 2]
    x))
```

Our parser gives us two `Identifier("x")` nodes. But they mean different things - they point to two completely different bindings. If the compiler only has strings, it has to redo scope resolution at compile time. Instead, semantic analysis does the resolution once, assigns each binding a unique integer id, and replaces every identifier reference with that id. Now the compiler sees:

```
Let { bindings: [(SymbolId(1), 1)],
  body: Let { bindings: [(SymbolId(2), 2)],
    body: Identifier(SymbolId(2)) }}  // unambiguous
```

`SymbolId(2)` unambiguously means "the inner x". The compiler can use it as an index into an array of symbol metadata, or map it to a stack slot — no string lookups, no re-resolution.

#### ScopeStack

Ephemeral data structure that lives only during AST walk.

Looks like:

```rust
pub struct Scope {
    bindings: HashMap<String, SymbolId>,
}

pub struct ScopeStack {
    scopes: Vec<Scope>,
}
```

When we walk AST, we carry both the `ScopeStack` and `SymbolTable` as mutable state:

**Entering a `let`:**

1. Resolve the init expression(s) *before* pushing the scope (so `.let([x x] ...)` doesn't let `x` see itself)

2. Push a new scope

3. For each binding name, call `symbol_table.fresh(...)` to get a `SymbolId`, call `scope_stack.define(name, id)`

4. Recurse into the body

5. Pop the scope — the names are gone, but the `SymbolId`s live on in the HIR and SymbolTable forever

**Encountering an `Identifier`:**

1. Call `scope_stack.resolve(name)`

2. If `None` → undefined variable error

3. If `Some((id, _))` → emit `HirNode::Identifier { symbol_id: id }`

**Entering a function `.([row index] body)`:**

1. Push a fresh scope, register each param with a new `SymbolId`

2. Recurse into the body — *but track which identifiers resolve to a depth > 0 from the function's own scope boundary*

3. Those are our captures — collect their `SymbolId`s, put them in the `Fn` node's `captures` list, and mark `symbol_table.symbols[id].is_captured = true`

4. Pop the scope

#### Closures

```
.let([prefix "Report: "]
  .map(
    .([row] .concat(prefix row))))
```

`prefix` is defined in the outer let-scope. The anonymous function `.([row] ...)` references it. When the function is called later (inside `.map`), the let-scope is long gone from the call stack. How does the runtime find `prefix`?

The answer is: **the function object itself carries a copy of (or reference to) its captured variables**. This object is called a **closure**. At runtime, when the interpreter or VM creates this function value, it bundles `prefix`'s current value into the closure object alongside the function's code.

1. During the walk, when we see `prefix` inside the function body, `scope_stack.resolve` returns
`Some((SymbolId(1), depth=2))` — depth > 0 relative to the function boundary means it's not local
but from outer scope.

2. We add `SymbolId(1)` to the `Fn` node's `captures: Vec<SymbolId>`

3. We mark `symbol_info.is_captured = true` for that symbol

### Compilation Stage

During compilation stage, we create some `ConstantPool` structure which is represented as a Vector, and a `ResolutionCache` (names can be different) HashMap.

When we walk AST and encounter data access procedure, we build a `Path` from `PathSegments` and resolve metadata from `DataBindingTable` using that `Path`. Now when we have a data, we push it into `ConstantPool` and obtain an index (last element). Then we cache that index into `ResolutionCache` (Path -> Index) so we don't need to construct it every time. Once we resolved the data, we can drop it from DataBindingTable which will be discarded eventually since it's not needed after compilation stage.

So our compiler now can emit bytecode where data load procedures point to a `ConstantPool` index.
And after the compilation we can also serialize that `ConstantPool` into a deserializable data for further usage in VM.
