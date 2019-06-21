/*

use nom::{
    IResult,
    branch::alt,
    sequence::{tuple,preceded, delimited},
    bytes::complete::{tag},
    combinator::{ map, },
    error::ErrorKind,
    character::complete::{char, space0, multispace0, alphanumeric1,},
};

use crate::helpers::*;

*/

pub mod header;
pub use header::{Header, parse_section_header};

pub mod regex;
pub use regex::{Regex, parse_regex};

pub mod node;
pub use node::{Node, parse_node};

pub mod edge;
pub use edge::{Edge, parse_edges};

pub mod comment;
pub use comment::parse_comment;