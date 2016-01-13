//! This crate allows one-liners to read from a terminal
//! A minimal example to get an i32 from the command line is
//!
//! ```rust,ignore
//! #[macro_use] extern crate text_io;
//! fn main() {
//!     let i: i32 = read!();
//! }
//! ```
//!
//! The `read!()` macro will always read until the next ascii whitespace character
//! (`\n`, `\r`, `\t` or space).
//!
//! Any type that implements the `FromStr` trait can be read with the `read!` macro.
//!
//! # Advanced
//! Text parsing can be done similar to `println!` by adding a format string
//! to the macro:
//!
//! ```rust,ignore
//! let i: i32 = read!("The answer: {}!");
//! ```
//!
//! This will read `"The answer: "`, then an integer, then an exclamation mark. Any deviation from
//! the format string will result in a panic.
//!
//! Note: only a single value can be read per `read!` invocation.

/// All text input is handled through this macro
#[macro_export]
macro_rules! read(
    () => { read!("{}") };
    ($text:expr) => { read!($text, std::io::stdin().bytes().map(|c| c.unwrap())) };
    ($text:expr, $input:expr) => {{
        use std::io::Read;
        use std::str::{FromStr, from_utf8};
        // typesafe macros :)
        let text: &'static str = $text;
        let stdin: &mut Iterator<Item = u8> = &mut ($input);

        let mut text = text.bytes();
        let value;
        loop { match text.next() {
            Some(b'{') => match text.next() {
                Some(b'{') => assert_eq!(Some(b'{'), stdin.next()),
                Some(b'}') => {
                    let s: Vec<u8> = match text.next() {
                        Some(c) => stdin.take_while(|&ch| ch != c).collect(),
                        None => stdin.take_while(|ch| !b"\t\r\n ".contains(ch)).collect(),
                    };
                    let s = std::str::from_utf8(&s).unwrap();
                    value = FromStr::from_str(s).unwrap();
                    for c in text {
                        assert_eq!(Some(c), stdin.next());
                    }
                    break;
                }
                Some(_) => panic!("found bad curly brace"),
                None => panic!("found single open curly brace at the end of the format string"),
            },
            Some(c) => assert_eq!(Some(c), stdin.next()),
            None => panic!("Bad read! format string: did not contain {{}}"),
        } }
        value
    }};
);
