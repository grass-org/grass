# bindings

use the `let` keyword to define an immutable binding

```
fn runApp {
    let a := 0
}
```

grass automatically infers the type of the binding you declared,
in this case, `a` is of type `Integer`.
if you need to explicitly type a binding,
this is how you do it

```
fn runApp {
    let a: Integer := 0
}
```

## mutation

use the `mutable` keyword to define a mutable binding

```
fn runApp {
    let a := 0
    a := 1 // cannot assign more than once to an immutable binding
    
    mutable b := 1
    b := 2 // OK!
}
```

also, unlike other languages,
grass uses `:=` instead of `=` for the assignment operator
(as you can see above). 
this is because grass reclaims the `=` operator for equality

## shadowing

grass lets you use the same name for a binding as another one

```
fn runApp {
    let a := 1
    let a := 2
}
```

this is not the same is mutating `a`.
`a` with the value `1` is still there, 
but is now inaccessible,
since `a` with the value `2` is always gonna be used when you reference `a`

this distinction between shadowing and mutation should be clearer here

```
fn runApp {
    let a := 1

    if flipCoin() {
        let a := 2
        printLine(a) // 2
    }

    printLine(a) // 1
}
```

since `a` with the value `2` went out of scope after the if block,
the original `a` is now accessible again

## constants

use the `constant` keyword to define a compile-time constant. it can be declared in any scope, including the top-level scope

```
constant DAYS_IN_AN_HOUR := 0
```
