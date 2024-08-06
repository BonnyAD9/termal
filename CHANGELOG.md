# CHANGELOG

## v2.0.0
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
