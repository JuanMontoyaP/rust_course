# Comprehensive Rust

## What is Rust?
Rust is a statically compiled language programming which had its 1.0 release in 2015. It is used for a wide range of devices:
- Firmware and boot loaders.
- Smart displays.
- Mobile phones.
- Desktops.
- Servers.

It offers: high flexibility, high level of control, can be scaled down for devises as microcontrollers, does not have runtime or garbage collection and focuses on reliability and safety without sacrificing performance.

Macros being "hygienic" means they don't accidentally capture identifiers from the scope they are used in. Rust macros are actually only [partially hygienic](https://lukaswirth.dev/tlborm/decl-macros/minutiae/hygiene.html).

Rust is multi-paradigm. For example, it has powerful [object-oriented](https://doc.rust-lang.org/book/ch18-00-oop.html) programming features, and, while it is not a functional language, it includes a range of [functional concepts](https://doc.rust-lang.org/book/ch13-00-functional-features.html).

## Table of Content
1. [Types and Values](./fibonacci/README.md)
2. [Control Flow Basics](./collatz_sequence/README.md)


# Bibliography
- [Comprehensive Rust.](https://google.github.io/comprehensive-rust/control-flow-basics.html)