.fn (fact [n]
    .if(.eq(n, 0)
      1
      .mul(n, .fact(.sub(n 1)))))

.println(.fact(3))
