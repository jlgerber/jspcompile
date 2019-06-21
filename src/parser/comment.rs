use nom::{
    IResult,
    sequence::{preceded},
    bytes::complete::{tag},
    combinator::{rest,map},
    character::complete::{space0},
};

use crate::ParseResult;


pub fn parse_comment(input: &str) -> IResult<&str, ParseResult> {
    map(
        preceded(
            preceded(
                space0, 
                tag("#")
            ),
            rest
        ), 
        |item: &str| {
            ParseResult::Comment(item.to_string())
        }
    )
    (input)
}


#[cfg(test)]
mod comment {
    use super::*;
 
    #[test]
    fn can_parse_comment() {
        let c = parse_comment(" # this is a comment");
        assert_eq!(c, Ok(("", ParseResult::Comment(" this is a comment".to_string()))));
    }

 
    #[test]
    fn can_parse_comment_2() {
        let c = parse_comment(" # this is a comment    ");
        assert_eq!(c, Ok(("", ParseResult::Comment( " this is a comment    ".to_string()))));
    }
}
