# elise-lang

A deterministic dataflow pipeline programming language with compile-time optimization.

## TODO

- [x] Add foundation for executing binary with arguments:
  - [x] --file=<file> - specify file to execute
  - [x] --print-bytecode - print bytecode of the executed file

- [x] Add library crate which exposes interpreter entry point

- [x] Use library crate in the binary crate

- [x] Add number parsing (positive, negative, float, scientific notation)

- [x] Add string parsing ("<UTF-8 string>")

- [ ] Add parsing lists -- IN PROGRESS

- [ ] Add tests for lists

- [ ] Add parsing dictionaries

- [ ] Add tests for dicts

- [ ] Add parsing identifiers

    - [ ] Add parsing bool

    - [ ] Add parsing nil

- [ ] Add tests for identifiers

- [ ] Add parsing function calls

- [ ] Add tests for function calls

- [ ] Add static analysis module

- [ ] Create bytecode generator for producing bytecode from AST

- [ ] Create virtual machine for executing bytecode

## Idea

### Deterministic Execution

Pure by default. No mutation.

Example:

```
.fn(perim [width, height]
    .mul(width, height))
```

The compiler can then guarantee deterministic output because each value is immutable.

### Reactive Language

Pipelines can become reactive graphs.

Example:

```
.define(width 200)
.define(height 300)

.define(p (.perim(width height)))
.define(text (.concat("Perimeter: ", p)))
```

When width changes → everything updates automatically.

### Distributed Pipelines

If `.pipe` is core, we can make execution distributed automatically.

Example:

```
.pipe(big_dataset
    .map(parse)
    .map(transform)
    .reduce(sum))
```

Compiler decides whether to run in in single of multi thread.

### Compile-Time Optimized Pipelines

In most languages a pipeline like this:

```
.pipe(
    data,
    parse,
    normalize,
    filter-valid,
    sum
)
```

is executed as a chain of function calls.

Conceptually:

```
.sum(.filter-valid(.normalize(.parse(data))))
```

But a pipeline-oriented compiler can instead treat the whole pipeline as one computation graph.

Instead of executing functions separately, the compiler:

1. builds an AST

2. converts it into a dataflow graph

3. optimizes the entire pipeline

4. generates a single fused function

Example transformation.

User code:

```
.pipe(
    numbers,
    .map(.mul(_, 2)),
    .filter(.gt(_, 10)),
    sum
)
```

Naive execution:

```
numbers
  -> map
  -> filter
  -> sum
```

Optimized execution (fused loop):

```
sum = 0
for n in numbers {
    x = n * 2
    if x > 10 {
        sum += x
    }
}
```

No intermediate arrays. This technique is called loop fusion or stream fusion. The compiler decides how to run the pipeline.

Possible modes:

- Single thread

```
for item in data
```

- Multi-thread

```
parallel_for chunk in data
```

The user writes the same code.

### Constant folding

Code:

```
.add(.mul(2,3), 4)
```

Compile-time result:

```
10
```

### Compile-Time Execution

Allow functions to run during compilation.

Example:

```
.primes(.generate-primes(1000))
```

The compiler computes primes and embeds them during compilation so in runtime there is no computation at all.

### Built-In Incremental Execution

If input changes slightly the runtime recomputes only changed parts.
