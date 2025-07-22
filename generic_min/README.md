# Generics

Rust lets you write functions, structs, enums, and traits that work with any type, instead of writing one version for each type. This is what generics do.

## Generic Functions

A generic function is a function that works with any type, not just one specific type like i32 or String.

```rust
fn pick<T>(cond: bool, left: T, right: T) -> T {
    if cond { left } else { right }
}

fn main() {
    println!("picked a number: {:?}", pick(true, 222, 333));
    println!("picked a string: {:?}", pick(false, 'L', 'R'));
}
```

Rust supports generics, which lets you abstract algorithms or data structures (such as sorting or a binary tree) over the types used or stored.

## Trait Bounds

A trait bound says: "The type you use must have certain abilities (traits) for this code to work."

When working with generics, you often want to require the types to implement some trait, so that you can call this trait’s methods.

You can do this with `T: Trait`:

```rust
fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

struct NotCloneable;

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");

    let fas = NotCloneable;
    let pa = dupicalte(fas) // error
}
```

When multiple traits are necessary, use `+` to join them.

You also can use `where` clause for decluttering the function signature if you have many parameters:

```rust
fn do_stuff<T: Clone + Debug, U: Copy + PartialOrd>(a: T, b: U) { ... }


fn do_stuff<T, U>(a: T, b: U)
where
    T: Clone + Debug,
    U: Copy + PartialOrd,
{
    // function body
}
```

It has an extra feature is that the type on the left of `:` can be arbitrary, like `Option<T>`.

```rust
fn show<T>(val: T)
where
    Option<T>: std::fmt::Debug,
{
    println!("{:?}", Some(val));
}
```

`Option<T>` must implement Debug. You can't write this inline like `fn show<T: Debug>(...)` because it's not `T` that needs to implement Debug, it's `Option<T>`.

## Generic Data Types

You can use generics to abstract over the concrete field type. Returning to the exercise for the previous segment:

```rust
pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Only log messages up to the given verbosity level.
struct VerbosityFilter<L> {
    max_verbosity: u8,
    inner: L,
}

impl<L: Logger> Logger for VerbosityFilter<L> {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}
```

The `VerbosityFilter<L>` struct jas a generic type parameter `L`. It wraps any type `L` that implements `Logger`.

Now, the implementation of the trait `Logger` for `VerbosityFilter<L>` can be read in two parts:

1. The `impl<L: Logger>` that says:
    > This block is generic over a type L, but only if L implements the Logger trait.

1. The `Logger for VerbosityFilter<L>` that says:
    > This says we are implementing the Logger trait for the type VerbosityFilter<L> — that is, the generic struct where L is the wrapped logger type.

So the full line means:

> "We are implementing the Logger trait for all `VerbosityFilter<L>` where L implements Logger."

Also you can do this:
```rust
struct VerbosityFilter<L: Logger> { ... }
```

But in Rust, it’s more idiomatic to:
- Keep the struct definition generic, with no bounds
- And only put bounds in the impl block (where they’re actually needed)

This keeps the struct flexible, and makes the compiler only care about trait requirements when methods are used.

## Generic Traits

Traits can also be generic, just like types and functions. A trait’s parameters get concrete types when it is used. For example the `From<T>` trait is used to define type conversions:

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(from: u32) -> Foo {
        Foo(format!("Converted from integer: {from}"))
    }
}

impl From<bool> for Foo {
    fn from(from: bool) -> Foo {
        Foo(format!("Converted from bool: {from}"))
    }
}

fn main() {
    let from_int = Foo::from(123);
    let from_bool = Foo::from(true);
    dbg!(from_int);
    dbg!(from_bool);
}
```

Implementations of the trait do not need to cover all possible type parameters. Here, `Foo::from("hello")` would not compile because there is no `From<&str>` implementation for `Foo`.

Generic traits take types as "input", while associated types are a kind of "output" type. A trait can have multiple implementations for different input types.

```rust
trait Iterator {
    type Item; // 👈 this is an associated type

    fn next(&mut self) -> Option<Self::Item>;
}

// type Item is not passed in — it’s defined inside the trait implementation.
// Each type that implements Iterator decides what the output type is (Item).

struct Counter;

impl Iterator for Counter {
    type Item = i32; // 👈 defines the "output" type

    fn next(&mut self) -> Option<i32> {
        Some(42)
    }
}

```

Think of a trait like a machine.
- A generic trait takes different materials in (you pass in the type) — like a machine that can work with wood, metal, or plastic.
- An associated type is like a machine that always outputs a fixed shape, like it always builds wheels, regardless of the input.

## `impl Trait`

Similar to trait bounds, an impl Trait syntax can be used in function arguments and return values:

```rust
// Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}

fn main() {
    let many = add_42_millions(42_i8);
    dbg!(many);
    let many_more = add_42_millions(10_000_000);
    dbg!(many_more);
    let debuggable = pair_of(27);
    dbg!(debuggable);
}
```

The meaning of `impl Trait` is a bit different in the different positions.
- For a parameter, `impl Trait` is like an anonymous generic parameter with a trait bound.
- For a return type, it means that the return type is some concrete type that implements the trait, without naming the type. This can be useful when you don't want to expose the concrete type in a public API.
    
    Inference is hard in return position. A function returning `impl Foo` picks the concrete type it returns, without writing it out in the source. A function returning a generic type like `collect<B>() -> B` can return any type satisfying B, and the caller may need to choose one, such as with `let x: Vec<_> = foo.collect()` or with the turbofish, `foo.collect::<Vec<_>>()`.

## `dyn Trait`

dyn Trait means:

> "This is a value of some type that implements a trait, but I don’t know which exact type at compile time."

It stands for dynamic dispatch — meaning Rust figures out at runtime which method to call.

```rust
struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

// Uses generics and static dispatch.
fn generic(pet: &impl Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

// Uses type-erasure and dynamic dispatch.
fn dynamic(pet: &dyn Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

fn main() {
    let cat = Cat { lives: 9 };
    let dog = Dog { name: String::from("Fido"), age: 5 };

    generic(&cat);
    generic(&dog);

    dynamic(&cat);
    dynamic(&dog);
}
```

A `dyn Trait` is considered to be "type-erased", because we no longer have compile-time knowledge of what the concrete type is.

When using `dyn Trait`, it instead uses dynamic dispatch through a virtual method table (vtable). This means that there's a single version of `fn dynamic` that is used regardless of what type of Pet is passed in.

When using `dyn Trait`, the trait object needs to be behind some kind of indirection. In this case it's a reference, though smart pointer types like `Box` can also be used.

At runtime, a `&dyn Pet` is represented as a "fat pointer", i.e. a pair of two pointers: One pointer points to the concrete object that implements Pet, and the other points to the vtable for the trait implementation for that type. When calling the `talk` method on `&dyn Pet` the compiler looks up the function pointer for talk in the vtable and then invokes the function, passing the pointer to the `Dog` or `Cat` into that function. The compiler doesn't need to know the concrete type of the `Pet` in order to do this.

## Exercise: Generic `min`-

In this short exercise, you will implement a generic min function that determines the minimum of two values, using the [`Ord`](https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html) trait.

```rust
use std::cmp::Ordering;

// TODO: implement the `min` function used in the tests.

#[test]
fn integers() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);
}

#[test]
fn chars() {
    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');
}

#[test]
fn strings() {
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}
```
