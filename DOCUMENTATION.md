```
SC - Source Code
SCH -> Schema

RUN (SC, SCH, Data)
    -> Parse(SC) -> SC_AST +
    -> Parse(SCH) -> SCH_AST +
    -> Parse(Data) -> ParsedData +
    -> Sema(SC_AST) -> SC_AAST 
    -> Resolve(SCH_AST) -> ResolvedSchema +
    -> Bind(ParsedData) -> DataBindingTable +
    -> Validate(ResolvedSchema, DataBindingTable)
    -> Compile(ResolvedSchema, SC_AAST) -> Bytecode
    -> VM(Bytecode, DataBindingTable)

BUILD (SC, SCH)
    -> Parse(SC) -> SC_AST
    -> Parse(SCH) -> SCH_AST
    -> Sema(SC_AST) -> SC_AAST 
    -> Resolve(SCH_AST) -> ResolvedSchema
    -> Compile(ResolvedSchema, SC_AAST) -> Bytecode

VALIDATE(SCH, Data)
    -> Parse(SCH) -> SCH_AST
    -> Parse(Data) -> ParsedData
    -> Resolve(SCH_AST) -> ResolvedSchema
    -> Bind(ParsedData) -> DataBindingTable
    -> Validate(ResolvedSchema, DataBindingTable)

EXEC(Bytecode, Data)
    -> Parse(Data) -> ParsedData
    -> Bind(ParsedData) -> DataBindingTable
    -> VM(Bytecode, DataBindingTable)
```

```
Source code
    -> Prelude (parser)
    -> Harmony (semanalyzer)
    -> Maestro (compiler)
    -> Score (bytecode)
    -> Sonata (VM)
```

## Design decisions

### Lexing & Parsing

Elise syntax is designed to be _Code as Data_ where the source code is already shaped like an _AST_.
Given that, lexing and parsing are combined into a single Parser step in order to reduce number of
iterations and build _AST_ right away.

### Data types resolution & validation

The main concept behind how data and types are resolved and validated is a usage of a common path
that is called ResolutionPath.

Imagine that you have a data like this:

```json
{
    "user": {
        "email": "test@mail.com",
        "phone_numbers": ["+00000", "+11111"]
    }
}
```

and let's say you have a schema for this data:

```
.schema(
    .dict(
        "user" .dict(
                    "email"         .string()
                    "phone_numbers" .list(.string()))))
```

How can we validate this data against the schema? How can we extract the data during runtime?
In order to do this, we first resolve the schema into the structure that looks like this:

```
HashMap {
    [Root] => Dict,
    [Root, Field("user")] => Dict,
    [Root, Field("user"), Field("email")] => Descriptor,
    [Root, Field("user"), Field("phone_numbers")] => List,
    [Root, Field("user"), Field("phone_numbers"), AbstractIndex] => String,
}
```

We can call this HashMap as a schema resolution since we resolve each path that represents a data
access to a data type.

After that, we build exactly the same HashMap for data itself but each path corresponds to the data
itself and its type.

So having 2 hash tables where each key is the same (if schema matches), the validation/type
extraction becomes trivial as well as data access during runtime since all we need to do is to
re-create the same path and access data/type from the respective hash map.

For example, if user accesses data during runtime:

```
.get(@data, "user", "email")
```

this is already tells us how to construct the path. `@data` slot is a Root, `"user"` is a dict key
(`Field("user")`) and so on. So by tracking resolution path during runtime, we can extract type/data
in O(1).

### Semantic Analysis Stage

Semantic analyzer takes `AST` and produces `HIR` (High-level Intermediate
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

1. Resolve the init expression(s) _before_ pushing the scope (so `.let([x x] ...)` doesn't let `x` see itself)

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

2. Recurse into the body — _but track which identifiers resolve to a depth > 0 from the function's own scope boundary_

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
