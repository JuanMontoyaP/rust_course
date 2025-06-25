# Pattern Matching

## Irrefutable Patterns

Irrefutable patterns in Rust are patterns that always match the value they are being compared to. There is not chance for the pattern to fail.

Rule of Thumb:
- Irrefutable = always matches (safe for let, fn, for).
- Refutable = might fail (use in if let, match, or while let).

```rust
fn takes_tuple(tuple: (char, i32, bool)) {
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;

    // This does the same thing as above.
    let (a, b, c) = tuple;

    // Ignore the first element, only bind the second and third.
    let (_, b, c) = tuple;

    // Ignore everything but the last element.
    let (.., c) = tuple;
}

fn main() {
    takes_tuple(('a', 777, true));
}
```

All of the demonstrated patterns are irrefutable, meaning that they will always match the value on the right hand side.

 - `_` is a pattern that always matches any value, discarding the matched value.
- `..` allows you to ignore multiple values at once.

More advanced usages of `..`, such as ignoring the middle elements of a tuple.

```rust
fn takes_tuple(tuple: (char, i32, bool, u8)) {
    let (first, .., last) = tuple;
}
```

All of these patterns work with arrays as well:

```rust
fn takes_array(array: [u8; 5]) {
    let [first, .., last] = array;
}
```

## Matching Values

The match keyword lets you match a value against one or more patterns. The patterns can be simple values but they can also be used to express more complex conditions:

```rust
#[rustfmt::skip]
fn main() {
    let input = 'x';
    match input {
        'q'                       => println!("Quitting"),
        'a' | 's' | 'w' | 'd'     => println!("Moving around"),
        '0'..='9'                 => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _                         => println!("Something else"),
    }
}
```

A match guard causes the arm to match only if the condition is true. If the condition is false the match will continue checking later cases. Match guards are a syntax feature important and necessary when we wish to concisely express more complex ideas than patterns alone would allow.



```rust
let num = 4;

match num {
    x if x % 2 == 0 => println!("Even"),
    _ => println!("Odd"),
}
```

There are some specific characters are being used when in a pattern:
- `|` as an or
- `..` can expand as much as it needs to be
- `1..=5` represents an inclusive range
- `_` is a wild card

Another usage for extracting inner values is using `@` character:
```rust
let opt = Some(123);
match opt {
    outer @ Some(inner) => {
        println!("outer: {outer:?}, inner: {inner}");
    }
    None => {}
}

// outer: Some(123), inner: 123
```

You use `@` when you:
- Need to access the full value (outer).
- But also want to pull out parts of it (inner).

You also can use destructure matching with structs:

```rust
struct Foo {
    x: (u32, u32),
    y: u32,
}

#[rustfmt::skip]
fn main() {
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
    }
}
```

### Structs

Try `match &foo` and check the type of captures. The pattern syntax remains the same, but the captures become shared references. This is [match ergonomics](https://rust-lang.github.io/rfcs/2005-match-ergonomics.html) and is often useful with match self when implementing methods on an enum.

The same effect occurs with match `&mut foo`: the captures become exclusive references.

### Enums

Like tuples, enums can also be destructured by matching:

Patterns can also be used to bind variables to parts of your values. This is how you inspect the structure of your types. Let us start with a simple enum type:

```rust
enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

fn main() {
    let n = 100;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}
```

Here we have used the arms to destructure the `Result` value. In the first arm, `half` is bound to the value inside the `Ok` variant. In the second arm, `msg` is bound to the error message.

## Let Control Flow

### `if let` Statement

In Rust, `if let` is a way to match a pattern in a simpler and more concise way than using a full match expression, when you only care about one pattern.

Imagine you have an `Option<T>`, like `Some(5)`, and you want to do something only if it’s Some. With match, you’d write:

```rust
let opt = Some(5);
match opt {
    Some(x) => println!("The number is {x}"),
    None => {}
}
```

That works, but it’s a bit long. If you’re only interested in the Some(x) case, you can write:

```rust
let opt = Some(5);
if let Some(x) = opt {
    println!("The number is {x}");
}
```

- Unlike `match`, `if let` does not have to cover all branches. This can make it more concise than `match`.
- A common usage is handling Some values when working with Option.
- Unlike match, if let does not support guard clauses for pattern matching.

### `while let` Statements

There is a [`while let`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-pattern-loops) variant which repeatedly tests a value against a pattern:

```rust
fn main() {
    let mut name = String::from("Comprehensive Rust 🦀");
    while let Some(c) = name.pop() {
        dbg!(c);
    }
    // (There are more efficient ways to reverse a string!)
}
```

```rust
// This 
while let Some(x) = some_option {
    // do stuff
}

// Is like
loop {
    match some_option {
        Some(x) => {
            // do stuff
        }
        _ => break,
    }
}
```

### `let else` Statements

For the common case of matching a pattern and returning from the function, use `let else`. The "else" case must diverge (return, break, or panic - anything but falling off the end of the block).

```rust
fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let Some(s) = maybe_string else {
        return Err(String::from("got None"));
    };

    let Some(first_byte_char) = s.chars().next() else {
        return Err(String::from("got empty string"));
    };

    let Some(digit) = first_byte_char.to_digit(16) else {
        return Err(String::from("not a hex digit"));
    };

    Ok(digit)
}
```

## Exercise: Expression Evaluation

Let’s write a simple recursive evaluator for arithmetic expressions.

An example of a small arithmetic expression could be 10 + 20, which evaluates to 30. A bigger and more complex expression would be (10 * 9) + ((3 - 4) * 5), which evaluate to 85.

In code, we will represent the tree with two types:
```rust
/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}
```

```
#[test]
#[ignore]
fn test_value() { .. }
```
```rust
/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}

fn eval(e: Expression) -> i64 {
    todo!()
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), 19);
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        30
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        85
    );
}

#[test]
fn test_zeros() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(0)),
            right: Box::new(Expression::Value(0))
        }),
        0
    );
}

#[test]
fn test_div() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(2)),
        }),
        5
    )
}
```
