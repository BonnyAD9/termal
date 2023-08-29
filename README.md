# Termal
Rust library for terminal features with ansi escape codes.

Currently the library contains the ansii codes, and a special macro works only
for colors.

## Example
### With macro
```rust
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
let a = move_to!(5, 7);
// expands to:
let a = "\x1b[5;7H";

let b = move_to!(2 + 3, 7);
// expands to:
let a = format!("\x1b[{};{}H", 2 + 3, 7);
```

## Macro syntax
In there are now 3 macros: `formatc`, `printc` and `printcln`. They are
equivalent to `format`, `print` and `println` respectively.

### In all of them the same sintax applies:
- braces starting with `'` are color formats (e.g. `{'yellow}`)
- other braces are interpreted by the macro `format`

#### The color format can contain
- names with ascii letters (e.g. `yellow`)
- hex colors (e.g. `#FF125C`)

the names and hex colors are separated by spaces

##### Hex colors
The hex colors may have either 1, 2, 3 or 6 digits. They are interpreted as
follows:
- Single digit colors are interpreted as the digit repeated 6 times (e.g. `#B`
  is same as `#BBBBBB`)
- Two digit colors are interpreted as the two digits repeated 3 times (e.g.
  `#AB` is same as `#ABABAB`)
- Three digit colors are interpreted as each digit being repeated once (e.g.
  `#ABC` is same as `#AABBCC`)
- Six digit colors are interpreted as typical rgb value (that is `#RRGGBB`)

If you want to set the foreground color you just type the hex code (e.g.
`#ABCDEF`), if you want to set the background color you immidietly follow the
hex color by uncerscore (`_`) (e.g. `#ABCDEF_`)

##### List of names
Most of the names have aliases (e.g. writing `white` and `w` is the same).
Some can be reset, that is done by the same name but starting with underscore
(`_`). Some names may/must have arguments. These are numbers and are supplied
by directly writing them after the name, multiple arguments are separated by
commas. If the argument is optional it will have the default value written
next to it. The commas must be present even if there are no arguments. (e.g.
`mt5,7` is valid, `mt,7` is valid, `mt5,` is valid, `mt,` is valid, but `mt`
is not valid)

###### Ascii
- `bell`: console bell (create sound)
- `backspace`: move left by one
- `htab`, `tab`: horizontal tabulator
- `move_down_scrl`, `mds`: move down by one line scrolling if needed
- `newline`, `nl`: move to the start of the next line
- `vtab`: vertical tabulator
- `carriage_return` | `cr`: move to the start of the current line

###### Moving the cursor
- `move_to`, `mt`: moves the cursor to the given position, has two arguments,
  default values are `0`.
- `move_up`, `mu`: moves the cursor up by the given amount, has one argument,
  default value is `1`
- `move_down`, `md`: moves the cursor down by the given amount, has one
  argument, default value is `1`
- `move_right`, `mr`: moves the cursor right by the given amount, has one
  argument, default value is `1`
- `move_left`, `ml`: moves the cursor left by the given amount, has one
  argument, default value is `1`
- `set_down`, `sd`: moves the cursor to the start of line n lines down, has one
  argument, default value is `1`
- `set_up`, `su`: moves the cursor to the start of line n lines up, has one
  argument, default value is `1`
- `move_to_column`, `mc`: moves the cursor to the given x coordinate, has one
  argument, default value is `0`
+ `move_up_scrl`, `mus`: moves the cursor up by one line, scrolling if needed
+ `save_cur`, `save`, `s`: saves the current cursor position (single slot, not
  lifo)
+ `load_cur`, `loat`, `l`: loads the last saved cursor position

###### Erasing
- `erase_to_end`, `e_`: erases from the cursor to the end of the screen
- `erase_from_start`, `_e`: erases from the start of the screen to the cursor
- `erase_screen`, `_e_`: erases the whole screen
- `erase_all`, `e`: erases the whole screen and the scroll buffer
- `erase_ln_end`, `el_`: erases from the cursor to the end of the line
- `erase_ln_start`, `_el`: erases from the start of the line to the cursor
- `erase_line`, `erase_ln`, `_el_`, `el`: erases the current line

