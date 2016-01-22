// Copyright 2016 ICHIKAWA, Yuji
// License: MIT

//! SGF(Smart Game Format) parser

extern crate regex;
use std::fmt;

pub mod sgf_node;
mod parser;

pub use sgf_node::*;
use parser::*;

/// Parses a SGF string and returns a SGF Collection, that is a vector of SGF root nodes
///
/// # Example
///
/// ```
/// use sgf::sgf_parse;
///
/// let nodes = sgf_parse("(;CA[UTF-8]FF[4])");
/// ```
///
#[inline(always)]
pub fn sgf_parse<'input>(input: &'input str) -> ParseResult<Vec<SgfNode>> {
    collection(input)
}

/// Writes a SGF collection(a vector of SGF game tree) in SGF format to f.
pub fn write_sgf<T: fmt::Write>(f: &mut T, collection: &Vec<SgfNode>) -> fmt::Result {
    collection.iter().fold(Ok(()), |acc, item|
        acc.and(write!(f, "(")).and(item.fmt_sgf(f)).and(write!(f, ")"))
    )
}

#[test]
fn test_parse1() {
    let result = sgf_parse("(;CA[UTF-8]FF[4])");
    assert!(result.is_ok());
}

#[test]
fn test_parse2() {
    let node = &sgf_parse("(;FF[4]C[root](;C[a];C[b](;C[c])
        (;C[d];C[e]))
        (;C[f](;C[g];C[h];C[i])
        (;C[j])))").unwrap()[0];
    assert!(node.children.len() == 2 && node.children[0].children[0].children.len() == 2);
}

#[test]
fn test_parse_fail() {
    let result = sgf_parse("(;CA[UTF-8]FFF[4])");
    assert!(result.is_err());
}

#[test]
fn test_write_sgf() {
    use std::fmt::Write;

    let sgf = "(;FF[4];C[a];C[b](;C[c])(;C[d];C[e])(;C[f](;C[g];C[h];C[i])(;C[j])))";
    let collection = sgf_parse(sgf).unwrap();
    let mut output = String::new();
    write_sgf(&mut output, &collection);
    assert_eq!(&output, sgf);
}
