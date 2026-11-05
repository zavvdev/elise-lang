# Specification

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

Performs data access using provided path.
Use integers for accessing List indexes,
and strings for accessing Record fields.

Example:

```
.get(@data)
.get(@data, 0, "price")
```

---
