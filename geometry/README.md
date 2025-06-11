# References

## Shared References

A reference provides a way to access another value without taking ownership of the value, and is also called "borrowing". Shared references are read-only, and the referenced data cannot change.

```rust
fn main() {
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;
    dbg!(r);

    r = &b;
    dbg!(r);
}
```

A shared reference to a type `T` has type `&T`. A reference value is made with the & operator. The `*` operator "dereferences" a reference, yielding its value.

References in Rust:

- References can never be null in Rust, so null checking is not necessary.
- A reference is said to “borrow” the value it refers to: code can use the reference to access the value, but is still "owned" by the original variable.
- References are implemented as pointers, and a key advantage is that they can be much smaller than the thing they point to.
- Explicit referencing with `&` is usually required. However, Rust performs automatic referencing and dereferencing when invoking methods.
- A shared reference does not allow modifying the value it refers to, even if that value was mutable.
- Rust is tracking the lifetimes of all references to ensure they live long enough. Dangling references cannot occur in safe Rust.

### Reference Validity

Rust enforces a number of rules for references that make them always safe to use. One rule is that references can never be null, making them safe to use without null checks. The other rule is that references can't outlive the data they point to.

```rust
fn main() {
    let x_ref = {
        let x = 10;
        &x
    };
    dbg!(x_ref);
}

let r;
{
    let x = 5;
    r = &x; // ❌ ERROR: x goes away at the end of the block
}
println!("{}", r); // BAD: r points to nothing!
```

Rust’s rules make sure that when you use a reference:

- It always points to something real
- It never lasts longer than what it points to

## Exclusive References

Exclusive references, also known as mutable references, allow changing the value they refer to. They have type `&mut T`. Exclusive references are references that let you change a value, but only if you are the only one using it at that moment.

```rust
fn main() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
}

let mut x = 5;
let r1 = &mut x;
let r2 = &mut x; // ❌ ERROR! You can’t have two &mut at the same time
```

"Exclusive" means that only this reference can be used to access the value. No other references (shared or exclusive) can exist at the same time, and the referenced value cannot be accessed while the exclusive reference exists.

## Slices

In Rust, a slice is a way to look at part of a collection — like part of an array, a vector, or a string — without copying it.

We create a slice by borrowing `a` and specifying the starting and ending indexes in brackets.

```rust
fn main() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    println!("s: {s:?}");
}
```

Slices are just references to existing data. They don’t contain the actual values — they just borrow them.

If the slice starts at index 0, Rust’s range syntax allows us to drop the starting index, meaning that `&a[0..a.len()]` and `&a[..a.len()]` are identical. The same is true for the last index, so `&a[2..a.len()]` and `&a[2..]` are identical. To easily create a slice of the full array, we can therefore use `&a[..]`.

You can't "grow" a slice once it’s created:

- You can't append elements of the slice, since it doesn't own the backing buffer.
- You can't grow a slice to point to a larger section of the backing buffer. The slice loses information about the underlying buffer and so you can't know how larger the slice can be grown.
- To get a larger slice you have to back to the original buffer and create a larger slice from there.

## Exercise: Geometry

We will create a few utility functions for 3-dimensional geometry, representing a point as [f64;3]. It is up to you to determine the function signatures.

```rust
// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.


fn magnitude(...) -> f64 {
    todo!()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.


fn normalize(...) {
    todo!()
}

// Use the following `main` to test your work.

fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
```
