use nom::{
    branch::alt,
    bytes::complete::{take_till, take_while1},
    character::{complete::space1, streaming::char},
    multi::many0,
};
use std::fs::read_to_string;

use nom::{character::is_digit, sequence::delimited, IResult};

fn number(s: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while1(is_digit)(s)
}

/// STRING := " character+ "
fn string(s: &[u8]) -> IResult<&[u8], &[u8]> {
    delimited(char('"'), take_till(|c| c == b'"'), char('"'))(s)
}

/// LIST := "(" (NUMBER | STRING)* ")"
fn list(s: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    delimited(char('('), many0(alt((number, space1, string))), char(')'))(s)
}

/// expr := LIST | ATOM
///
/// LIST := ( expr+ )
/// ATOM := NUMBER | STRING | VARIABLE | FUNCTION
fn main() {
    let s = read_to_string("test.v").unwrap();
    let (rest, out) = list(s.as_bytes()).unwrap();
    dbg!(String::from_utf8_lossy(rest));
    dbg!(out
        .into_iter()
        .map(|cs| String::from_utf8_lossy(cs))
        .collect::<Vec<_>>());
}
