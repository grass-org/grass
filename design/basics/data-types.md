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
| Architecture-dependent | `ArchInteger`            | `ArchMagnitude`              |

`Integer` and `Magnitude` are just aliases for `Integer32` and `Magnitude32` respectively

### fraction

| bits | type                       |
| ---- | -------------------------- |
| 32   | `Fraction32`               |
| 64   | `Fraction64` or `Fraction` |

> represented as IEEE-754 floating point numbers

`Fraction` is just an alias for `Fraction64`

### numeric operators

```
fn runApp {
    let sum := 1 + 2
    let difference := 1 - 2
    let product := 1 * 2
    let quotient := 1 / 2
    let remainder := 1 % 2
}
```

### binary

a binary value (can be one of two states) can be respresented with the `Binary` type.
you might know it as `boolean` in other languages

for aesthetic reasons (length and symmetry), grass uses these for its binary values

| `boolean` | `Binary` |
| --------- | -------- |
| `true`    | `Yea`    |
| `false`   | `Nah`    |

the binary type is the only type accepted as `if` predicates

```
fn runApp {
    let a := Yea
    let b := Nah

    if a {
        printLine("YEA!") // will print
    }

    if b {
        printLine("NAH!") // won't print
    }
}
```

### ternary

sometimes, you want to represent uncertainty on top of `Yea` and `Nah`.
in other languages, 
this could be represented as a nullable `boolean` (`Option<bool>` in Rust).
grass has the `Ternary` type

| `boolean?` | `Binary` |
| ---------- | -------- |
| `true`     | `Yea`    |
| `false`    | `Nah`    |
| `null`     | `Idk`    |

### character

the `Character` type represents a single UTF-8 character

```
fn runApp {
    let a := 'c'
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
fn runApp {
    // explicit type for clarity; not required
    let tuple: (Integer, Integer, Character, Binary) := (1, 2, 'c', Yea)
}
```

an unnamed tuple can be destructured positionally

```
fn printValues(tuple: (Integer, Integer, Character, Binary)) {
    let (a, b, c, d) := tuple

    // explicit type added for clarity. not required
    let a: Integer := a
    let b: Integer := b
    let c: Character := c
    let d: Binary := d
}
```

tuple fields can also be named, 
allowing you to access each value with the name you gave

```
fn getTuple: { a: Integer, b: Integer, c: Character, d: Binary } {
    out (a: 1, b: 2, c: 'c', d: Yea)
}

fn runApp {
    let tuple ~ getTuple()
    printLine(tuple.d) // prints "Yea"
}
```

named tuples can be destructured by name

```
fn runApp {
    // using the same getTuple as the last example
    let { a, c } := getTuple()
}
```

unnamed tuple values can be accessed without destructuring
using the `.` operator, with index `0` being the first value

```
fn printThirdValue(tuple: (Integer, Integer, Character, Binary)) {
    printLine(tuple.2)
}
```

tuples types can of course be inferred

```
// inferred: (Integer, Integer, Character, Binary)
fn getTuples {
    out (1, 2, 'c' Yea)
}

// inferred: { a: Integer, b: Integer, c: Character, d: Yea }
fn getNamedTuples {
    out (a: 1, b: 2, c: 'c', d: Yea)
}

fn runApp {
    // note: you can explicitly type destructured tuple fields

    let (a: Integer, b: Integer, c: Character, d: Binary) := getTuples()

    let { a: Integer, b: Integer, c: Character, d: Binary } := 
        getNamedTuples()
}
```

### union

a union is an anonymous sum type.
we won't go over sum types here,
but here is the syntax

```
fn double(value: Integer | Character) -> Integer | String {
    out when value {
        is Integer integer: integer * 2,
        is Character character: character.repeat(2),
    }
}
```

read more about [sum types](algebraic-types.md#sum-types)

### array

an array is also a group of multiple values with fixed size,
but they are all of the same type

```
fn runApp {
    // explicit type for clarity; not required
    let a: [Integer of 5] ~ [1, 2, 3, 4, 5]
}
```

an array's elements can be access using the `[]` operator,
with index `0` being the first element

```
fn getThirdElement(array: [Integer of 5]) {
    out array[2]
}
```
