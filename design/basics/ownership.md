# ownership

grass follows rust's ownership rules.
this means a value can only have one owner,
and when that owner goes out of scope,
that value is freed from memory

if a function or a data structure wants
wants access to that value,
it take ownership of it, clone it,
or borrow it

## moving ownership

after a value has been moved,
it is no longer accessible via its old binding.

a function can take ownership of a value

```
fn eatText[text: Text] {
    printLine(text)
}

fn runApp {
    let text -- Text("Hello!")
    eatText(text)

    printLine(text) // ERROR: text was moved
}
```

a value can take ownership of another value

```
type TextEater {
    text: Text,
}

fn runApp {
    let text -- Text("Hello!")
    let textEater -- TextEater { text }

    printLine(text) // ERROR: text was moved
}
```

## borrowing

if a function or a type just needs to reference a value,
then it can borrow it without taking ownership

mark the type with `-` to indicate
that you're asking for a reference

```
fn borrowText[text: -Text] {
    printLine(text)
}

fn runApp {
    let text -- Text("Hello!")
    borrowText(text-)

    printLine(text)
}
```

use `~` for mutable references

```
fn borrowText[text: ~Text] {
    printLine(text)
}

fn runApp {
    let text -- Text("Hello!")
    borrowText(text~)

    printLine(text)
}
```

## copying

if a type is simple enough
that copying it is just as
(and maybe even more)
efficient than referencing it,
then it would be nice if we could just
copy it by default

`Copy` is a special trait
that allows us to do just that.
all types that pass the criteria above
should implement `Copy`

one example of a copy type is `Integer`

```
fn printDouble(integer: Integer) {
    printLine(integer * 2)
}

fn runApp {
    let integer -- 1
    printDouble(integer)
    printDouble(integer) // still valid here!
}
```

if your type is purely made up of copy types
(or is a Unit type -- has no fields),
then it can derive the `Copy` trait

```
#[derive(Copy)]
type FavoriteNumbers {
    top1: Integer,
    top2: Integer,
    top3: Integer,
}
```

## cloning

`Clone` is another trait that has one function: `clone`.
you can call `clone` on a reference,
and it will make a clone of that reference's value

```
fn runApp {
    let text -- Text("Hello!")
    eatText(text-.clone())

    printLine(text) // text is still valid here
}
```

note that cloning may be expensive
and should be used with caution.
types that implement `Clone` but not `Copy`
indicate that depending on the scenario,
it might be smarter to pass these values
as references

## lifetimes

grass' borrow checker
is just Rust's borrow checker.
to read more about lifetimes,
just read this: 
[Rust - Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

just like Rust,
grass can elide lifetime annotations,
but when it can't,
then you have to specify them explicitly

what differs is grass' syntax.
this is how you specify a lifetime in grass

```
fn doSomething(a: -a Text, b: -a Text): -a Text {}
```

the example above requires that
the return value of `doSomething`
cannot outlive `a` and `b`

```
fn do_something<'a>(a: &'a str, b: &'a str): &'a str {}
```
