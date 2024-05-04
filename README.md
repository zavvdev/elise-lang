# elise-lang

Example:

```
@mul (
    @add (1 2)
    @div (10 2))

@let ([x : num, @mul(2 2)]
    @if (@>= (x 20)
        @print (x)
        @print ("x is less than 20")))

@fn (my-fn [a b x] -> num
    @add (a b x))

@let ([res : num, @my-fn ()]
    @println (res))
```
