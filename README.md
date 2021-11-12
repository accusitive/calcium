[![CI](https://github.com/accusitive/calcium/actions/workflows/ci.yml/badge.svg)](https://github.com/accusitive/calcium/actions/workflows/ci.yml)
# Calcium
Calcium is a new prgramming language i'm making for fun, with no real goals
Calcium is implemented in rust, using bison for the parser and llvm for the backend.
# Syntax
The syntax is subject to change at any moment, but current a simple function to add 2 numbers would look like
```rust
fn add(left: i32, right: i32): i32 {
    return left + right
}
```
Its quite similar to rust as you can see, since it is my favorite language.
