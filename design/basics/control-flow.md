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
meaning it can return a value using the `out` keyword.

```
myFunction predicate: Binary -> {
    let value: Integer -- if predicate {
        out 1
    } else {
        out 2
    }
}
```

> note: grass uses the `out` keyword
> to return from the innermost scope.
> in the example above,
> `1` is returned from `if`, not `myFunction`
> [out](#out)

`if` without an `else` returns an `Optional`
of whatever you return from it

```
myFunction predicate: Binary -> {
    let a: Optional<Integer> -- if predicate {
        out 1
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
    let a: Optional<Integer> -- when predicate {
        Yea: 1,
        Idk: 67,
    }
}
```

you can also do rust-like pattern matching

```
myFunction predicate: Optional<Integer> -> {
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

use `out` to exit the loop.
the following will print "Hello, world!" 5 times

```
function -> {
    mutable x -- 0

    loop {
        if x >= 5: out

        printLine("Hello, world!")

        x -- x + 1
    }
}
```

a grass `loop` is also an expression,
and you can return a single value with the `out` keyword

```
function -> {
    mutable x -- 0

    let y: Integer -- loop {
        if x >= 5: out x

        printLine("Hello, world!")

        x -- x + 1
    }

    printLine(y) // prints "5"
}
```

a `loop` expression can also collect values into a `List`
with the `yield` keyword

```
function -> {
    mutable x -- 0

    let y: List<Integer> -- loop {
        if x >= 5: out

        printLine("Hello, world!")

        x -- x + 1
        yield x
    }

    printLine(y) // prints List("1, 2, 3, 4, 5")
}
```

you can skip the rest of the current iteration
by using the `skip` keyword.
this is the equivalent of `continue` in other languages

```
function -> {
    mutable x -- 0

    loop {
        if x % 2 = 0: skip

        printLine(x)

        x -- x + 1
    }
}
```

the function loops forever printing `x`, 
skipping all even values for `x`

### while

you can simplify a `loop` with a condition using `while`.
except from that, `while` is also an expression,
and acts exactly like a `loop`.
the following is equivalent to the last `loop` example

```
function -> {
    mutable x -- 0

    let y: List<Integer> -- while x < 5 {
        printLine("Hello, world!")

        x -- x + 1
        yield x
    }

    printLine(y) // prints List("1, 2, 3, 4, 5")
}
```

### for

you can iterate through an `Iterator` by using a `for` loop

```
function numbers: Iterator<Integer> -> {
    for numbers: number {
        printLine(number)
    }
}
```

`for` loops are also expressions

```
function numbers: Iterator<Integer> -> {
    let a -- for numbers: number {
        printLine(number)

        if number % 2: yield number
    }

    printLine(a) // prints even elements of `numbers`
}
```

#### ranges

grass doesn't have a `for (initializer; predicate; incrementer)` loop.
if you want to, say, iterate through 1 to 5,
you can make a `Range<Integer>`, which implements `Iterator<Integer>`,
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

if you just need to iterate a number of times,
but you don't really need the value/index,
then you can omit the name for the value/index.
the following code prints "Hello" five times

```
function -> {
    for 1..5 {
        printLine("Hello")
    }
}
```

## out

grass uses the `out` keyword to:

- return a value from a scope
- skip everything between the `out` and the closest `}`

this is unlike other languages,
where `return` is only used
to exit out of the innermost function.
to do the same in grass, 
you use the `out from` syntax

```
runApp -> {
    if true {
        printLine("Hi!")
        out from runApp
    }

    printLine("Bye!")
}
```

in the example above, 
"Hi!" would print, but "Bye!" wouldn't,
since we returned from `runApp`.

returning out of functions is such a common practice,
that grass has a special syntax for it
to bring back the convenience in other languages
of immediately exiting functions
by using `return`

```
runApp -> {
    if true {
        printLine("Hi!")
        out! // equivalent to `out from runApp`
    }

    printLine("Bye!")
}
```

you can use the `out from` syntax
with a return value

```
getNumber likesSix: Binary -> Integer {
    if likesSix {
        out 6 from getNumber
    }

    out 7
}
```

you can also use `out!` with a return value

```
getNumber likesSix: Binary -> Integer {
    if likesSix {
        out! 6
    }

    out 7
}
```

you can use the type of scope you're in
to indicate what scope you wanna exit

```
runApp -> {
    mutable i = 0

    loop {
        if i >= 5 {
            out from loop
        }

        printLine(i)
    }
}
```

in the example above,
we need to specify that we're returning from `loop`,
since if we just wrote `out`,
we'd just be returning from the `if` scope,
and nothing would really happen

we could use the `if` shorthand tho
to save us from doing a `out from`,
since it doesn't introduce a new scope

```
runApp -> {
    mutable i = 0

    loop {
        if i >= 5: out

        printLine(i)
    }
}
```

`out from` will exit
out of the innermost identifier match

```
runApp -> {
    mutable i = 0

    loop {
        loop {
            if i >= 5 {
                out from loop // this
            }

            printLine(i)
        } // will exit you out of this
    }
}
```

if you want to exit out of a scope
with the same identifier as a more inner scope,
then name the scope you wanna exit

```
runApp -> {
    mutable i = 0

    loop outerLoop {
        loop {
            if i >= 5 {
                out from outerLoop // this
            }

            printLine(i)
        }
    } // will exit you out of this
}
```

you can use the same syntax
to name regular non-control flow scopes

```
function predicate: Binary -> {
    let a -- scope {
        if predicate {
            out 1 from scope
        }

        out 2
    }
}
```
