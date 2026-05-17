# Elise: Schema-Specialized Execution Model

/eˈliːs/ → pronounced like “eh-LEES”

[Grammar Rules](./GRAMMAR.md), [Todos](./TODO.md), [Documentation](./DOCUMENTATION.md)

## Overview

Elise is a strongly-typed, schema-driven language for processing structured data efficiently. Programs are compiled against **schemas** rather than specific datasets, separating **code correctness** from **data correctness**.

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

### 2. Unsafe Execution (Maximum Performance)

Step 1 — Build an executable

```bash
elise --mode=build --source-code=sample.eli --data-schema=data.elt --output=program.elc
```

Step 2 — Execute

```bash
elise --mode=exec --executable=program.elc --data=data.csv
```

- Requires precompiled .elc

- Skips runtime validation

- Executes fastest possible path

**Use case**: trusted, prevalidated data

**Safety**: None ⚠️

**Performance**: Maximum

### 3. Validation-Only Step

```bash
elise --mode=validate --data=data.csv --data-schema=data.elt
```

- Full scan of data to ensure strict schema compliance

- Can be used before unsafe execution
