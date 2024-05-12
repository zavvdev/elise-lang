# elise-lang

Example:

```
@mul (
    @add (1 2)
    @div (10 2))

@let ([x @mul(2 2)]
    @if (@>= (x 20)
        @print (x)
        @print ("x is less than 20")))

@fn (my-fn [a b x]
    @add (a b x))

@let ([res @my-fn()]
    @println (res))
```

### ToDo

- [x] number
- [x] print function
- [x] basic arithmetics
- [x] value binding
- [ ] strings
- [ ] boolean
- [ ] conditions
