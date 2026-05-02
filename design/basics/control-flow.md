# control flow

## operators

### relational

| expression   | C-equivalent |
| ------------ | ------------ |
| `a < b`      | `a < b`      |
| `a <= b`     | `a <= b`     |
| `a = b`      | `a == b`     |
| `a >= b`     | `a >= b`     |
| `a > b`      | `a > b`      |

> note: any of these operators usually return a `Binary` in grass
> when used on primitive types.
> however, some primitives, like `Fraction`, return `Ternary` instead
> due to how they're implemented
>
> | expression                | return |
> | ------------------------- | ------ |
> | `0.0 < 1.0`               | `Yea`  |
> | `0.0 > 1.0`               | `Nah`  |
> | `NotANumber = NotANumber` | `Idk`  |

### logical

| expression | C-equivalent  |
| ---------- | ------------- |
| `!a`       | `!a`          |
| `a & b`    | `a && b`      |
| `a \| b`   | `a \|\| b`    |
| `a !& b`   | `!(a && b)`   |
| `a !\| b`  | `!(a \|\| b)` |

## conditional expressions

### if-else

here is how you do an `if` in grass.
remember that only the `Binary` type can be used as an `if` predicate

```
function predicate: Binary -> {
    if predicate {
      printLine("predicate is Yea"!)
    }
}
```

and here is an `if-else`

```
function predicate: Binary -> {
    if predicate -> {
        printLine("predicate is Nah!")
    } else {
        printLine("predicate is Yea!")
    }
}
```

`if-else` is an expression, 
meaning it can return a value using the `then` keyword.

```
myFunction predicate: Binary -> {
    let value: Integer -- if predicate {
        then 1
    } else {
        then 2
    }
}
```

`if` without an `else` returns an `Optional`
of whatever you return from it

```
myFunction predicate: Binary -> {
    let a: Optional(Integer) -- if predicate {
        then 1
    }
}
```

both `if` and `if-else` expressions 
that contain nothing but return values
can use the following shorthand

```
myFunction predicate: Binary -> {
    let a -- if predicate: 1
    let b -- if predicate: 1; else: 2
}
```

### when expressions

you can do C#-like `switch` expressions in grass

```
myFunction predicate: Ternary -> {
    when predicate {
        Yea: printLine("Yea!"),
        Nah: printLine("Nah!"),
        Idk: printLine("Idk..."),
    }

    let a -- when predicate {
        Yea: 1,
        Nah: 2,
        Idk: 67,
    }
}
```

and just like with `if` expressions,
unhandled cases would make the type `Optional`

```
myFunction predicate: Ternary -> {
    let a: Optional(Integer) -- when predicate {
        Yea: 1,
        Idk: 67,
    }
}
```

you can also do rust-like pattern matching

```
myFunction predicate: Optional(Integer) -> {
    let a -- when predicate {
        Some(value): value * 2,
        None: 0,
    }
}
```

## loops

### loop

here is a basic `loop` that prints "Hello, world!" forever

```
function -> {
    loop {
        printLine("Hello, world!")
    }
}
```

use the `break` keyword to exit the loop.
the following will print "Hello, world!" 5 times

```
function -> {
    mutable x -- 0

    loop {
        if x >= 5 {
            break
        }

        printLine("Hello, world!")

        x -- x + 1
    }
}
```

a grass `loop` is also an expression,
and you can return a single value with the `break` keyword

```
function -> {
    mutable x -- 0

    let y: Integer -- loop {
        if x >= 5 {
            break x
        }

        printLine("Hello, world!")

        x -- x + 1
    }

    printLine(y) // prints "5"
}
```

a `loop` expression can also collect values into a `List`
with the `then` keyword

```
function -> {
    mutable x -- 0

    let y: List(Integer) -- loop {
        if x >= 5 {
            break
        }

        printLine("Hello, world!")

        x -- x + 1
        then x
    }

    printLine(y) // prints List("1, 2, 3, 4, 5")
}
```

### while

you can simplify a `loop` with a condition using `while`.
except from that, `while` is also an expression,
and acts exactly like a `loop`.
the following is equivalent to the last `loop` example

```
function -> {
    mutable x -- 0

    let y: List(Integer) -- while x < 5 {
        printLine("Hello, world!")

        x -- x + 1
        then x
    }

    printLine(y) // prints List("1, 2, 3, 4, 5")
}
```

### for

you can iterate through an `Iterator` by using a `for` loop

```
function numbers: Iterator(Integer) -> {
    for numbers: number {
        printLine(number)
    }
}
```

`for` loops are also expressions

```
function numbers: Iterator(Integer) -> {
    let a -- for numbers: number {
        printLine(number)

        if number % 2: then number
    }

    printLine(a) // prints even elements of `numbers`
}
```

#### ranges

grass doesn't have a `for (initializer; predicate; incrementer)` loop.
if you want to, say, iterate through 1 to 5,
you can make a `Range(Integer)`, which implements `Iterator(Integer)`,
and use it on a `for` loop

```
function -> {
    for 1..5: index {
        printLine(number)
    }
}
```

| expression | interval notation |
| ---------- | ----------------- |
| `a..b`     | `[a, b]`          |
| `a >..b`   | `(a, b]`          |
| `a..< b`   | `[a, b)`          |
| `a >..< b` | `(a, b)`          |

> [interval notation](https://en.wikipedia.org/wiki/Interval_(mathematics))
