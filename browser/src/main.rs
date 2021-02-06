extern crate pom;
use pom::parser::*;
use pom::Parser;

use std::str::{self, FromStr};

fn main() {
    println!("Hello, world!");
    println!("{:?}", number().parse(b"a0334"));
}

fn number() -> Parser<u8, f64> {
    let integer = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
    integer
        .collect()
        .convert(str::from_utf8)
        .convert(|s| f64::from_str(&s))
}
