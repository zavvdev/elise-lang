# Semantics

## Pipeline Overview

```
Parser → AST (untyped, raw literals)
              ↓
Semantic Analysis → AAST (typed (granular like split number into int and float), annotated, folded where possible)
```

## Core Data Structures

### LangType

```rust
pub enum LangType {
    Int,
    Float,
    String,
    Bool,
    // ...
}
```

Don't use a bare `Number` variant — split `Int` and `Float` early so the bytecode
emitter can generate typed arithmetic opcodes without re-deriving it later.

### LiteralValue

```rust
pub enum LiteralValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}
```

### SymbolDescriptor

```rust
pub struct SymbolDescriptor {
    pub name: String,
    pub ty: LangType,
    pub const_value: Option<LiteralValue>,  // Some only when value is known at compile time
    pub is_captured: bool,
}
```

`const_value` is populated by semantic analysis via constant folding.
It is `None` for any symbol whose value is only known at runtime.

### AastValue — bridges SymbolTable and BindingTable

```rust
pub enum AastValue {
    Const(LiteralValue),   // fully resolved → goes into constant pool
    DataRef(Path),         // runtime data path → emits LOAD_FIELD opcode.
    // for example:
    // AastNode::Get {
    //  ty: LangType::String,           // known from schema
    //  path: DataRef(vec![
    //      PathSegment::Index(0),
    //      PathSegment::Field("name".to_string()),
    //  ])
    // }
    // we just carry a path to the data that will be used by VM. Code example: .get(.get(@data, 0), "name")
    SymbolRef(SymbolId),   // refers to another symbol → emits LOAD_SYM opcode
}
```

## What Semantic Analysis Does

### 1. Scope and Symbol Registration

Walk the AST. When an identifier is encountered:

- Check the current scope stack for an existing binding.
- If absent: create a new `SymbolDescriptor`, register it in `SymbolTable`,
  map the name → `SymbolId` in the current scope stack frame.
- If present: emit a redefinition error (Elise does not allow shadowing at this stage).

### 2. Known Function Validation

For built-ins (`.var`, `.const`, `.add`, `.mul`, `.get`, etc.) check:

- **Arity** — correct number of arguments.
- **Type rules** — e.g. `.add` requires numeric operands.
- **Semantic constraints** — `.const` requires a literal value, not a runtime ref.

### 3. Number Literal Resolution (Parser → Semantic)

The parser emits a raw `Number(String)` node. Semantic analysis resolves it:

```rust
fn resolve_number(raw: &str) -> LiteralValue {
    if raw.contains('.') { LiteralValue::Float(raw) }
    else                 { LiteralValue::Int(raw) }
}
```

This is intentionally deferred from the parser — distinguishing `Int` from `Float`
is a question of meaning, not syntax.

### 4. Constant Folding

Fold eagerly wherever all operands are known at compile time:

```
.const(PI 3.1415)
  → SymbolDescriptor { ty: Float, const_value: Some(Float(3.1415)) }

.var([x 12, y 20] .mul(PI .add(x y)))
  → .add(12, 20)        = Const(Int(32))
  → .mul(3.1415, 32)    = Const(Float(100.528))
  → x, y, result all get const_value populated
```

### 5. Using BindingTable Inside Semantic Analysis

`SemanticAnalyzer` holds a reference to the `DataBindingTable` produced by the Binder:

```rust
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    binding_table: DataBindingTable,
    scope_stack: Vec<Scope>,
}
```

This enables:

- **Type resolution from schema** — `.var(x .get(@data, "price")` looks up `[Field("price")]`
  in `BindingTable` → gets `ty: Float` → populates `SymbolDescriptor.ty` without inference.
- **Compile-time value folding for data refs** — when a data path is fully static
  (no dynamic indices) and the value is being assigned to a symbol, read the value
  from `BindingTable` and store it in `const_value`.
- **Type mismatch detection** — `.add(data.name, data.price)` is `String + Float` →
  semantic error before any bytecode is emitted.

---

## The Two Namespaces

|                         | SymbolTable                           | BindingTable                             |
| ----------------------- | ------------------------------------- | ---------------------------------------- |
| **Owns**                | Program symbols (vars, consts)        | Data descriptors (CSV rows/fields)       |
| **Keys**                | `SymbolId`                            | `Path` (Vec of `PathSegment`)            |
| **Values known at**     | Compile time (if foldable)            | Runtime (VM resolves against bound data) |
| **AAST representation** | `SymbolRef(SymbolId)` or `Const(...)` | `DataRef(Path)`                          |
| **Bytecode**            | `LOAD_SYM` / constant pool            | `LOAD_FIELD`                             |

They are separate but communicate through the AAST during semantic analysis.

---

## Constant Folding Rule for Data References

```
Expression context   → DataRef(path), resolved by VM at runtime
Assignment context   → check BindingTable now:
                         path fully static (no dynamic indices)?
                           yes → fold into const_value in SymbolDescriptor
                           no  → stays DataRef, resolved at runtime
```

### Example

```
.var(x .get(@data, 0, "name"))   → index 0 is a literal
                                   path [Index(0), Field("name")] is fully static
                                   BindingTable lookup → "Alice"
                                   x → { ty: String, const_value: Some(Str("Alice")) }
                                   emitter: constant pool entry, no LOAD_FIELD

.var(x .get(@data, i, "name"))   → i is a runtime symbol
                                   path is dynamic → cannot fold
                                   x → { ty: String, const_value: None }
                                   emitter: LOAD_SLOT @data → LOAD_FIELD [i, "name"]
```

For the last case we can apply loop unrolling optimization:

```
@data has 3 rows (known from BindingTable)

.var(x .get(@data, i, "name"))  in a loop over @data → unroll at compile time:

x = .get(@data, 0, "name")  → "Alice"
x = .get(@data, 1, "name")  → "Bob"  
x = .get(@data, 2, "name")  → "Charlie"

→ three Const values, no LOAD_FIELD at all
```

## Bytecode Emitter Signals (from AAST)

| AastValue       | Emitter action                                  |
| --------------- | ----------------------------------------------- |
| `Const(v)`      | Write `v` into constant pool, emit `LOAD_CONST` |
| `DataRef(path)` | Emit `LOAD_SLOT @data` + `LOAD_FIELD path`      |
| `SymbolRef(id)` | Emit `LOAD_SYM id`                              |

---

# Optimization strategy trouble

Since we have a second execution mode which allows us to execute bytecode against different kinds of
datasets, we can't use the same strategy for semantic analysis for run and exec modes because run
mode relies on DataBindingTable as well in order to perform optimizations like constant folding.

So we probably need to have some kind of branching for semantic analysis.

---

## .const
