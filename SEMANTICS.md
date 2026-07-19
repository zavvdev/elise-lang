# Semantics

## Data Types

```
LangPrimitiveType ::= Int | Float

LangType ::= LangPrimitiveType
```

## Functions

### .define

Allows to define a constant identifier in a current scope. This function must not create
its own scope stack record, but rather define its symbol inside the current scope. Therefore,
at the end of the **.define** scope it must not remove any scope stack records.

It's allowed to call this function at any nesting level.

#### Semantics

```
.define (Identifier LangPrimitiveType)
```
