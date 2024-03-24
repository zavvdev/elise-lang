# elise-lang

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
@mul (
    @add (1 2) 
    @div (10, 2))

@let (x: i64 @mul(2 2)
    @if (@>= (x 20)
        @show (x)
        @show ("x is less than 20")))
 
@fn (my-fn [a b x] -> i64 
    @add (a b x))

@let (res: i64 @my-fn ()
    @show (res))
```
