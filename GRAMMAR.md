# Grammar (Extended Backus–Naur Form)

_<n> - non-terminal symbol_

_\* - zero or more_

_+ - one or more_

_? - zero or one_

_(...)_ - grouping terminals/non-terminals

_/* ... */_ - semantic description (not a part of EBNF)

_separator_ - either comma, whitespace, new line or tab. Rules for using a specific type
of separators are not enforced and they are literally decorative. It doesn't matter which
separator you use and how much of them, you just have to use it to separate expressions.

```
<expression> ::= <call> | <primitive> | <compound> | <identifier>

<call> ::= '.'<no-separator><identifier>? '(' <expression>* ')'

<primitive> ::= <number> | <string> | <boolean> | "null"
<compound> ::= <list> | <dictionary>

<list> ::= '[' <expression>* ']'
<dictionary> ::= '{' (<string> <expression>)* '}'

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
<no-separator> ::= /* prohibition of using separator */
```

## Examples

### 1. call

Call is a main part of the language and it represents an expression that should run
a specific amount of code written in this language in order to evaluate to some value.
In other words, it's a function call.

```
.function-name(...)
```

It can also be anonymous. For example if you want to pass it as an argument.

```
.([x] .add(x 1))
```

- You can pass anything as an argument.

### 2. primitive and compound

_Primitives_ are numbers (signed, unsigned, float, scientific), strings, boolean
and special value `null` that represents an absence of any value.

_Compounds_ are data structures that can comprise more that one primitive or other compound value.

Example of the **list** data structure:

```
[1, 2, -1, 4.3, "hello", false, null, [1, 2]]
```

- Separator between each element is not required.

Example of the **dictionary** data structure:

```
{ 
    "name" "John", 
    "age" 27, 
    "married" false, 
    "address" null,
    "0" {
        "1" "Some numeric key nested value"
    }
}
```

- Separator between each pair is not required.

- Keys are always strings.

### 3. identifier

It's a single value that can be evaluated to another value that has been bound to that identifier.
Can be also called an "alias" because we just labeling another value. In other languages it can be called "variable".
