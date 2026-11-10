# Specification

## @data slot

A reference to the data provided via `--data` CLI flag.

## .typedef function

Creates a custom type definition.

Example:

```
.typedef (Product :Record<{
                    "id"    :Int
                    "price" :Str }>)

.typedef (Data :List<:Product>)
```

---

## .get function

Performs data access using its arguments as access path.
Use integers for accessing List indexes, and strings for
accessing Record fields.

Example:

```
.get(@data)
.get(@data, 0, "price")
```

---

## .add function

Performs mathematical addition of its arguments.
Each argument must be an expression that resolves
to either Int or Float data type.

Example:

```
.add (1)
.add (2, 3.4, 12)
.add (2, .get(@data, 0, "id"))
```

---
