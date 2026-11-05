# Specification

## .typedef semantics

Custom type definition function.

```
.typedef (Identifier :TypeDefinition)
```

Example:

```
.typedef (Product :Record<{
                    "id"    :Int
                    "price" :Str }>)
```

---

## .get semantics

Data access function.

```
.get (Data (Int | Str)*)
```

Example:

```
.get(@data)
.get(@data, 0, "price")
```

---
