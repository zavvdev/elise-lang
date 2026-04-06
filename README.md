------ IN PROGRESS ------

# Elise: Schema-Specialized Execution Model

/eˈliːs/ → pronounced like “eh-LEES”

[Grammar Rules](./GRAMMAR.md), [Todos](./TODO.md)

## Overview

Elise is a strongly-typed, schema-driven language for processing structured data efficiently. Programs are compiled against **schemas** rather than specific datasets, separating **code correctness** from **data correctness**. Execution can be optimized or unsafe depending on user requirements.

## File Types

Only `.csv` files are supported for now.

| Extension | Purpose                                             |
| --------- | --------------------------------------------------- |
| `.eli`    | Source code                                         |
| `.elt`    | Schema / type definitions for input data            |
| `.csv`    | Input data file                                     |
| `.elc`    | Compiled, schema-specialized bytecode with metadata |

## Execution Modes

### 1. Safe Direct Execution

```bash
elise --mode=run --source-code=sample.eli --data=data.csv --data-schema=data.elt
```

- Compiles in-memory (no `.elc` output)

- Performs full runtime validation of input data against schema

- Executes immediately

**Safety**: High

**Performance**: Medium

### 2. Compiled Execution (Optimized, Safe)

Step 1 — Build an executable

```bash
elise --mode=build --source-code=sample.eli --data-schema=data.elt --output=program.elc
```

Step 2 — Execute

```bash
elise --mode=exec --executable=program.elc --data=data.csv
```

- Uses precompiled schema-specialized bytecode

- Performs minimal runtime validation (structural + parsing checks)

- Executes optimized pipeline

**Safety**: High

**Performance**: High

### 3. Unsafe Execution (Maximum Performance)

Step 1 — Build an executable

```bash
elise --mode=build --source-code=sample.eli --data-schema=data.elt --output=program.elc
```

Step 2 — Execute

```bash
elise --mode=exec --executable=program.elc --data=data.csv --unsafe-assume-valid
```

- Requires precompiled .elc

- Skips runtime validation

- Executes fastest possible path

**Use case**: trusted, prevalidated, or immutable data

**Safety**: None ⚠️

**Performance**: Maximum

### 4. Validation-Only Step

```bash
elise --mode=validate --data=data.csv --data-schema=data.elt
```

- Full scan of data to ensure strict schema compliance

- Can be used before unsafe execution

### Technical features

#### Distributed Pipelines & Loop fusion

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
    x = n \* 2
    if x > 10 {
        sum += x
    }
}
```

#### Constant folding

Pseudo code:

```
add(mul(2,3), 4)
```

Compile-time result:

```
10
```

#### Compile-Time Execution

Allow functions to run during compilation.

Example (pseudo code):

```
primes(generate-primes(1000))
```

The compiler computes primes and embeds them during compilation so in runtime there is no computation at all.
