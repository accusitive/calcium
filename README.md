[![CI](https://github.com/accusitive/calcium/actions/workflows/ci.yml/badge.svg)](https://github.com/accusitive/calcium/actions/workflows/ci.yml)
# Calcium
NOTE: Current development is being done on [Next Branch](https://github.com/accusitive/calcium/tree/next)
Calcium is implemented in rust, using [bison](https://github.com/iliabylich/rust-bison-skeleton) for the parser and llvm for the backend.
# Goals
- [ ] Type checking
- [ ] Control flow (In progress)
- [ ] Generics
- [ ] Json Parser example
- [ ] Package manager/better import system
- [ ] Self hosting
# Syntax
The syntax is subject to change at any moment, but current a simple function to create an object and return some data looks like
```rust
struct TwoNumbers {
    first: i32,
    second: i32
}
fn main(): i32 {
    # Anything here
    let z: _ = new TwoNumbers(50, 10);
    return z.first
}
fn first(self: TwoNumbers): i32 {
    return self.first
}
fn second(self: TwoNumbers): i32 {
    return self.second
}
```
Its quite similar to rust as you can see, since it is my favorite language.
# License
Everything in this repository is licensed under MIT.