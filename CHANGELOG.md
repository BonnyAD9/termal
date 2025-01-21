# CHANGELOG

## Future
- New methods for `Terminal`: `has_input`, `wait_for_input`,
  `read_ambiguous_timeout`, `read_timeout`, `read_raw`, `read_raw_timeout`,
  `read_raw_single_timeout` and `read_byte_timeout`.
- Add texels - drawing images using charactes.
- Add option for nearest when scaling image.
- Add new codes.
- Parse mouse events (classic, SGR, UTF-8, URXVT).
- Parse focus events.
- Parse cursor position responses.
- Parse terminal name response.
- Parse terminal window parameters responses.
- Add code for setting the scroll region.
- Add codes to insert and delete lines, chars and columns.
- Add option for universal terminal reset and to regiester panic hook with that
  reset.
- Reader can now edit text.
- Reader can have prompt.
- More methods on reader.
- Add best effort readline functions: `prompt_to`, `prompt`, `read_line_to` and
  `read_line`.
- Add new codes: `DONT_LIMIT_PRINT_TO_SCROLL_REGION` and
  `LIMIT_PRINT_TO_SCROLL_REGION`.
- Add new methods to `Terminal` to make it work as generic.
- Add `WaitForIn` trait.
- Add `IoProvider` trait.
- Add `StdioProvider` trait.
- Add `ValueOrMut` to make `IoProvider` possible.
- Add `SS3` code.
- Add code `OVERLINE`, also add to print macros as `overline`/`ol`. (also add
  reset codes)
- Add codes to enable and disable inverse color in whole terminal.
- Add option to enable bracketed paste mode (pasted text is verbatim).
- Support bracketed paste mode in terminal.
- Add codes to set and reset the underline color (rgb an 256) and their codes
  to format macros (`uc`/`ucolor`, `#RGBu`, `_uc`/`_ucolor`).
- Add codes to change the character size for current line (
  `DUBLE_CHAR_HEIGHT_DOWN`, `DOUBLE_CHAR_HEIGHT_UP`, `DOUBLE_CHAR_WIDTH`,
  `RESET_CHAR_SIZE`)
+ Move around some logic around sixels.
+ Refactor readers.
+ Split into features.
+ Windows support (untested).
+ Make `Terminal` generic.
+ Make `TermRead` more generic.
+ Implement `Eq` for `AmbigousEvent` and all sub structs.
+ Support control characters in `TermRead`.
- Fix publicity of macro `codes::move_up`.
- Macros in `codes` now evaluate their arguments only once. (also fixes
  affected codes in color macros)
- Fix `move_to` and `mt` in color marcros.
- Fix `delete_lines`, `insert_columns`, `set_down` and `set_up` codes.
- Recognize amiguity with `Ctrl+Delete` and `Alt+d`.

## v1.2.2
- Fix `writemcln` macro.

## v1.2.1
- Support sixels
- Add `writecln`, `writec`, `writencln`, `writenc`, `writemcln` and `writemc`
  macros.

## ?
- Rename `TermText::to_string` to `to_string_cache`.

## v1.2.0
- Add raw mode for linux and support reading char by char.
- Add `TermText` that can be used to get information about string with control
  sequences.

## v1.1.0
- Allow additional comma in all the macros.
- Add automatic coloring macros: `printac`, `printacln`, `eprintac`,
  `eprintacln`

## v1.0.2
- Fix six digit hex RGB colors in proc macros

## v1.0.1
- Add conditionally formatting macros: `formatmc`, `printmc`, `printmcln`,
  `eprintmc`, `eprintmcln`

## v1.0.0
- Fix inline variable names in the `*c` macros
- New formatting macros: `eprintc`, `eprintcln`
- Unformatting macros: `formatnc`, `printnc`, `printncln`, `eprintnc`,
  `eprintncln`

## v0.1.0
The first version
- Color codes
- Macros: `formatc`, `printc`, `printcln`
- Gradients
