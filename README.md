# elise-lang

### ToDo

- [x] number
- [x] print function
- [x] basic arithmetics
- [x] value binding
- [x] nil
- [x] boolean
- [x] strings
- [x] conditions

## @print

Prints result of expressions to console

Min number of arguments: _0_

Max number of arguments: _unlimited_

```
@print(n1..n)
```

## @println

Like `@print` but with `\n` at the end

Min number of arguments: _0_

Max number of arguments: _unlimited_

```
@println(n1..n)
```

## @add

Performs mathematical addition of expressions passed as arguments

Min number of arguments: _0_

Max number of arguments: _unlimited_

```
@add(1 2 3)
```

Result: `6`

## @sub

Performs mathematical subtraction of expressions passed as arguments

Min number of arguments: _1_

Max number of arguments: _unlimited_

```
@sub(10 5)
```

Result: `5`

## @mul

Performs mathematical multiplication of expressions passed as arguments

Min number of arguments: _0_

Max number of arguments: _unlimited_

```
@mul(10 5)
```

Result: `50`

## @div

Performs mathematical division of expressions passed as arguments

Min number of arguments: _1_

Max number of arguments: _unlimited_

```
@div(10 5)
```

Result: `2`

## @let 

Performs value binding.

Min number of arguments: _1_

Max number of arguments: _unlimited_

_First argument should always be a list literal ([]) with even number of elements where each non-even is an Identifier_

```
@let ([x 2, y 10]
    @println (@add (x y)))
```

Result: `12`

Also, `@let` function returns the result of the last expression:

```
@print (@let ([x 1] x))
```

Result: `1`

Keep in mind that binding won't be performed if there are no further expressions found for `@let` function:

```
@let ([x 1])
```

Result: `x` will not be bound to `1` because it makes no sense since there are no expressions found to use this binding.

## Boolean

Boolean literals: `true` and `false`

## String

Use `"` to create string literals. Example:

```
"Hello, World"
"Hello\nWorld"
```
