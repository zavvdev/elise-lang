# Specification

[General Terms](#general-terms), [Data Types](#data-types), [General Execution Model](#general-execution-model),
[Built-In Functions](#built-in-functions), [Grammar Rules](./GRAMMAR.md).

---

## General Terms

### Expression

An **Expression** is any instruction written in Elise. When you write code, you write a set of
expressions that are going to be [evaluated](#evaluation).

### Evaluation

Evaluation is the process of computing the [value](#value) that an [expression](#expression) produces.
Every expression, when evaluated, reduces to a single _value_ of one of Elise's 8 [data types](#data-types).

### Value

A _Value_ is the result of [evaluating](#evaluation) an [expression](#expression). It is a concrete
instance of one of Elise's 8 [data types](#data-types).

### Identifier

An identifier lets you create an alias for a [value](#value) of a specific [data type](#data-types), so you can
use this alias expression in order to evaluate the value it references. In other languages, this is also called a
"variable." You can bind an identifier to any data type.

It is a sequence of characters that conforms to this pattern:

```
Identifier = letter (letter | digit | '-' | '?' | '!' | '_')*
```

For example, create an alias for a primitive data type:

```
.let ([name "John", age 26, married false] ...)
```

so whenever you use `name` [expression](#expression), it evaluates to `"John"`, and so on.

```
.let ([
    address {
        "street" "Washington St.",
        "house"  23,
    },
    colors  ["red", "green", "blue"]
])

.let ([say-hi .fn([name]
                    .concat("Hello, " name))])
```

### Call

Call refers to the process of [Function](#function) invocation using following syntax:

```
.function-name(Expression*)
```

It takes an [identifier](#identifier) (`function-name` in this case) that must be previously
bound to an instance of the [Function](#function) data type. The argument [expressions](#expression)
inside the parentheses are [evaluated](#evaluation) first, and their resulting [values](#value) are bound
to the function's parameters. Then the function's body is evaluated with those bindings in
scope, and the whole _call_ expression evaluates to the [value](#value) the body produces.

---

## Data Types

Elise has 8 data types divided into 3 categories: **Primitive** data types, **Compound** data
types, and **Function**.

**LangType**: _LangPrimitiveType_ | _LangCompoundType_ | _Function_

**LangPrimitiveType**: _Int_ | _Float_ | _String_ | _Bool_ | _Null_

**LangCompoundType**: _List_ | _Dict_

### Int

64-bit signed or unsigned integer.
An _Int_ [expression](#expression) [evaluates](#evaluation) to itself.

```
2, 356, -42, 9999, 0
```

### Float

64-bit double-precision [IEEE-754](https://en.wikipedia.org/wiki/IEEE_754) floating point number.
All numbers written in scientific notation (e.g. `1e7`) are interpreted as _Float_.
A _Float_ [expression](#expression) [evaluates](#evaluation) to itself.

```
2.3, 5.0, -23.03, 1e8, -1.2E-3 
```

### String

[UTF-8](https://en.wikipedia.org/wiki/UTF-8) character sequence wrapped in double quotes.

```
"Elise"
```

A _String_ [expression](#expression) [evaluates](#evaluation) to itself.

### Bool

Boolean value. One of: `true` | `false`.

A _Bool_ [expression](#expression) [evaluates](#evaluation) to itself.

### Null

Special type that represents the absence of any [Value](#value).
```
null
```

A _Null_ [expression](#expression) [evaluates](#evaluation) to itself.

### List

A data structure that represents a collection of [values](#value).
A comma between _expressions_ is not required.

```
[Expression*]
```

Example:

```
[1, 1.2, "John", false, null, [1 2 3], { "name" "Jane" }, .mul(2 2), my-identifier]
```

Each value has its own index (Int) starting from `0` which you need to use in order to get the value.

```
.let ([colors ["red", "green", "blue"]]
        .get(colors, 0))
```

Here `.get(colors, 0)` [evaluates](#evaluation) to [value](#value) `"red"`.

A _List_ [expression](#expression) [evaluates](#evaluation) to itself.

### Dict

A collection of key-value pairs where each odd element is a _key_ and each even element is the
[value](#value) associated with that key. A key must always be a [String](#string); a value can be any
[expression](#expression).

```
{ (String Expression)* }
```

Example:

```
{
    "name" "John",
    "age" 26,
}
```

You can think of _Dict_ as the same thing as [List](#list), but instead of indexes that are
integers, _Dict_ has named indexes - its keys, which you can name however you want.

Therefore, this is also a valid _Dict_:

```
{ "name", "John", "age", 26 }
```
A _Dict_ [expression](#expression) [evaluates](#evaluation) to itself.

### Function

A _Function_ is a special [data type](#data-types) that lets you [evaluate](#evaluation) any number of
[expressions](#expression) in order to perform some task and produce a [value](#value) as its
result.

Other [data types](#data-types) evaluate to themselves — for example, the _String_ expression `"Hello"` evaluates
to `"Hello"`. A _Function_ is different: it lets you define custom logic for producing a result, so
when [called](#call), it doesn't evaluate to itself but to whatever value that logic returns.

Think of it like a custom [expression](#expression) that produces a _value_ you make it produce.

A _Function_ can be created with built-in [.fn](#.fn) function.

For example, using [.let](#.let):

```
.let ([
    my-function .fn([value]
                    .add(2 value))
])
```

Here, `.fn([value] ...)` evaluates to a _Function_ that is assigned to `my-function`
identifier. And now we can [call](#call) it by referencing `my-function` [identifier](#identifier).
In this case the result of evaluation is `4`.

Or we can use the `.fn` function providing a name for an identifier that will be automatically
created and bound to the _Function_ returned by `.fn`:

```
.fn (my-function [value]
        .add(2 value))
```

The result is the same as with `.let` but shorter.

#### Semantics

1. The last [expression](#expression) in the function body is the result of the function's [evaluation](#evaluation).
2. Referencing an identifier that was bound to a _Function_ (without [calling](#call) it) evaluates to
the _Function_ value itself, not to a resulting value.
3. Using [call](#call) syntax on identifier that references to _Function_ value evaluates to a resulting
value of that function.
3. Functions create their own scope stack record and destroy it once evaluated.
4. Functions are closures and capture parent scope stack record [identifiers](#identifier).

---

## General Execution Model

This section describes the idea of how execution model works in high level.
Any data structure names or anything else that refers to implementation might
not be precise in terms of the naming of the original implementation.

### Pre-compilation Stage

Elise syntax is designed to be _Code as Data_ where source is already shaped like an _AST_. Given that, lexing and parsing are combined into a single Parser step in order to reduce number of iterations and build _AST_ right away.

#### Source Code Parsing

This stage uses Parser in order to build _AST_ from source code written in `.eli` file.

#### Source Code Semantic Analysis

TODO

#### Data Schema Parsing

This stage uses Parser in order to build _AST_ from data schema code written in `.elt` file. Data
schema uses the same syntax as source code, so the same parser is used for this step as well.

#### Data Schema Resolution

Takes data schema _AST_ and resolves it into _ResolvedSchema_ data structure that simplifies
type definition addressing for specific data fields. This data structure can be constructed in
any way that simplifies type definition access for the specific data type (csv json) being resolved.

For example, for CSV it can be:

```rust
struct CsvColDescriptor {
    name: String,
    ty: DataType,
    opt: bool,
}

struct CsvResolvedSchema {
    row: Vec<CsvColDescriptor>,
}
```

#### Data Parsing

This stage uses a dedicated parser depending on data being parsed (csv, json). The result is a data
structure that can be constructed according to the data type being parsed. For example, for CSV it
can be an array of row descriptors.

#### Compile-time Data Binding

TODO

#### Runtime Data Binding

TODO

## Compilation Stage

TODO

## Runtime Stage

TODO

---

## Built-In Functions

Elise provides a set of built-in functions.

### .define

Defines a constant identifier in the current scope stack record. This function does not create its own scope
stack record; it defines its symbol directly in the current one. Consequently, `.define` does
not remove any scope stack records when its evaluation finishes, since it never added one.

It can be [called](#call) at any nesting level.

#### Semantics

```
.define (Identifier LangPrimitiveType)
```

1. Takes exactly 2 arguments.
2. The first argument is always an [identifier](#identifier).
3. The second argument is always a primitive type.
4. Never creates a new scope stack record.
5. Defines the symbol in the current scope stack record.
6. Does not remove any scope stack records.

#### Example

```
.define (PI 3.1415)
```

### .let

Provides a way to create lexical bindings of [values](#value) to [identifiers](#identifier).
These bindings are available only within the lexical context of the `.let`.

The result of a `.let` expression is the result of the last expression evaluated within its scope
stack record.

#### Semantics

```
.let ([(Identifier Expression)+] Expression+)
```

1. Takes a minimum of 2 arguments.
2. The first argument is always a [List](#list).
3. Odd items in that list are always [identifiers](#identifier).
4. Even items in that list are always [expressions](#expression), which must be [evaluated](#evaluation) first.
5. The result of evaluation is always the result of the last evaluated expression.
6. Creates its own scope stack record on entry.
7. Removes its own scope stack record when evaluation finishes.
8. Does not allow re-binding of symbols.
9. Can access the outer scope stack record.

#### Example

```eli
.let ([age 26, name "John"]
    .concat(
        name
        " is "
        .to-str(age)
        " years old"))
```

The evaluation result of this `.let` expression is `"John is 26 years old"`.

### .fn

Creates a [Function](#function) [value](#value). 

#### Semantics

```
.fn (Identifier? [Identifier*] Expression+)
```

1. If the _Identifier_ argument exists, it creates a new [identifier](#identifier) in the current scope
stack record that is bound to a _Function_ value returned from `.fn`.
2. `[Identifier*]` is a list of parameter names that the function accepts as an input (possibly empty).
3. `[Identifier*]` parameters are identifiers that exist only in the function's scope stack record.
4. The remaining arguments form the function body; the last expression evaluated is the result of
the _Function_ evaluation.
5. Creates a closure over the enclosing scope stack records.

#### Example

```
.fn (my-function [value]
        .add(2 value))

.let ([my-function2 .fn([value]
                        .add(2 value))] ...)
```

## CSV

Empty cells like `,,`, `,"",` or `,"  ",` are always treated as String type. They don't coerce to
Null. Null only equals to itself.
