# CHANGELOG

## future
### New Features
- Add option to disallow certain background color for texels.
- Add functions `raw::request` and `raw::request_ambiguous` with feature
  `events`.
- Add module `raw::request` which handles requests and response to specific
  codes.

### Breaking changes
- Move the module `error` directly into the root of the crate.
- Rename `ToColorStr` to `ToAnsiColorStr` and its method to `to_ansi_color_str`
  and rename `FromColorStr` to `FromAnsiColorStr` and its method to
  `from_ansi_color_str`.

### Changes
- Finalize documentation of the module `codes`.
- Document the modules `error` and `proc::err`.
- Document items in the root of the crate.
- Partially document the module `rgb`.

## v3.0.1
### Fixes
- Fix blue tint in texels.

## v3.0.0
### Features
- Support more mouse button events.
- Add `Terminal::println`.

### Changes
- Improve documentation for some codes.
- Make following methods on `Terminal` available without the feature `events`:
  `is_out_terminal`, `is_in_terminal`, `print`, `flushed`.

### Breaking changes
- Remove Rect and Rgb in favor of minlin data types.

### Fixes
- Fix parsing of mouse scroll events.
- Don't panic on unknown mouse button event.
- Properly interpret newlines in print functions with raw terminal.

## v2.1.2
### New features
- Readers support `Ctrl+Backspace` and `Ctrl+Delete` for deleting whole words.

### Changes
- Improve documentation for some codes.
- `reset_terminal` will now also reset line wrapping.

### Fixes
- Fix codes `ENABLE_LINE_WRAP` and `DISABLE_LINE_WRAP`.

## v2.1.1
### New features
- Add codes `ERASE_BUFFER`, `CLEAR` and `MOVE_HOME`.

### Changes
- Fix name type of `codes::request_selectoin` to `codes::request_selection`.
- Improve documentation for some codes.

### Fixes
+ Fix `ERASE_ALL`.
+ Some codes macros would return `String` even if all arguments were literals.
+ Fix panic when using raw api on windows.

## v2.0.0
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
- Add generic code macros: `graphic`, `osc`, `enable` and `disable`.
- Add osc codes: `request_color_code!`, `reset_color_code!`,
  `define_color_code`, `set_default_fg_color`, `set_default_bg_color`,
  `set_cursor_color`, `RESET_ALL_COLOR_CODES`, `RESET_DEFAULT_FG_COLOR`,
  `RESET_DEFAULT_BG_COLOR`, `RESET_CURSOR_COLOR`, `REQUEST_DEFAULT_FG_COLOR`,
  `REQUEST_DEFAULT_BG_COLOR`, `REQUEST_CURSOR_COLOR`, `REQUEST_SELECTION`,
  `request_selection` and `set_selection`. Also parse their respective
  responses.
- Reader now supports paste with `Ctrl+v`.
+ Move around some logic around sixels.
+ Refactor readers.
+ Split into features.
+ Windows support (untested).
+ Make `Terminal` generic.
+ Make `TermRead` more generic.
+ Implement `Eq` for `AmbigousEvent` and all sub structs.
+ Support control characters in `TermRead`.
+ Use the rgb type for gradient.
+ Implement new methods for rgb.
- Fix publicity of macro `codes::move_up`.
- Macros in `codes` now evaluate their arguments only once. (also fixes
  affected codes in color macros)
- Fix `move_to` and `mt` in color marcros.
- Fix `delete_lines`, `insert_columns`, `set_down` and `set_up` codes.
- Recognize amiguity with `Ctrl+Delete` and `Alt+d`.
- Fix name of `codes::OSC`.
- Fix codes `ENABLE_MOUSE_XY_ALL_TRACKING` and `DISABLE_MOUSE_XY_ALL_TRACKING`.
- Fix panic when terminal would send some invalid URXVT sequences.
+ Add unit tests.

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
