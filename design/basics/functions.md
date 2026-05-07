# functions

a function that returns nothing can be declared like this

```
fn runApp {
    printLine("Hello, world!")
}
```

> the example above is the `runApp` function,
> which is the entry point to your application.
> this is equivalent to the `main` function in languages like C

a function can return a value using the `out` keyword

```
fn getNumber {
    out 67
}
```

this is how you call a function

```
fn runApp {
    let number -- getNumber()
    printLine(number)
}
```

grass automatically infers the return type of a private function.
if you want to write the return type explicitly,
you can write it after a `:`

```
fn getNumber: Integer {
    out 67
}
```

public functions require explicit typing.
type inference is awesome,
but they can cause ripples of code changes
caused by a function changing its inferred type.
explicit typing forces us to think ahead,
preventing multi-file code changes

```
// ERROR: a public function requires an explicit return type
public parent
fn getNumber {
    out 67
}
```

a function can have parameters

```
fn add(left: Integer, right: Integer) {
    out left + right
}

fn runApp {
    let number -- add(left: 1, right: 2)
    printLine(number)
}
```

normally, arguments to functions are required to be named.
this prevents you from accidentally mixing up your arguments,
especially if they have the same type.
this also allows you to write the arguments in any order

```
fn runApp {
    printLine(add(right: 2, left: 1)
}
```

however, if you have a binding with the same name as a named parameter,
then you don't have to name the argument itself

```
fn runApp {
    let left -- 1
    let right -- 2

    // unnecessary named arguments
    let sum -- add(right: right, left: left)

    // simpler call. same as the one above
    let sum -- add(right, left)
}
```

if you want to allow the caller to provide arguments
without naming them,
put them inside `[]`

```
fn add[left: Integer, right: Integer] {
    out left + right
}

fn runApp {
    printLine(add(1, 2))
}
```

functions that do not return anything have the `()` (unit) return type.
`()` is automatically inferred as the return type of a function
without any return statements.
however, of course, you can write still write it explicitly
if you really want to

```
fn runApp: () {
    bark()
}
```

but of course, you can omit `()` return type in a public function

```
public
fn doSomething {
    bark()
}
```

## shorthand

if a function returns a value immediately,
then you can replace the function body `{}` and the `out` keyword
with a `->`

```
fn getNumber -> 67
```

