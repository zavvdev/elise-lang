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

## @greatr

Returns non-nil if nums are in monotonically decreasing order, otherwise false.

```
@greatr(x)
@greatr(x y)
@greatr(x y & more)
```

## @greatr-eq

Returns non-nil if nums are in monotonically non-increasing order, otherwise false.

```
@greatr-eq(x)
@greatr-eq(x y)
@greatr-eq(x y & more)
```

## @less

Returns non-nil if nums are in monotonically increasing order, otherwise false.

```
@less(x)
@less(x y)
@less(x y & more)
```

## @less-eq

Returns non-nil if nums are in monotonically non-decreasing order, otherwise false.

```
@less-eq(x)
@less-eq(x y)
@less-eq(x y & more)
```

## @eq

Equality. Returns true if x equals y, false if not.

```
@eq(x)
@eq(x y)
@eq(x y & more)
```

## @not

Returns true if x is logical false, false otherwise.

```
@not(x)
```

## @not-eq

Same as `@not(@eq(x))`.

```
@not-eq(x)
@not-eq(x y)
@not-eq(x y & more)
```

## @and

Evaluates exprs one at a time, from left to right. If a form returns logical false (nil or false), and returns that value and doesn't evaluate any of the other expressions, otherwise it returns the value of the last expr. `@and()` returns true.

```
@and()
@and(x)
@and(x & next)
```

## @or

Evaluates exprs one at a time, from left to right. If a form returns a logical true value, or returns that value and doesn't evaluate any of the other expressions, otherwise it returns the value of the last expression. `@or()` returns nil.

```
@or()
@or(x)
@or(x & next)
```

## @bool 

Coerce to boolean. Everything except `false` and `nil` is true in boolean context.

```
@bool(x)
```

## @if 

Evaluates the first argument and performs boolean coercion of the result. If it results to true evaluates the second argument and returns result. Otherwise, evaluates the third argument and returns the result. If the third argument is absent - returns `nil`.

```
@if (condition then)
@if (condition then else)
```
