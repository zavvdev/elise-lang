# Elise: Bytecode-compiled DSL for typed data transformation

[![XERJ Corpus Hub contributor](https://hub.xerj.org/contributors/zavvdev/badge.svg)](https://hub.xerj.org/contributors/zavvdev)

/eˈliːs/ → pronounced like “eh-LEES”

[Grammar Rules](./GRAMMAR.md), [Specification](./SPECIFICATION.md), [Todos](./TODO.md)

## Overview

A domain-specific language for schema-driven data processing, with ahead-of-time compilation to type-optimized bytecode. Write once, run against any conforming dataset.

The main idea is simple: if you know the types of your dataset, you can describe them in a schema, and Elise uses that schema to produce optimized bytecode. These optimizations are possible because the compiler knows all the types at compile time. You can then use that bytecode on any other dataset, as long as it matches the original schema that was used to generate the bytecode.

## File Types

> **_NOTE:_** Only `.csv` data files are supported for now.

| Extension | Purpose                                  |
| --------- | ---------------------------------------- |
| `.eli`    | Source code                              |
| `.elt`    | Schema / type definitions for input data |
| `.csv`    | Input data file                          |
| `.elb`    | Generated file with compiled bytecode    |

## Execution Modes

### 1. Safe Direct Execution

```bash
elise --mode=run --source-code=sample.eli --data=data.csv --data-schema=data.elt
```

- Compiles in-memory (no `.elb` output)

- Performs full runtime validation of input data against schema

- Executes immediately

### 2. Unsafe Execution

Step 1 — Build an executable

```bash
elise --mode=build --source-code=sample.eli --data-schema=data.elt --output=program.elb
```

Step 2 — Execute

```bash
elise --mode=exec --executable=program.elb --data=data.csv
```

- Requires precompiled .elb

- Data must comply with schema that was used for building executable

- Skips runtime validation

- Executes fastest possible path

**Use case**: trusted, prevalidated data

### 3. Validation-Only Step

```bash
elise --mode=validate --data=data.csv --data-schema=data.elt
```

- Full scan of data to ensure strict schema compliance

- Can be used before unsafe execution
