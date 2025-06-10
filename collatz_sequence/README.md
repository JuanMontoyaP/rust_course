# Control Flow Basics

### Blocks and Scopes

- **Block:** A block in rust is any sequence enclosed in a pair of curly braces `{}`. Each block has a value and a type e.g.
    ```rust 
    fn main() {
        let y = {
            let x = 3;
            x + 1  // <--- This is the value the block returns!
        };
        println!("y is: {}", y); // prints "y is: 4"
    }
    ```
    In this example the value of the block will `4` and its type is `i32` which is the type of the value returned. If a block does not return anything, it will return `()`, which is the unit type that means "nothing".

    Blocks are expressions too, which means they can be used anywhere a value is expected. Blocks are expressions because they return a value.

- **Scope:** The variables in Rust are block scoped. They only live inside the block were they were created. When the scope ends, Rust automatically cleans up the variable e.g.
    ```rust
    fn main() {
        let x = 5;  // x is created here

        {
            let y = 10; // y exists only inside this block
            println!("Inside: x = {}, y = {}", x, y);
        }

        println!("Outside: x = {}", x); // This works
        // println!("Outside: y = {}", y); // ❌ This won't compile!
    }
    ```
    ```rust
    fn main() {
        let x = 5;
        {
            let x = x + 1; // shadows outer x. This x is a new variable
            println!("Inner x = {}", x); // prints 6
        }
        println!("Outer x = {}", x); // prints 5
    }
    ```
## `IF` Expressions

`IF` expressions can be used the same way as in other languages e.g.
```rust
fn main() {
    let x = 10;
    if x == 0 {
        println!("zero!");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }
}

// Output: biggish
```

You also can do inline expression to assign values to a variable:
```rust
fn main() {
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);
}

// Output: small
```

## `match` Expressions

It is used to check if a value fits in one or more options:
```rust
fn main() {
    let val = 1;
    match val {
        1 => println!("one"),
        10 => println!("ten"),
        100 => println!("one hundred"),
        _ => {
            println!("something else");
        }
    }
}
```
As `IF` expressions they can also be used as inline expression:
```rust
fn main() {
    let flag = true;
    let val = match flag {
        true => 1,
        false => 0,
    };
    dbg!(val)
}
```

- `match` arms are evaluated from top to bottom, and the first one that it matches has its corresponding body executed.
- `match` expressions are exhaustive, meaning they need to cover all posible cases or have a default case.


## Loops

### `while``

Exactly like other languages, executes its body as long as the condition is true.
```rust
fn main() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    dbg!(x);
}
```

### `for``
Iterates over ranges of values or items in a collection:
```rust
fn main() {
    for x in 1..5 {
        dbg!(x)
    }

    for elem in [2, 4, 6, 8] {
        dbg!(elem)
    }
}
```
In the first loop `x` iterates from 1 to 4. For an inclusive range use `1..=5`.

### `loop``
A loop that loops forever until a `break`.
```rust
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        dbg!(i);
        if i > 100 {
            break;
        }
    }
}
```

### `break` and `continue`

If you want to immediately start the next iteration use `continue`.

If you want to exit any kind of loop early, use `break`. With loop, this can take an optional expression that becomes the value of the loop expression.
```rust
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break 2 + 2;
        }
        if i % 2 == 0 {
            continue;
        }
        dbg!(i);
    }
}
```

Both continue and break can optionally take a label argument which is used to break out of nested loops:
```rust
fn main() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    dbg!(elements_searched);
}
```

Labeled break also works on arbitrary blocks, e.g.
```rust
'label: {
    break 'label;
    println!("This line gets skipped");
}
```

## Functions
 
Some features of Rust functions are:

- Overloading is not supported. You can not have functions with the same name but different inputs - That's called function overloading.
- Always takes a fixed number of parameter. You must specify exactly how many parameters it takes.
- Default arguments are not supported.
- Macros can be used to support variadic functions. "Variadic" means functions with a variable number of arguments, like `println!()`:

## Macros

In Rust, macros are like code-writing shortcuts. They let you write code that writes other code for you. This helps you avoid repeating yourself and can make your code more flexible and powerful.

1. They can take a variable number of arguments.
1. They can generate code depending on the input.
1. They run before the compiler checks your code, so they can change or generate code on the fly.

```rust
macro_rules! say_hello {
    () => {
        println!("Hello from a macro!");
    };
}

fn main() {
    say_hello!(); // expands to println!("Hello from a macro!");
    
    let my_vec = vec![1, 2, 3]; // becomes: Vec::new(); v.push(1); v.push(2); ...
}
```

They are distinguished by a ! at the end. The Rust standard library includes an assortment of useful macros.

- `println!(format, ..)` prints a line to standard output, applying formatting described in `std::fmt`.
- `format!(format, ..)` works just like `println!` but returns the result as a string.
- `dbg!(expression)` logs the value of the expression and returns it.
- `todo!()` marks a bit of code as not-yet-implemented. If executed, it will panic.

## Exercise: Collatz Sequence

The Collatz Sequence is defined as follows, for an arbitrary $n_1$ greater than zero:

- If $n_i$ is $1$, then the sequence terminates at $n_i$.
- If $n_i$ is even, then $n_{i+1}$ = $n_i / 2$.
- If $n_i$ is odd, then $n_{i+1} = 3 * n_i + 1$.

For example, beginning with n1 = 3:
- $3$ is odd, so $n_2 = 3 * 3 + 1 = 10$.
- $10$ is even, so $n_3 = 10 / 2 = 5$.
- $5$ is odd, so $n_4 = 3 * 5 + 1 = 16$.
- $16$ is even, so $n_5 = 16 / 2 = 8$.
- $8$ is even, so $n_6 = 8 / 2 = 4$.
- $4$ is even, so $n_7 = 4 / 2 = 2$.
- $2$ is even, so $n_8 = 1$.
- and the sequence terminates.

Write a function to calculate the length of the collatz sequence for a given initial $n$.

```rust
/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
  todo!("Implement this")
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}
```
