# data types

grass is statically typed. even if you omit the type, 
it is automatically inferred and assigned at compile time,
and there is no way to change a binding's type thereafter

## scalar types

these are types that represent a single value

### integer

| bits                   | signed                   | unsigned                     |
| ---------------------- | ------------------------ | ---------------------------- |
| 8                      | `Integer8`               | `Magnitude8`                 |
| 16                     | `Integer16`              | `Magnitude16`                |
| 32                     | `Integer32` or `Integer` | `Magnitude32` or `Magnitude` |
| 64                     | `Integer64`              | `Magnitude64`                |
| Architecture-dependent | `ArchitectureInteger`    | `AchitectureMagnitude`       |

`Integer` and `Magnitude` are just aliases for `Integer32` and `Magnitude32` respectively

### fractional

| bits | type                           |
| ---- | ------------------------------ |
| 32   | `Fractional32`                 |
| 64   | `Fractional64` or `Fractional` |

> represented as IEEE-754 floating point numbers

`Fractional` is just an alias for `Fractional64`

### numeric operators

```
runApp -> {
    let sum -- 1 + 2
    let difference -- 1 - 2
    let product -- 1 * 2
    let quotient -- 1 / 2
    let remainder -- 1 remainder 2
}
```

### binary

a binary value (can be one of two states) can be respresented with the `Binary` type.
you might know it as `boolean` in other languages

for aesthetic reasons (length and symmetry), grass uses these for its binary values

| `boolean` | `Binary` |
| --------- | -------- |
| `true`    | `yea`    |
| `false`   | `nah`    |

the binary type is the only type accepted as `if` predicates

```
runApp -> {
    let a -- yea
    let b -- nah

    if a {
        printLine("YEA!") // will print
    }

    if b {
        printLine("NAH!") // won't print
    }
}
```

### character

the `Character` type represents a single UTF-8 character

```
runApp -> {
    let a -- 'c'
    printLine(a) // will print 'c'
}
```

## compound types

these group multiple values into one type.
grass has two basic compound types: tuples and arrays

> grass also has sum and product types, but they are discussed later

### tuple

a tuple is group of multiple values with fixed and independent types

```
runApp -> {
    // explicit type for clarity; not required
    let tuple: (Integer, Integer, Character, Binary) -- (1, 2, 'c', yea)
}
```

an unnamed tuple can be destructured positionally

```
printValues tuple: (Integer, Integer, Character, Binary) -> {
    let (a, b, c, d) -- tuple

    // explicit type added for clarity. not required
    let a: Integer -- a
    let b: Integer -- b
    let c: Character -- c
    let d: Binary -- d
}
```

tuple fields can also be named, 
allowing you to access each value with the name you gave

```
getTuple -> (a: Integer, b: Integer, c: Character, d: Binary) {
    return (1, 2, 'c', yea)
}

runApp -> {
    let tuple ~ getTuple.
    printLine(tuple.d) // prints "yea"
}
```

we can also infer the return type of `getTuple`

```
getTuple -> {
    return (a: 1, b: 2, c: 'c', d: yea)
}
```

named tuples can be destructured by name

```
runApp -> {
    // using the same getTuple as the last example
    let { a, c } = getTuple.
}
```

unnamed tuple values can be accessed without destructuring
using the `.` operator, with index `0` being the first value

```
printThirdValue tuple: (Integer, Integer, Character, Binary) -> {
    printLine(tuple.2)
}
```

> note to self: since tuples are basically just anonymous product types,
> it would be awesome if we could also have anonymous sum types!

### array

an array is also a group of multiple values with fixed size,
but they are all of the same type

```
runApp -> {
    // explicit type for clarity; not required
    let a: [Integer of 5] ~ [1, 2, 3, 4, 5]
}
```

an array's elements can be access using the `[]` operator,
with index `0` being the first element

```
getThirdElement array: [Integer of 5] -> {
    return array[2]
}
```
