# generics

generics allow us to make
a single definition of a function, a type, or a trait,
which we can use with multiple concrete types

## generic functions

if you find yourself making multiple functions
that do the same thing but for different types like this

```
fn getLargest[list: -[Integer]]: Optional<Integer> {
    mutable largest -- None

    for list: value {
        if largest.any(largest -> value <= largest) {
            skip
        }

        largest -- value
    }

    out largest
}

fn getLargest[list: -[Character]]: Optional<Character> {
    mutable largest -- None

    for list: value {
        if largest.any(largest -> value <= largest) {
            skip
        }

        largest -- value
    }

    out largest
}
```

then it might be time to generify

```
<Element: Compare>
fn getLargest[list: -[Element]]: Optional<Element> {
    mutable largest -- None

    for list: value {
        if largest.any(largest -> value <= largest) {
            skip
        }

        largest -- value
    }

    out largest
}
```

`Compare` is the trait that defines the comparison operators,
so we had to bound the generic type `Element`
to be any type that implements `Compare`,
or we wouldn't be able to use `<=`

this is how you call a generic function

```
runApp -> {
    // explicit type arguments
    let largest -- getLargest<Element: Integer>([1, 2, 3, 4]-)

    // inferred type arguments
    let largest -- getLargest([1, 2, 3, 4]-) // <Element: Integer>
}
```

if a type parameter is unambiguous and doesn't need to be named,
then you can define it positionally

```
[TypeA, TypeB]
doSomething a: TypeA, b: TypeB -> {}
```

this is how you call a function with positional type parameters

```
runApp -> {
    // explicit type arguments
    doSomething<Integer, Integer>(1, 2)

    // inferred type arguments
    doSomething(1, 2) // <Integer, Integer>
}
```

## generic types

here is the `Result` type
which is a sum type
that has two named type parameters

```
<Value, Error>
type Result {
    Value<Value>,
    Error<Error>,
}

fn use(result: Result<Value: Integer, Error: Text>) {}
```

here are the `Value` and `Error` types,
each having one positinal type parameter

```
[Value]
type Value {
    value: Value,
}

[Error]
type Error {
    error: Error,
}

fn use(value: Value<Integer>) {}
fn use(value: Error<Text>) {}
```

## generic traits

here is the `Into` trait,
which has a single positional type parameter

```
[Target]
trait Into {
    fn (self: Self).into -> Target
}
```

here is how you implement `Into`

```
Decision: Into<String> {
    fn (decision).into -> {
        let { inFavor, final } -- decision
        out "Decision(inFavor: {inFavor}, final: {final})"
    }
}
```

> note: [trait associated types](traits.md#associated-types)
> are different from trait type parameters

## syntax

```
<Type>
fn example1(a: Type): Type {}

<Type1, Type2>
fn example2(a: Type1, b: Type2) {}

<Type1: Compare, Type2: Equals>
fn example3(a: Type1, b: Type2) {}

<Type1: Compare + Equals, Type2: Equals>
fn example4(a: Type1, b: Type2) {}

public
<Type1: Compare + Equals>
<Type2: Equals>
fn example5(a: Type1, b: Type2) {}

[Type1: Compare + Equals, Type2: Equals]
fn example6(a: Type1, b: Type2) {}

[Type1: Compare + Equals]
[Type2: Equals]
fn example7(a: Type1, b: Type2) {}

public
<Type1: Compare + Equals>
<Type2: Equals>
type Example8 {
    type1: Type1,
    type2: Type2,
}

public
<Type1: Compare + Equals>
<Type2: Equals>
trait Example9 {
    fn into(type1: Type1) -> Type2
}
```
