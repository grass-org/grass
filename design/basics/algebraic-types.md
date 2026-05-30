# algebraic types

algebraic data types (composite types) 
are types that are a combination of other types.
we have already seen algebraic types 
in [data types/tuples](data-types.md#tuple).
however, it would be nice
if we could name them as a whole,
so we wouldn't have to rewrite all the fields
whenever they're reused

## product types

a product type is a type whose number of possible states
is the product of all the possible states of its underlying type.
that's the technical definition of product types
(which is helpful to know when differentiating from sum types);
but in simple terms, these are just C-like structs

this is how you make a simple product type in grass

```
type Decision {
    inFavor: Ternary,
    final: Binary,
}
```

here we have an example of a product type `Decision`,
which has two fields: `inFavor: Ternary` and `final: Binary`

the number of possible states a `Decision` can have
is how many possible combination of unique states
`inFavor` and `final` can have as a pair

- `inFavor: Ternary`: 3 possible states
- `final: Binary`: 2 possible states
- `Decision`: 3 * 2 = 6 possible states

## sum types

a sum type is a type whose number of possible states
is the sum of all the possible states of its underlying type.
an example of a sum type is Rust's enums

a sum type is a really powerful type
that makes invalid states unrepresentable.
for example, you defined a simple `Cat` type like this as a product type

```
type Cat {
    alive: Binary,
    hungry: Binary,
}
```

the problem here is that a `Cat` can both be dead and hungry,
which doesn't make any sense.
instead, you can represent it as a sum type like this

```
type Cat {
    Alive { hungry: Binary } | Dead
}
```

a `Cat` can only be constructed as one of each variant,
`Cat.Alive` or `Cat.Dead`.
since only the `Cat.Alive` variant has the `hungry` field,
a `Cat` can no longer be dead and hungry at the same time

the number of possible states a `Cat` can have
is how many possible unique states
`Cat.Alive` and `Cat.Dead` can have individually

- `Cat.Alive`
  - `hungry: Binary`: 2 possible states
- `Cat.Dead`: 1 possible state
- `Cat`: 2 + 1 = 3 possible states

unlike in languages like Rust, 
where an enum variant isn't a type by itself,
grass sum type variants are their own types

```
fn feedCat(cat: Cat.Alive) {
    if hungry {
        feed(cat)
    }
}
```

also, grass sum types can have variants of already existing types,
making it more similar to TypeScript unions

```
type Pizza {
    toppings: Toppings,
    sauce: Sauce,
}

type Cake {
    icing: Icing,
}

type Food {
    | Pizza
    | Cake
}
```

> note: the code above can be written as
>
> ```
> type Food { Pizza | Cake }
> ```
>
> or
>
> ```
> type Food {
>     Pizza
>     | Cake
> }
> ```
>
> but the above syntax is preferred for extensibility

in this example, `Pizza` and `Cake` are indeed variants of `Food`,
but they are not under the `Food` name,
so you reference their types via just `Pizza` or `Cake`
and not `Food.Pizza` nor `Food.Cake`

however, if you do want these types to be under the `Food` name,
but want to declare them as top-level 
to avoid nesting in the type definition,
you can do this

```
type Food.Pizza {
    toppings: Toppings,
    sauce: Sauce,
}

type Food.Cake {
    icing: Icing,
}

type Food.Salt

type Food {
    | Pizza
    | Cake
    // ERROR: Food must have Food.Salt as one of its variants
}
```

## constructors

you can construct your named type
using its private field initializer

```
fn getDecision: Decision {
    out Decision { inFavor: Idk, final: Nah }
}

fn getCat: Cat {
    out Cat.Alive { hungry: Nah }
}
```

however, since the field initializer is always private
to the module the type was defined,
you need to expose your own public constructor.
a constructor is just a function,
usually with the same name as the type.
this also allows you to add custom logic to your constructor

```
public
fn Decision(inFavor: Ternary, final: Binary): Decision {
    if inFavor = Idk: 
        out Decision { inFavor, final: Nah }

    out Decision { inFavor, final }
}

public
fn Cat(alive: Binary, hungry: Binary): Cat {
    if !alive:
        out Cat.Dead

    out Cat.Alive { alive, hungry }
}
```

since a constuctor is just a function,
it doesn't have to return the same type 
as the one it's being defined for.
however, it is recommended that a constructor
has a return type that's at least related
to the type it's defined for

```
public
fn Decision(inFavor: Ternary, final: Binary): Optional<Decision> {
    if inFavor = Idk & final = Yea:
        out None

    out Decision { inFavor, final }
}

public
fn Cat(alive: Binary, hungry: Binary): Optional<Cat> {
    if !alive & hungry:
        out None

    if !alive:
        out Cat.Dead

    out Cat.Alive { alive, hungry }
}
```

however, if the constructor you want to expose
just directly calls the field initializer like this:

```
public
fn Decision(inFavor: Ternary, final: Binary): Decision {
    out Decision { inFavor, final }
}

public
fn Cat.Alive(hungry: Binary): Cat.Alive {
    out Cat.Alive { hungry }
}
```

then you can just derive `Constructor` for your type,
which produces said constructor

```
#[derive(Constructor)]
type Decision {
    inFavor: Ternary,
    final: Binary,
}

#[derive(Constructor)]
type Cat {
    | Alive { hungry: Binary }
    | Dead
}
```

## fields

you can access a type's fields via the `::` operator

```
fn isInFavor(decision: Decision): Ternary {
    out decision::inFavor
}
```

however, a type's fields are always private to their module.
to make it accessible elsewhere,
you need to expose a [getter method](#methods) or a [property](#properties)

## methods

methods are functions that you can call on a type.
here is an example getter method that exposes the `inFavor` field

```
public
fn [decision: Decision].isInFavor: Ternary {
    out decision::inFavor
}
```

you can call a method like this

```
fn printInFavor(decision: Decision): {
    printLine(decision.isInFavor())
}
```

a method's receiver is just another parameter you pass into the method.
the example above is basically the same as the example in [fields](#fields).
the only difference is the way you call it

making a parameter a receiver or just a regular parameter 
does not change anything about your access to its members.
both types of parameters can access all members of that type visible to that module.
a method can be declared in a different module,
but it wouldn't have access to any of the private members of that type

## properties

a property is a value associated to a type. 
it often is declared as an alias to a field,
or to associate some other computed value to that type

properties are basically just functions with a special syntax.
this means you can do literally anything inside the blocks,
and return literally anything

### property getter

here is how you declare a property getter

```
public
get [decision: Decision].inFavor: Ternary {
    out decision::inFavor
}
```

and here is how you get that property

```
fn printInFavor(decision: Decision): {
    printLine(decision.inFavor)
}
```

### property setter

here is how you declare a property setter

```
public
set [decision: ~Decision].inFavor: Ternary {
    decision::inFavor -- inFavor
}
```

> note: `~` means `decision` 
> is a [mutable reference](ownership.md#borrowing) to a `Decision`

and here is how you set that property

```
fn runApp {
    let decision -- Decision(inFavor: Yea, final: Yea)
    decision.inFavor -- Nah
    printLine(decision.inFavor) // prints Nah
}
```

### derive properties

if your getters and setters just directly access the fields,
then derive them

```
#[derive(Properties)]
type Decision {
    #[get, set]
    inFavor: Ternary,

    #[get, set]
    final: Binary,
}
```

## destructuring

since fields are private always private,
destructuring cannot be automatically implemented

to allow your struct to be destructured,
then you have to implement the `Destructure` trait

```
Decision: Destructure {
    Output: { inFavor: Ternary, final: Binary }

    fn [decision].destructure {
        out { decision::inFavor, decision::final }
    }
}

fn function(decision: Decision) {
    let { inFavor, final } -- decision
}
```

you can also make it positionally destructable
by returning an unnamed tuple instead

```
Decision: Destructure {
    Output: (Ternary, Binary)

    fn (decision).destructure {
        out (decision::inFavor, decision::final)
    }
}

fn function(decision: Decision) {
    let (a, b) -- decision
}
```

you can derive a named destructor for a type
that just destructures all its fields

```
#[derive(Destructure)]
type Decision {
    inFavor: Ternary,
    final: Binary,
}
```