###### Styles and colors
+ `reset`, `_`: resets all colors and styles
- `bold`: sets style to bold
- `faint`, `f`: sets style to faint
- `italic`, `i`: sets style to italic
- `underline`, `u`: sets style to underline
- `blinking`, `blink`: sets style to blinking
- `inverse`: sets style to inverse (swap background and foreground)
- `invisible`, `invis`: sets the style to invisible (foreground and background
  are same)
- `striketrough`, `strike`: sets the style to striketrough
- `double_underline`, `dunderline`, `dun`: sets the style to double underline
+ `_bold`: resets bold and faint
+ `_italic`, `_i`: resets italic
+ `_underline`, `_u`: resets underline and double underline
+ `_blinking`, `_blink`: resets blinking
+ `_inverse`: resets inverse
+ `_invisible`, `_invis`: resets invisible
+ `_striketrough`, `_strike`: resets striketrough
- `black_fg`, `black`, `bl`: sets the foreground to black
- `white_fg`, `white`, `w`: sets the foreground to white
- `gray_fg`, `gray`, `gr`: sets the foreground to green
- `bright_gray_fg`, `bgray`, `bgr`: sets the foreground to bright gray
+ `red_fg`, `red`, `r`: sets the foreground to red
+ `green_fg`, `green`, `g`: sets the foreground to green
+ `yellow_fg`, `yellow`, `y`: sets the foreground to yellow
+ `magenta_fg`, `magenta`, `m`: sets the foreground to magenta
+ `cyan_fg`, `cyan`, `c`: sets the foreground to cyan
- `dark_red_fg`, `dred`, `dr`: sets the foreground to dark red
- `dark_green_fg`, `dgreen`, `dg`: sets the foreground to dark green
- `dark_yellow_fg`, `dyellow`, `dy`: sets the foreground to dark yellow
- `dark_magenta_fg`, `dmagenta`, `dm`: sets the foreground to dark magenta
- `dark_cyan_fg`, `dcyan`, `dc`: sets the foreground to dark cyan
+ `_fg`: resets the foreground color
- `black_bg`, `blackb`, `blb`: sets the background to black
- `white_bg`, `whiteb`, `wb`: sets the background to white
- `gray_bg`, `grayb`, `grb`: sets the background to green
- `bright_gray_bg`, `bgrayb`, `bgrb`: sets the background to bright gray
+ `red_bg`, `redb`, `rb`: sets the background to red
+ `green_bg`, `greenb`, `gb`: sets the background to green
+ `yellow_bg`, `yellowb`, `yb`: sets the background to yellow
+ `magenta_bg`, `magentab`, `mb`: sets the background to magenta
+ `cyan_bg`, `cyanb`, `cb`: sets the background to cyan
- `dark_red_bg`, `dredb`, `drb`: sets the background to dark red
- `dark_green_bg`, `dgreenb`, `dgb`: sets the background to dark green
- `dark_yellow_bg`, `dyellowb`, `dyb`: sets the background to dark yellow
- `dark_magenta_bg`, `dmagentab`, `dmb`: sets the background to dark magenta
- `dark_cyan_bg`, `dcyanb`, `dcb`: sets the background to dark cyan
+ `_bg`: resets the background
- `fg`: sets the foreground color to one of the 256 colors, has one argument
- `bg`: sets the background color to one of the 256 colors, has one argument

###### Other
- `line_wrap`, `wrap`: enable line wrapping
- `_line_wrap`, `_wrap`: disable line wrapping
+ `hide_cursor`, `nocur`: hide the cursor
+ `show_cursor`, `_nocur`: show the cursor
+ `save_screen`, `sscr`: saves the screen view
+ `load_screen`, `lscr`: restores the last saved screen view
+ `alt_buf`, `abuf`: enable alternative buffer
+ `_alt_buf`, `_abuf`: disable alternative buffer

###### Compound
- `clear`, `cls`: erases the screen and the buffer and moves the cursor to the
  topleft position (equivalent of `e mt,`)
