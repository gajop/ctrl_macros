# README

Control macros for `Option` and `Result`.

Helps you change this:
```rust
fn returns_early_with_value(input: Result<i32, String>) {
    // Early return when something is wrong.
    let input: i32 = match input {
        Ok(value) => value,
        Err(_) => return,
    };

    // Use it normally below...
    println!("Input is: {}", input);
}
```

into

```rust
use ctrl_macros::{ok_or_return};

fn returns_early_with_value(input: Result<i32, String>) {
    let input: i32 = ok_or_return!(input);
    println!("Input is: {}", input);
}
```

## Examples

```rust
let x = ok_or!(x, return 42);
```

```rust
for i in 0..5 {
    let x = ok_or_continue!(x);
}
```

```rust
for i in 0..5 {
    let x = ok_or_break!(x);
}
```

and `some_or` equivalents
```rust
some_or!
some_or_return!
some_or_continue!
some_or_break!
```

# License

`ctrl_macros` is dual licensed under either:

- MIT license ([LICENSE-MIT](docs/LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0, ([LICENSE-APACHE](docs/LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
