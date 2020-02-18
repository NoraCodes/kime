# Kime

[![Build Status](https://img.shields.io/travis/com/NoraCodes/kime/master?logo=travis)](https://travis-ci.com/NoraCodes/kime)
[![Crate](https://img.shields.io/crates/v/kime.svg)](https://crates.io/crates/kime)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.41+-blue.svg?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/crates/l/kime?color=blue)](#license)

[![asciicast WIP]()]()

A tiny, configurable [**ki**bi](https://github.com/ilai-deutel/kibi)-based **m**odal text
**e**ditor with UTF-8 support, incremental search, syntax highlighting, line numbers and
more.

## Installation

### With `cargo`

You can install Kime with [`cargo`](https://github.com/rust-lang/cargo/):

```bash
$ cargo install kime
```

Syntax highlighting configuration files are available in the [`config_example/syntax.d`](config_example/syntax.d)
directory of this repository. They need to be placed in one of the configuration directories mentioned in the
[Configuration/Syntax Highlighting](#syntax-highlighting) section.

For instance:

```bash
$ cd ~/repos
$ git clone https://github.com/NoraCodes/kime.git
$ mkdir -p ~/.config/kime
$ ln -sr ./kime/syntax ~/.config/kime/syntax.d
```

## Usage

```bash
# Start an new text buffer
$ kime
# Open a file
$ kime <file path>
```

## Keyboard Shortcuts

| Keyboard shortcut | Description                                                   |
| ----------------- | ------------------------------------------------------------- |
| Ctrl-F            | Incremental search; use arrows to navigate                    |
| Ctrl-S            | Save the buffer to the current file, or specify the file path |
| Ctrl-G            | Go to `<line number>[:<column number>]` position              |
| Ctrl-Q            | Quit                                                          |
| Ctrl-D            | Duplicate the current row                                     |

### Configuration

#### Global configuration

Kime can be configured using:
* A system-wide configuration file, located at `/etc/kime/config.ini`
* A user-level configuration file, located at:
  * `$XDG_CONFIG_HOME/kime/config.ini` if environment variable `$XDG_CONFIG_HOME` is defined
  * `~/.config/kime/config.ini` otherwise

Example configuration file:
```ini
# The size of a tab. Must be > 0.
tab_stop=4
# The number of confirmations needed before quitting, when changes have been made since the file.
# was last changed.
quit_times=2
# The duration for which messages are shown in the status bar, in seconds.
message_duration=3
# Whether to show line numbers.
show_line_numbers=true
```

#### Syntax Highlighting

Syntax highlighting can be configured using INI files located at:
* `/etc/kime/syntax.d/<file_name>.ini` for system-wide availability
* For user-level configuration files:
  * `$XDG_CONFIG_HOME/kime/syntax.d/<file_name>.ini` if environment variable `$XDG_CONFIG_HOME` is defined
  * `~/.config/kime/syntax.d/<file_name>.ini` otherwise

Kime is compatible with kibi syntax highlighting files and will fall back to the kibi
locations if highlighting files are not found in the kime locations.

Syntax highlighting configuration follows this format:

```ini
### /etc/kime/syntax.d/rust.ini ###
# Kime syntax highlighting configuration for Rust

name=Rust
extensions=rs
highlight_numbers=true
highlight_strings=true
singleline_comment_start=//
multiline_comment_delims=/*, */
; In Rust, the multi-line string delimiter is the same as the single-line string delimiter
multiline_string_delim="
; https://doc.rust-lang.org/book/appendix-01-keywords.html
keywords_1=abstract, as, async, await, become, box, break, const, continue, crate, do, dyn, else, enum, extern, false, final, fn, for, if, impl, in, let, loop, macro, match, mod, move, mut, override, priv, pub, ref, return, self, Self, static, struct, super, trait, true, try, type, typeof, unsafe, unsized, use, virtual, where, while, yield
keywords_2=i8, i16, i32, i64, i128, isize, u8, u16, u32, u36, u128, usize, f32, f64, bool, char, str
```

## Comparison with `kibi`

This project is built on top of `kibi`, adding the following features:

- nothing, yet.

## Comparison with `kilo`

`kibi` was inspired by [`kilo`](https://github.com/antirez/kilo), a text editor written by Salvatore Sanfilippo
(antirez) in C, and [this tutorial](https://viewsourcecode.org/snaptoken/kilo/) (also in C).

`kime` provides additional features:
- Support for UTF-8 characters
- Command to jump to a given row/column
- Handle window resize
- Parsing configuration files: global editor configuration, language-specific syntax highlighting configuration
- Display line numbers on the left of the screen; display file size in the status bar
- Syntax highlighting: multi-line strings
- "Save as" prompt when no file name has been provided
- Command to duplicate the current row
- Memory safety, thanks to Rust!
- Many bug fixes

## Dependencies

This project must remain tiny, so using advanced dependencies such as [`ncurses`](https://crates.io/crates/ncurses),
[`toml`](https://crates.io/crates/toml) or [`ansi-escapes`](https://crates.io/crates/ansi-escapes) would be cheating.

These dependencies provide safe wrappers around `libc` calls, to avoid using `unsafe` code as much as possible:

* `libc`
* `nix`
* `signal-hook`

In addition, `unicode-width` is used to determine the displayed width of Unicode characters. Unfortunately, there is no
way around it: the [unicode character width table](https://github.com/unicode-rs/unicode-width/blob/3033826f8bf05e82724140a981d5941e48fce393/src/tables.rs#L52)
is 230 lines long.

## Why Kime?

1. Porting the `kilo` source code from C to Rust and trying to make it idiomatic was interesting
2. Implementing new features while under the 1024-line constraint is a good challenge
3. Most importantly, I wanted to learn Rust and this was a great project to start (thanks Reddit for the idea)

## License

Kime is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for details.
