# Semantics

## Data Types

```
LangPrimitiveType ::= Int | Float | String | Bool | Null

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

1. Has only 2 arguments
2. First argument is always an identifier
3. Second argument is always primitive type
4. Never creates a new scope stack record
5. Defines symbols in the current scope stack
6. Does not remove any scope stack entries

#### Example

```
.define (PI 3.1415)
```

### .let

Provides a way to create lexical bindings of data structures to symbols.
The binding, and therefore the ability to resolve the binding,
is available only within the lexical context of the let.

The result of the **.let** expression evaluation is the result of the last
expression inside the **.let** scope.

#### Semantics

```
.let ([(Identifier Expression)+] Expression+)
```

1. Min 2 arguments
2. First argument is always a List
3. Odd items in the list are always identifiers
4. Even items in the list are always expressions that must be evaluated first
5. The result of evaluation is always a result of the last evaluated expression
6. Creates its own scope stack when enters
7. Removes its own scope stack when evaluation finishes
8. Does not allow symbol re-bindings
9. Can access outer scope

#### Example

```
.let ([age 26, name "John"]
    .concat(
        "John is "
        .to-str(age)
        "years old"))
```

The evaluation result of this **.let** expression is "John is 26 years old".
