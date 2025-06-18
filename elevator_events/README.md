# User Defined Types

## Named Structs

Named structs are a way to group related data under a single type with named fields. Think it as a blueprint for creating your own data types.

```rust
struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);
}
```

If you already have variables with the right names, then you can create the struct using a shorthand.

```rust
struct Person {
    name: String,
    age: u8,
}

let name = String::from("Alice");
let age = 30;

let person = Person {
    name,
    age,
};
```

**Note:** You can only use the shorthand when the field name in the struct is the same as the variable name you are using to assign it.

There are different types of structs.    
- Zero-sized structs (e.g. struct Foo;) might be used when implementing a trait on some type but don’t have any data that you want to store in the value itself.
- Tuple structs, used when the field names are not important.

You can reuse data of one defined struct into another one:
```rust
let jackie = Person { name: String::from("Jackie"), ..avery };
```

## Tuple Structs

If the field names are unimportant, you can use a tuple struct:

```rust
struct Point(i32, i32);

fn main() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}
```

This is often used for single-field wrappers (called `newtypes`):
```rust
struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(force: Newtons) {
    // ...
}

fn main() {
    let force = compute_thruster_force();
    set_thruster_force(force);
}
```
Newtypes are a great way to encode additional information about the value in a primitive type, for example: 
- The number is measured in some units: Newtons in the example above.
- The value passed some validation when it was created, so you no longer have to validate it again at every use: PhoneNumber(String) or OddNumber(u32).

When a tuple struct has zero fields, the () can be omitted. The result is a zero-sized type (ZST), of which there is only one value (the name of the type).

Zero-sized types are useful when:
- You want to mark something (like a label or flag).
- You want to implement traits for them.
- You don’t need to store any actual data, just want to say "this thing exists".

## Enums

An enum (short for enumeration) is a type that can be one of several possible values. Each value is called a variant. They allow you to collect a set of values under one type.

```rust
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

fn main() {
    let dir = Direction::Left;
    let player_move: PlayerMove = PlayerMove::Run(dir);
    println!("On this turn: {player_move:?}");
}
```

## Type Aliases
A type alias creates a name for another type. The two types can be used interchangeably. It doesn’t create a new type — just a new name for it.

```rust
enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// Aliases are more useful with long, complex types:
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>;
```

A newtype is often a better alternative since it creates a distinct type. Prefer `struct InventoryCount(usize)` to `type InventoryCount = usize`.

## Const

Constants are evaluated at compile time and their values are inlined wherever they are used:

```rust
const DIGEST_SIZE: usize = 3;
const FILL_VALUE: u8 = calculate_fill_value();

const fn calculate_fill_value() -> u8 {
    if DIGEST_SIZE < 10 { 42 } else { 13 }
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [FILL_VALUE; DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");
}
```

Only functions marked `const` can be called at compile time to generate `const` values. const functions can however be called at runtime.

Consts are:
- Immutable: Cannot be changed
- Compile-time: Must be known when the program is compiled
- Global: Can be used anywhere in the code, like a global value
- Must have a type: You must specify the type explicitly

**NOTE:** When a value is inlined, it means: The compiler replaces the name of the const with its actual value wherever it’s used in the code. It’s like doing a copy-paste of the value during compilation

## Static

A static variable is a global variable that:
- Lives for the entire program (has 'static lifetime)
- Is stored in a fixed memory location
- Can be mutable (with unsafe)

```rust
static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}
```
As noted in the [Rust RFC Book](https://rust-lang.github.io/rfcs/0246-const-vs-static.html), these are not inlined upon use and have an actual associated memory location. This is useful for unsafe and embedded code, and the variable lives through the entirety of the program execution. When a globally-scoped value does not have a reason to need object identity, const is generally preferred.

Use static when:
- You need shared data across your entire program
- You want to store something once in memory
- You might want to use it in low-level or unsafe code (like FFI)

## Exercise: Elevator Events

We will create a data structure to represent an event in an elevator control system. It is up to you to define the types and functions to construct various events. Use `#[derive(Debug)]` to allow the types to be formatted with `{:?}`.

This exercise only requires creating and populating data structures so that main runs without errors. The next part of the course will cover getting data out of these structures.

```rust
#![allow(dead_code)]

#[derive(Debug)]
/// An event in the elevator system that the controller must react to.
enum Event {
    // TODO: add required variants
}

/// A direction of travel.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// The car has arrived on the given floor.
fn car_arrived(floor: i32) -> Event {
    todo!()
}

/// The car doors have opened.
fn car_door_opened() -> Event {
    todo!()
}

/// The car doors have closed.
fn car_door_closed() -> Event {
    todo!()
}

/// A directional button was pressed in an elevator lobby on the given floor.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    todo!()
}

/// A floor button was pressed in the elevator car.
fn car_floor_button_pressed(floor: i32) -> Event {
    todo!()
}

fn main() {
    println!(
        "A ground floor passenger has pressed the up button: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
    println!("The car door opened: {:?}", car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
}
```
