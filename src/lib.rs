// Copyright 2016 ICHIKAWA, Yuji
// License: MIT

//! SGF(Smart Game Format) parser

extern crate regex;

pub mod sgf_node;
mod parser;

pub use sgf_node::*;
use parser::*;

/// Returns a vector of SGF root nodes
/// # Example
///
/// ```
/// let nodes = sgf_parse("(;CA[UTF-8]FF[4])");
/// ```
#[inline(always)]
pub fn sgf_parse<'input>(input: &'input str) -> ParseResult<Vec<SgfNode>> {
    collection(input)
}
