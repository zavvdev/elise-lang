# elise-lang

```
@mul (
    @add (1 2)
    @div (10 2))

@let (x: int @mul(2 2)
    @if (@>= (x 20)
        @show (x)
        @show ("x is less than 20")))

@fn (my-fn [a b x] -> int
    @add (a b x))

@let (res: int @my-fn ()
    @show (res))
```

Source Code -> Lexical Analysis (Lexer) -> Syntax Analysis (Parser) -> Semantic Analysis (SA) -> Execution (Interpreter/Compiler) -> Program output

### Lexer

Converts text into meaningful lexical tokens belonging to categories defined by a "lexer" program.

Consider this expression in the C programming language: `x = a + b * 2;`

The lexical analysis of this expression yields the following sequence of tokens:

```
[(identifier, x), (operator, =), (identifier, a), (operator, +), (identifier, b), (operator, *), (literal, 2), (separator, ;)]
```

A lexical analyzer generally does nothing with combinations of tokens, a task left for a parser. For example, a typical lexical analyzer recognizes parentheses as tokens, but does nothing to ensure that each "(" is matched with a ")".

## Parser

Takes input data from lexer and builds an Abstract Syntax Tree data structure or other hierarchical structure, giving a structural representation of the input while checking for correct syntax.

```
[
    {
        kind: KnownFunction,
        value: FnAdd,
        branches: [
            {
                kind: KnownFunction,
                value: FnMul,
                branches: [
                    {
                        kind: RawValue,
                        value: Integer(2),
                        branches: [],
                    },
                    {
                        kind: RawValue,
                        value: Integer(3),
                        branches: [],
                    },
                ],
            },
            {
                kind: RawValue,
                value: Integer(4),
                branches: [],
            },
        ],
    },
]
```

## Semantic Analyser

Semantic Analysis it's a process to help us determine whether a program makes sense, and that it has meaning, according to a language definition. For example, the variables that are used in source code are defined, the function is in some scope or uses correct arguments etc.
