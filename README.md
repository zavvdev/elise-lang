# elise-lang

A deterministic dataflow programming language with compile-time optimization.

[Grammar Rules](./GRAMMAR.md), [Todos](./TODO.md)

## Main Idea

### Each value is immutable

In order to update the value you need to return a new one.

### No side effects

Each function is pure and deterministic which allows to perform compile-time optimizations.

### Distributed Pipelines & Loop fusion

For sequential data transformations compiler can decide whether to run in in single of multi thread.

Example (pseudo code):

```
pipe big_dataset
    map(parse)
    map(transform)
    reduce(sum)
```

We can also optimize sequential calls by fusing it into one loop.

Before (pseudo code):

```
pipe numbers
    map(mul(_, 2))
    filter(.gt(_, 10))
    sum
```

After (pseudo code):

```
sum = 0
for n in numbers {
    x = n * 2
    if x > 10 {
        sum += x
    }
}
```

### Constant folding

Pseudo code:

```
add(mul(2,3), 4)
```

Compile-time result:

```
10
```

### Compile-Time Execution

Allow functions to run during compilation.

Example (pseudo code):

```
primes(generate-primes(1000))
```

The compiler computes primes and embeds them during compilation so in runtime there is no computation at all.
