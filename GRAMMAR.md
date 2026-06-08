# Grammar (Extended Backus–Naur Form)

_<n> - non-terminal symbol_

_\* - zero or more_

_+ - one or more_

_? - zero or one_

_(...)_ - grouping terminals/non-terminals

_/* ... */_ - semantic description (not a part of EBNF)

```
<expression> ::= <call> | <primitive> | <compound> | <identifier> | <slot>

<call> ::= '.' <identifier>? '(' <expression>* ')'

<primitive> ::= <number> | <string> | <boolean> | "null"
<compound> ::= <list> | <dictionary>

<list> ::= '[' <expression>* ']'
<dictionary> ::= '{' (<string> <expression>)* '}'

<identifier> ::= <letter> (<letter> | <digit> | '-' | '?' | '!' | '_')*

<slot> ::= '@' <identifier>

<string> ::= '"' <string-char>* '"'
<string-char> ::= /* any character except " (double quote) */

<boolean> ::= "true" | "false"

<number> ::= <int> <float-tail>? <scient-tail>?
<float-tail> ::= '.' <digit>+
<scient-tail> ::= <scient-expon> <int>

<digit> ::= '0'..'9'
<int> ::= ('+' | '-')? <digit>+
<letter> ::= 'a'..'z' | 'A'..'Z'
<scient-expon> ::= 'e' | 'E'
```
