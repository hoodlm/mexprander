This is an ugly toy parser/compiler for expanding math expressions.

It expands a compound math expression like:

```
1 + 2 + 4 + 8
```

into a series of assignments from binary operations, like

```
a = 1 + 2
b = a + 4
c = b + 8
c
```

The output can be piped into a calculator like `bc` for evaluation/verification.


### Supports

* Integers
* Addition/Multiplication

It does not yet have a conventionally correct order-of-operations; operators are evaluated left-to-right.
