# Termal
[![crates.io][version-badge]][crate]
[![donwloads][downloads-badge]][releases]

Rust library for working with terminal. One of the main features of this crate
are the convinient macros for colored printing to the terminal. Their names
follow the names of the normal macros but with added `c`, `ac` or `mc` after
the main macro name (e.g `print` -> `printc`, `println` -> `printcln`). The
macros add new features to the format string. The new format features appear
within braces `{}` that start with `'` (e.g. `{'}`). Within the braces you can
specify some commands to the terminal, such as color change. The macro will
process the format string at compile time to produce the escape codes for the
terminal.

This crate also contains many escape codes for comunicating with terminal.

Apart from this, this crate also supports reading events from terminal (enable
with feature `events`), provides custom readline implementation (enable with
`readers`, note that the lower level api for readline is unstable), allows
drawing images to terminal (feature `term_image` and optionally `image`) and
has api for determining apperent width of text in terminal (feature
`term_text`).

## Example
### With macro
```rust
use termal::*;

// you can use a special macro to inline the color codes, this will write
// italic text with yellow foreground and reset at the end.
printcln!("{'yellow italic}hello{'reset}");

// the macro also supports standard formatting
printcln!("{'yellow italic}{}{'reset}", "hello");

// you can also use short versions of the codes
printcln!("{'y i}{}{'_}", "hello");

// you can also use true colors with their hex codes
printcln!("{'#dd0 i}{}{'_}", "hello");
```

### Without macro
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
use termal::codes::*;

let a = move_to!(5, 7);
// expands to:
let a = "\x1b[5;7H";

let b = move_to!(2 + 3, 7);
// expands to:
let b = format!("\x1b[{};{}H", 2 + 3, 7);
```

If you know the values for the arguments you can also use the `*c` macros:
```rust
use termal::formatc;

// the spaces, or the lack of them is important
let a = formatc!("{'move_to5,7}");
```

### Gradients
Youn can create gradients with the function `termal::gradient`:
```rust
use termal::*;

// This will create foreground gradient from the rgb color `(250, 50, 170)`
// to the rgb color `(180, 50, 240)`
printcln!("{}{'_}", gradient("BonnyAD9", (250, 50, 170), (180, 50, 240)));
```

## How to use it
To see all the possible commands and uses see [docs][docs].

## How to get it
It is available on [crates.io][crate]:

### With cargo
```shell
cargo add termal
```

### Features
- `raw`: enable features for raw terminal.
- `term_image`: enables functionality for drawing images to terminal.
- `image`: enables `term_image` and dependency for `image` with impl for
  `Image` trait.
- `events`: enables support for reading events from raw terminal.
- `term_text`: enable features for basic parsing of ansi escape codes.
- `all`: enable all features.
- `readers`: enables custom readline.

## RAW terminal support
- Unix (linux)
- Windows (not well tested)

## Links
- **Author:** [BonnyAD9][author]
- **GitHub repository:** [BonnyAD/raplay][repo]
- **Package:** [crates.io][crate]
- **Documentation:** [docs.rs][docs]
- **My Website:** [bonnyad9.github.io][my-web]

[version-badge]: https://img.shields.io/crates/v/termal
[downloads-badge]: https://img.shields.io/crates/d/termal
[crate]: https://crates.io/crates/termal
[author]: https://github.com/BonnyAD9
[repo]: https://github.com/BonnyAD9/termal
[docs]: https://docs.rs/termal/latest/termal/
[my-web]: https://bonnyad9.github.io/
[releases]: https://github.com/BonnyAD9/termal/releases
