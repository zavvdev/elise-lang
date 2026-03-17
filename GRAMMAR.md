# Grammar (Extended Backus–Naur Form)

_<n> - non-terminal symbol_

_\* - zero or more_

_+ - one or more_

_? - zero or one_

_(...)_ - grouping terminals/non-terminals

_/* ... */_ - semantic description (not a part of EBNF)

```
<expression> ::= <call> | <primitive> | <compound> | <identifier>

<call> ::= '.' <identifier> '(' <expression>* ')'

<primitive> ::= <number> | <string> | <boolean> | "null"
<compound> ::= <list> | <dictionary>

<list> ::= '[' <list-body>? ']'
<list-body> ::= <expression> <list-tail>? ','?
<list-tail> ::= ',' <expression> <list-tail>?

<dictionary> ::= '{' <dictionary-body>? '}'
<dictionary-body> ::= <dictionary-pair> <dictionary-tail>? ','?
<dictionary-tail> ::= ',' <dictionary-pair> <dictionary-tail>?
<dictionary-pair> ::= (<string> | <digit>+) ':' <expression>

<identifier> ::= <letter> (<letter> | <digit> | '-' | '?' | '!' | '_')*

<string> ::= '"' <string-char>* '"'
<string-char> ::= /* any character except " */

<boolean> ::= "true" | "false"

<number> ::= <normal-number> | <scientific-number>
<normal-number> ::= <integer> | <float>
<integer> ::= ('+' | '-')? <digit>+
<float> ::= <integer> '.' <digit>+
<scientific-number> ::= <normal-number> <scientific-exponent> <integer>

<digit> ::= '0'..'9'
<letter> ::= 'a'..'z' | 'A'..'Z'
<scientific-exponent> ::= 'e' | 'E'
```

## Examples

1. **<call>**

Call is a main part of the language and it represents an expression that should run
a specific amount of code written in this language in order to evaluate to some value.
In other words, it's a function call.

```
.function-name(...)
```

- You can pass anything as argument.

2. **<primitive>** and **<compound>**

These are 2 main data types that the language can operate with.

_Primitives_ are numbers (signed, unsigned, float, scientific), strings, boolean
and special value `null` that represents an absence of any value.

_Compounds_ are data structures that can comprise more that one primitive or other compound value.

Example of the **list** data structure:

```
[1, 2, -1, 4.3, "hello", false, null, [1, 2]]
```

- Comma between elements is required.

- Trailing comma is allowed.

Example of the **dictionary** data structure:

```
{ 
    "name": "John", 
    "age": 27, 
    "married": false, 
    "address": null,
    0: {
        1: "Some numeric key nested value"
    }
}
```

- Comma between pairs is required.

- Numeric keys allowed.

- Trailing comma is allowed.

3. **<identifier>**

It's a single value that can be evaluated to another value that has been bound to that identifier.
Can be also called an "alias" because we just labeling another value.
