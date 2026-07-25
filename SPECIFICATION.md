# Data Types

Elise language has 8 data types which are divided into 3 categories: **Primitive** data types, **Compound** data types and **Function*.

**LangType**: _LangPrimitiveType_, _LangCompoundType_, _Function_

**LangPrimitiveType**: _Int_, _Float_, _String_, _Bool_, _Null_

**LangCompoundType**: _List_, _Dict_

## Int

64 bit signed and unsigned integer.

## Float

64 bit double precision IEEE-754 floating point number.
All numbers written in scientific (1e7) notations are interpreted as Float.

# Identifier

Identifiers allow you to create an alias for some data type by creating a binding between them, so
you can get this data type later by evaluating the identifier.

It is a sequence of characters that conforms to this pattern:

```
<identifier> ::= <letter> (<letter> | <digit> | '-' | '?' | '!' | '_')*
```

and evaluates into something that it was bound to. In other language it's also called a "variable".
You can alias any data type with identifier.

For example, create an alias for primitive data types:

```
.let ([name "John", age 26, married false] ...)
```

so whenever you refer to the _name_, it evaluates to "John", and so on.

Compound data types:

```
.let ([
    address {
        "street" "Washington St.",
        "house"  23,
    },
    colors  ["red", "green", "blue"]
])
```

and even functions:

```
.let ([say-hi .([name] 
                    .concat("Hello, " name))])
```

# Expression

Expressions are language instructions that evaluate to some data type. Everything in Elise language is an expression.

# Functions

## .define

Allows to define a constant identifier in a current scope. This function must not create
its own scope stack record, but rather define its symbol inside the current scope. Therefore,
at the end of the **.define** scope it must not remove any scope stack records.

It's allowed to call this function at any nesting level.

### Semantics

```
.define (Identifier LangPrimitiveType)
```

1. Has only 2 arguments
2. First argument is always an identifier
3. Second argument is always primitive type
4. Never creates a new scope stack record
5. Defines symbols in the current scope stack
6. Does not remove any scope stack entries

### Example

```
.define (PI 3.1415)
```

## .let

Provides a way to create lexical bindings of data structures to symbols.
The binding, and therefore the ability to resolve the binding,
is available only within the lexical context of the let.

The result of the **.let** expression evaluation is the result of the last
expression inside the **.let** scope.

### Semantics

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

### Example

```eli
.let ([age 26, name "John"]
    .concat(
        name
        " is "
        .to-str(age)
        " years old"))
```

The evaluation result of this **.let** expression is "John is 26 years old".
