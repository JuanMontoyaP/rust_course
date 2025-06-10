# Tuples and Arrays

## Arrays

An array in Rust is a collection of values, all of them have the same type with a fixed size.

```rust
fn main() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[2] = 0;
    println!("a: {a:?}");
}
```

A value of the array type `[T; N]`holds `N` (a compile-time constant) elements of the same type `T`. The length of the array is part of its type, which means that `[u8; 3]` and `[u8; 4]` are different types.

Arrays can also be initialized in a shorthand syntax, e.g.

```rust
let zeros = [0; 5];       // [0, 0, 0, 0, 0]
let trues = [true; 3];    // [true, true, true]
let spaces = [' '; 4];    // [' ', ' ', ' ', ' ']
```

This only works when the value you're repeating can be copied easily (i.e., types that implement the Copy trait, like numbers, booleans, and chars).

The `println!` macro asks for the debug implementation with the `?` format parameter: `{}` gives the default output, `{:?}` gives the debug output. Types such as integers and strings implement the default output, but arrays only implement the debug output. This means that we must use debug output here. Adding `#`. e.g. `{a:#?}`, invokes a "pretty printing" format, which can be easier to read.

## Tuples

A tuple is and ordered collection of values. Tuples can contain multiple elements, hold different types of data and usually are immutable and fixed in size.

```rust
fn main() {
    let t: (i8, bool) = (7, true);
    dbg!(t.0);
    dbg!(t.1);
}
```

Fields of a tuple can be accessed by the period and the index of the value, e.g. `t.0`, `t.1`. The empty tuple `()` is referred to as the "unit type" and signifies absence of a return value.

## Array Iteration


The `for` statement supports iterating over arrays (but not tuples).
```rust
fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            dbg!(prime);
            dbg!(i);
            assert_ne!(prime % i, 0);
        }
    }
}
```

If you need indexing:
```rust
let letters = ['a', 'b', 'c'];

for (i, letter) in letters.iter().enumerate() {
    println!("Index: {}, Letter: {}", i, letter);
}
```

If you're just reading values, use `.iter()`:
```rust
for val in array.iter() { ... }
```

If you're changing values, use `.iter_mut()`:
```rust
for val in array.iter_mut() {
    *val += 1;
}
```

## Patterns and Destructuring

Rust supports using pattern matching to destructure a larger value like a tuple into its constituent parts:
```rust
fn check_order(tuple: (i32, i32, i32)) -> bool {
    let (left, middle, right) = tuple;
    left < middle && middle < right
}

fn main() {
    let tuple = (1, 5, 3);
    println!(
        "{tuple:?}: {}",
        if check_order(tuple) { "ordered" } else { "unordered" }
    );
}
```

- The patterns used here are "irrefutable", meaning that the compiler can statically verify that the value on the right of `=` has the same structure as the pattern.

## Exercise: Nested Arrays
Arrays can contain other arrays:
```rust
let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
```

What is the type of this variable? `[[i32, 3], 3]`

Use an array such as the above to write a function transpose which will transpose a matrix (turn rows into columns):

$$ transpose \left(
\begin{bmatrix}
1 & 2 & 3 \\
4 & 5 & 6 \\
7 & 8 & 9
\end{bmatrix}
\right)
= 
\begin{bmatrix}
1 & 4 & 7 \\
2 & 5 & 8 \\
3 & 6 & 9
\end{bmatrix}
$$

Copy the code below to https://play.rust-lang.org/ and implement the function. This function only operates on 3x3 matrices.
```rust
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    todo!()
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    dbg!(matrix);
    let transposed = transpose(matrix);
    dbg!(transposed);
}
```
