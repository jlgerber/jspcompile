

use nom::{
    IResult,
    branch::alt,
    sequence::{tuple,preceded, delimited},
    bytes::complete::{tag},
    combinator::{ map, },
    error::ErrorKind,
    character::complete::{char, space0, multispace0, alphanumeric1,},
};

use crate::{helpers::*, components::ParseResult};



pub mod header;
pub use header::{parse_section_header};

pub mod regex;
pub use regex::{parse_regex};

pub mod node;
pub use node::{ parse_node};

pub mod edge;
pub use edge::{parse_edges};

pub mod comment;
pub use comment::parse_comment;

pub fn parse(input: &str) -> IResult<&str, ParseResult> {
    alt((
        parse_comment,
        parse_section_header,
        parse_regex,
        parse_node,
        parse_edges,
    ))(input)
}