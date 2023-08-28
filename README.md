# Termal
Rust library for terminal features with ansi escape codes.

Currently the library only contains the ansii codes, but you can already use
them as a more readable way of using the ansi codes.

## Example
```rust
// Move cursor to position column 5 on line 7 and write 'hello' in italic
// yellow

use termal::codes::*;

println!("{}{YELLOW_FG}{ITALIC}hello{RESET}", move_to!(5, 7));
```

The macros such as `move_to!` can accept either literals or dynamic values.
Its main feature is that if you supply literals, it expands to a string
literal with the ansi code.
If you however supply dynamic values it expands to a `format!` macro:
```rust
let a = move_to!(5, 7);
// expands to:
let a = "\x1b[5;7H";

let b = move_to!(2 + 3, 7);
// expands to:
let a = format!("\x1b[{};{}H", 2 + 3, 7);
```
