

use nom::{
    IResult,
    branch::alt,
    combinator::{all_consuming },
};

use crate::{ components::ParseResult};


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

pub mod empty;
pub use empty::parse_empty;

pub mod metadata;
pub use metadata::parse_metadata;

fn parse_str(input: &str) -> IResult<&str, ParseResult> {
    all_consuming(
        alt((
            parse_edges,
            parse_comment,
            parse_section_header,
            parse_node,
            parse_regex,
            parse_empty,
        ))
    )(input)
}

pub fn start_parser(input: &str) -> IResult<&str, ParseResult> {
    all_consuming(
        alt((
            parse_comment,
            parse_section_header,
            parse_empty,
        ))
    )(input)
}

pub fn regex_parser(input: &str) -> IResult<&str, ParseResult> {
    all_consuming(
        alt((
            parse_comment,
            parse_section_header,
            parse_regex,
            parse_empty,
        ))
    )(input)
}

pub fn node_parser(input: &str) -> IResult<&str, ParseResult> {
    all_consuming(
        alt((
            parse_comment,
            parse_section_header,
            parse_node,
            parse_empty,
        ))
    )(input)
}

pub fn edge_parser(input: &str) -> IResult<&str, ParseResult> {
    all_consuming(
        alt((
            parse_edges,
            parse_comment,
            parse_empty,
        ))
    )(input)
}
pub fn parse(input: &str) -> Result<(), String> {
    match parse_str(input) {
        Ok(("", ParseResult::Comment(comment))) => println!("Comment {:?}", comment),
        Ok(("", ParseResult::Header(header)))   => println!("Header  {:?}", header),
        Ok(("", ParseResult::Regex(r)))         => println!("Regex   {:?}", r),
        Ok(("", ParseResult::Node(n)))          => println!("Node    {:?}", n),
        Ok(("", ParseResult::Edges(e)))         => println!("Edges   {:?}", e),
        Ok(("", ParseResult::Empty))            => println!(""),

        Err(e) => return Err(format!("Error {:?}", e)),
        _ => println!("unexpected result"),
    }
    Ok(())
}