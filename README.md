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

## Boolean

Boolean literals: `true` and `false`

## String

Use `"` to create string literals. Example:

```
"Hello, World"
"Hello\nWorld"
```

## Number

Number literals: `1`, `1.2`

## @print

Prints result of expressions to console

```
@print(& more)
```

## @println

Like `@print` but with `\n` at the end

```
@println(& more)
```

## @add

Returns the sum of numbers. `@add()` returns 0

```
@add()
@add(x)
@add(x y)
@add(x y & more)
```

## @sub

If no ys are supplied, returns the negation of x, else subtracts the ys from x and returns the result

```
@sub(x)
@sub(x y)
@sub(x y & more)
```

## @mul

Returns the product of nums. `@mul()` returns 1

```
@mul()
@mul(x)
@mul(x y)
@mul(x y & more)
```

## @div

If no denominators are supplied, returns result of `1/numerator`, else returns numerator divided by all of the denominators

```
@div(x)
@div(x y)
@div(x y & more)
```

## @let 

Evaluates the exprs in a lexical context in which the symbols in the binding-forms are bound to their respective init-exprs 

```
@let ([binding-form init-expr] exprs*)
```

Example:

```
@let ([x 2, y 10]
    @println (@add(x y)))
```

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
