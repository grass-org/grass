# traits

grass does not support inheritance.
however, it does support polymorphism via traits.
grass traits are like interfaces in languages like C# or Java,
and is basically the same as Rust traits

a trait is created via the `:` syntax,
and is implemented via the same `:` syntax

```
type Dog

type Cat

trait Bark {
    fn bark()
}

Dog: Bark {
    fn bark {
        printLine("woof!")
    }
}

Cat: Bark {
    fn bark {
        printLine("meoof?")
    }
}
```

a trait can have default implementations.
in the following example,
we avoided having to manually implement
`bark` for `Dog`, 
since the default implementation
already does what `Dog` wants as its implementation

```
trait Bark {
    fn bark {
        printLine("woof!")
    }
}

Dog: Bark

Cat: Bark {
    fn bark {
        printLine("meoof?")
    }
}
```

## associated types

a trait can allow its implementers
to associate types to their own implementatins

```
trait Contains {
    [Output]

    fn (self: Self).contains[value: Value]: Binary
}
```

this is different from a generic trait

```
[Value]
trait GenericContains {
    (self: Self).contains(a: Value): Binary
}
```

`GenericContains<Value>` results into different traits;
`GenericContains<int>` is a different trait
from `GenericContains<string>`.
`Contains` on the other hand is one trait,
and you can only implement it once for a given type,
and that type defines what type `Value` is for that given type

`Self` is an automatically generated associated type,
which is just the exact type of the implementer

## using traits

a trait is not really a type.
it's more of a contract that types implement a specific type.
methods are implemented on types, not traits.
so for the compiler to know which methods to call for a trait,
we need to choose a dispatch strategy

### static dispatch

we can achieve static dispatch by knowing at compile time
the exact type that implements a specific trait.
we can achieve this by using generics

```
<Value: Trait>
fn doSomething(value: Value) {}

<Value: Trait>
type Type {
    value: Value,
}
```

in the example above,
if you use `doSomething<Value: Integer>` or `Type<Value: Integer>`,
the compiler will generate something like this,
which are the ones that will actually be used under the hood

```
fn doSomethingInteger(value: Integer) {}

type TypeInteger {
    value: Integer,
}
```

however, it would be nice to use static dispatch
without having to make a lot of generic notations.
the code below is treated by the compiler
as if you explictly defines type parameters

```
fn doSomething(value: Trait) {}

type Type {
    value: Trait,
}
```

however, the compiler automatically assumes
that one usage of trait is unrelated to another,
even if they're the exact same trait

```
fn doSomething(value: Trait): Trait

Type {
    a: Trait,
    b: Trait,
}
```

the code above gets translated into this

```
<Value: Trait, Return: Trait>
fn doSomething(value: Value): Return

<A: Trait, B: Trait>
type Type {
    a: A,
    b: B,
}
```

if you meant for them to be the same type,
then declare the type parameter yourself

```
<Value: Trait>
fn doSomething(value: Value): Value

<Value: Trait>
type Type {
    a: Value,
    b: Value,
}
```

### dynamic dispatch

however, sometimes,
you don't want to constrain your trait usage
to be a specific type.
an example is when you're doing
conditional dependency injection,
where the conditions are only known at runtime.
in this case, you have to use dynamic dispatch

```
fn doSomething(value: Box<dynamic Trait>) {}
```

since the compiler has no idea
which type you're using for that trait,
instead, grass uses pointers inside a trait object
to know which methods to call

> note: since the compiler
> doesn't know the exact type of `dynamic Trait`,
> it wouldn't how much space
> to allocate for `value` in the stack.
> so it has to be in a container like `Box`,
> since `Box` itself is of fixed size
> that points to dynamically sized data
