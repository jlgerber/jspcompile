use nom::{
    IResult,
    sequence::{tuple,preceded, delimited},
    bytes::complete::{tag},
    combinator::{ map, },
    character::complete::{ space0, multispace0, alphanumeric1,},
};

use crate::ParseResult;

pub fn parse_empty(input: &str) -> IResult<&str, ParseResult> {
    map(
        multispace0, 
        |_item: &str| {
            ParseResult::Empty
        }
    )
    (input)
}