of course, if it's a public function,
you have to specify the return type
(unless it's `()`)

```
public
fn getNumber: Integer -> 67
```

## overloading

if the compiler can differentiate a function 
from another function with the same name,
then it will let you reuse the same name for that function

these can all have the same name
since the compiler can pick which one to use
depending on how they're called

```
fn doNothing(number: Integer) {}         // 1
fn doNothing(integer: Integer) {}        // 2
fn doNothing(text: Text) {}              // 3
fn doNothing[a: Integer] {}              // 4
fn doNothing[a: Integer, b: Integer] {}  // 5

runApp {
    doNothing(number: 1)  // calls 1
    doNothing(integer: 2  // calls 2
    doNothing(text: "Hi") // calls 3
    doNothing(1)          // calls 4
    doNothing(1, 2)       // calls 5
}
```

the compiler checks the types of the parameters,
the number of parameters,
and the the names of named parameters
to resolve which overload to call

the following is an example of illegal overloading,
since despite having different parameter names,
they are both positional parameters with the same type.
there is no way for the compiler to disambiguate them when called

```
fn doNothing[number: Integer] {}
fn doNothing[pizza: Integer] {}

fn runApp {
    doNothing 1 // not sure which to call
}
```

the return type also doesn't factor into the disambiguation,
since return types are not usually present at the call site.
we could let the compiler allow this by forcing you to use
explicit types at every call site of the function,
but that makes your function too inconvenient to use and ungrass-like,
so it's straight up just not supported

## default values

function parameters can have default values,
allowing you to skip them when calling the function

```
fn order[food: Food -- Grass, drink: Drink -- Water] {}

fn runApp {
    order(food: Pizza) // same as order food: Pizza, drink: Water
    order(drink: Coke) // same as order food: Grass, drink: Coke
}
```

positional parameters with defaults should always be 
the last positional parameters declared,
since they are assigned positionally,
you wouldn't be able to skip a positional parameter with default
in the middle of other positional parameters

## closures

you can nest functions in grass

```
fn runApp {
    fn doSomething -> bark()

    bark()
}
```

a nested function can capture values from the outer scope;
we call functions with captured values `closures`

```
fn runApp {
    let number -- 1

    fn printDoubledNumber -> printLine(number * 2)

    printDoubledNumber()
}
```

## higher-order funtions

grass functions are first-class,
meaning we can store them in bindings and data structures.
each function automatically implements
at least one of these [traits](traits.md)

- `Function`
  - `(parameters) -> return`
  - a function that does not capture any value,
    or only captures non-mutable references
- `MutatingFunction`
  - `(parameters) ~> return`
  - a function that captures values mutably
- `OwningFunction`
  - `(parameters) => return`
  - a function that captures values and takes ownership of them
  - can only be called once

examples
- `(Integer, Integer) -> ()`
  - `Function<Input: (Integer, Integer), Output: ()>`
- `() -> Integer`
  - `Function<Input: (), Output: Integer>`
- `() ~> ()`
  - `MutatingFunction<Input: (), Output: ()>`
- `(Character) => (Integer)`
  - `OwningFunction<Input: (Character), Output: Integer)>`

here is an example of a function
taking another function as a parameter

```
fn printValue(getValue: () -> Integer) {
    let value -- getValue()
    printLine(value)
}

fn getFive -> 5

fn runApp {
    printValue(getValue: getFive)

    fn getSix {
        out 6
    }

    printValue(getValue: getSix)

    fn getSeven -> 7
    printValue(getValue: getSeven)
}
```

here is another,
where the function parameter has a parameter

```
fn (optional: Optional<Integer>).map[
    transform: (Integer) -> Integer,
]: Optional<Integer> {
    if optional !is Some(value) {
        out! None
    }

    out transform(value)
}

fn runApp {
    let optional: Optional<Integer> -- 2

    fn double(integer: Integer) -> integer * 2

    let optional -- optional.map(double)
}
```

you can also inline function arguments

```
fn runApp {
    printValue(getValue: fn getSix -> 6)
    printValue(getValue: fn getSeven -> 7)

    let optional: Optional<Integer> -- 2
    let optional -- optional.map(fn double(integer: Integer) -> integer * 2)
}
```

in the examples above,
since the names `getSix` and `getSeven`
were never really used
(`printValue` uses a different name `getValue`),
we can omit them,
and since `getValue` accepts a function,
we can also omit `fn` in the declaration.
and since the function parameter is already typed,
we don't have to write types in our function arguments

```
fn runApp {
    printValue(getValue: -> 6)
    printValue(getValue: -> 7)

    let optional: Optional<Integer> -- 2
    let x -- optional.map(x -> x * 2)
    let y -- optional.map(x {
        if x % 2 = 0 {
            out! x * 2
        }

        out x / 2
    })
}
```

also, grass allows you to let a function
accept its last positional function parameter
outside the call parentheses
if the function argument is declared inline

```
printValue[getValue: () -> Integer] -> {
    let value -- getValue()
    printLine(value)
}

runApp -> {
    printValue() -> 6
    printValue() {
        out 7
    }

    let optional: Optional<Integer> -- 2
    let x -- optional.map() x -> x * 2
    let y -- optional.map() x {
        if x % 2 = 0 {
            out! x * 2
        }

        out x / 2
    }
}
```

if there are no other arguments
but a trailing function argument,
then you can also omit the parentheses

```
runApp -> {
    printValue -> 6

    let optional: Optional<Integer> -- 2
    let x -- optional.map x -> x * 2
    let y -- optional.map x {
        if x % 2 = 0 {
            out! x * 2
        }

        out x / 2
    }
}
```
