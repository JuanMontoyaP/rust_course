# Types and Values

## Variables

Variables are immutable by default. That mean that its value cannot be changed after its initialization.

```rust
fn main() {
    let x: i32 = 10;
    println!("x: {x}")
}
```

For creating mutable variables you should add the keyword `mut` in the definition command as follows:

```rust
fn main() {
    let mut x: i32 = 10;
    println!("x: {x}");
    x = 20;
    println!("x: {x}");
}
```

The `i32` here is the type of the variable. This must be known at compile time, but type inference (covered later) allows the programmer to omit it in many cases. On the following table you can find basic types of variables in Rust:

| Type Category   | Type Name     | Description                                           | Example                    |
|----------------|---------------|-------------------------------------------------------|----------------------------|
| Integer         | `i8`, `i16`, `i32`, `i64`, `i128` | Signed integers of different sizes                  | `let x: i32 = -42;`        |
|                 | `u8`, `u16`, `u32`, `u64`, `u128` | Unsigned integers of different sizes                | `let x: u8 = 255;`         |
|                 | `isize`, `usize`                 | Pointer-sized integers (platform-dependent)         | `let idx: usize = 10;`     |
| Floating Point  | `f32`, `f64`                     | 32-bit and 64-bit floating point numbers            | `let pi: f64 = 3.14;`      |
| Boolean         | `bool`                           | Boolean type with `true` or `false`                 | `let is_ready: bool = true;` |
| Character       | `char`                           | A single Unicode scalar value (4 bytes)             | `let c: char = '🦀';`       |
| String          | `&str` (string slice)            | Immutable view into a string                        | `let name: &str = "Rust";` |
|                 | `String`                         | Growable, heap-allocated string                     | `let name = String::from("Rust");` |
| Tuple           | `(T1, T2, ...)`                  | Grouping of multiple values of different types      | `let tup = (1, true, 'a');`|
| Array           | `[T; N]`                         | Fixed-size collection of elements of the same type  | `let arr = [1, 2, 3];`     |
| Slice           | `&[T]`                           | View into a sequence of elements                    | `let slice = &arr[0..2];`  |
| Unit            | `()`                             | Represents an empty value or return type            | `let unit = ();`           |


The types have widths as follows:

- `iN`, `uN`, and `fN` are `N` bits wide.
- `isize` and `usize` are the width of a pointer.
- `char` is 32 bits wide.
- `bool` is 8 bits wide.

When nothing constrains the type of an integer literal, Rust defaults to i32. This sometimes appears as {integer} in error messages. Similarly, floating-point literals default to f64.

## Exercise: Fibonacci

The Fibonacci sequence begins with [0,1]. For n>1, the n’th Fibonacci number is calculated recursively as the sum of the n-1’th and n-2’th Fibonacci numbers.

Write a function fib(n) that calculates the n’th Fibonacci number. When will this function panic?

```rust
fn fib(n: u32) -> u32 {
    if n < 2 {
        // The base case.
        return todo!("Implement this");
    } else {
        // The recursive case.
        return todo!("Implement this");
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
```