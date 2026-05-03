# yielding

## yield

in [control flow](control-flow.md),
we saw an example of using the `yield` keyword to yield multiple values.
however, `yield` is not exlusive to loops.
any scope can use the `yield` keyword to yield values

```
function -> {
    let numbers: List<Integer> -- {
        yield 1
        yield 2
        yield 42
    }

    printLine(numbers) // prints "List(1, 2, 42)"
}
```

you can also do this in a function scope

```
function -> List<Integer> {
    yield 1
    yield 2
    yield 42
}

runApp -> {
    printLine(function()) // prints "List(1, 2, 42)"
}
```

## defer

the `defer` keyword allows the scope you're yielding values from
to lazily execute.
this means the code inside the block will not execute
until you iterate through the `Iterator` it returned

```
function -> {
    let numbers: Iterator<Integer> -- defer {
        printLine("hey")
        yield 1

        printLine("hello")
        yield 42

        printLine("a b c d")
    }

    // nothing is printed yet...

    for numbers: number {
        printLine(number)
    }

    // printed:
    // hey
    // 1
    // hello
    // 42
    // a b c d
}
```

a deferred function

```
getIterator -> Iterator<Integer> defer {
    printLine("hey")
    yield 1
    
    printLine("hello")
    yield 42
    
    printLine("a b c d")
}

runApp -> {
    let iterator -- getIterator() // does not print anything

    for iterator: number {
        printLine(number)
    }

    // printed:
    // hey
    // 1
    // hello
    // 42
    // a b c d
}
```

this is how you can implement your own `start..end`

```
inclusiveRange start: Integer, end: Integer -> Iterator<Integer> {
    mutable index -- start

    return while index <= end {
        yield index

        index -- index + 1
    }
}

runApp -> {
    for inclusiveRange(start: 1, end: 5) { number
        printLine(number)
    }

    // prints:
    // 1
    // 2
    // 3
    // 4
    // 5
}
```

## yield all

use `yield all` to yield all the elements of an iterator

```
getNumbers -> Iterator<Integer> defer {
    yield all 3..5

    yield 6
    yield 7

    yield all for 9..11: x {
        yield x
    }
}

runApp -> {
    for getNumbers(): number {
        printLine(number)
    }

    // prints:
    // 3
    // 4
    // 5
    // 6
    // 7
    // 9
    // 10
    // 11
}
```
