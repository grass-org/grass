# functions

a function that returns nothing can be declared like this

```
runApp -> {
    printLine("Hello, world!")
}
```

> the example above is the `runApp` function,
> which is the entry point to your application.
> this is equivalent to the `main` function in languages like C

a function can return a value using the `out` keyword

```
getNumber -> {
    out 67
}
```

this is how you call a function

```
runApp -> {
    let number -- getNumber()
    printLine(number)
}
```

grass automatically infers the return type of a private function.
if you want to write the return type explicitly,
you can write it after the `->`

```
getNumber -> Integer {
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
getNumber -> {
    out 67
}
```

a function can have parameters

```
add left: Integer, right: Integer -> {
    out left + right
}

runApp -> {
    let number -- add(left: 1, right: 2)
    printLine(number)
}
```

normally, arguments to functions are required to be named.
this prevents you from accidentally mixing up your arguments,
especially if they have the same type.
this also allows you to write the arguments in any order

```
runApp -> {
    printLine(add(right: 2, left: 1)
}
```

however, if you have a binding with the same name as a named parameter,
then you don't have to name the argument itself

```
runApp -> {
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
put them inside `()`

```
add (left: Integer, right: Integer) -> {
    out left + right
}

runApp -> {
    printLine(add(1, 2))
}
```

functions that do not return anything have the `()` (unit) return type.
`()` is automatically inferred as the return type of a function
without any return statements.
however, of course, you can write still write it explicitly
if you really want to

```
runApp -> () {
    bark()
}
```

but of course, you can omit `()` return type in a public function

```
public
doSomething -> {
    bark()
}
```

## overloading

if the compiler can differentiate a function 
from another function with the same name,
then it will let you reuse the same name for that function

these can all have the same name
since the compiler can pick which one to use
depending on how they're called

```
doNothing number: Integer -> {}           // 1
doNothing integer: Integer -> {}          // 2
doNothing text: Text -> {}                // 3
doNothing (a: Integer) -> {}              // 4
doNothing (a: Integer, b: Integer) -> {}  // 5

runApp -> {
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
doNothing (number: Integer) -> {}
doNothing (pizza: Integer) -> {}

runApp -> {
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
order(food: Food -- Grass, drink: Drink -- Water) -> {}

runApp -> {
    order(food: Pizza) // same as order food: Pizza, drink: Water
    order(drink: Coke) // same as order food: Grass, drink: Coke
}
```

positional parameters with defaults should always be 
the last positional parameters declared,
since they are assigned positionally,
you wouldn't be able to skip a positional parameter with default
in the middle of other positional parameters
