use nom::{
    IResult,
    sequence::{preceded},
    bytes::complete::{tag},
    combinator::{rest},
    character::complete::{space0},
};



pub fn parse_comment(input: &str) -> IResult<&str, &str> {
    preceded(
        preceded(
            space0, 
            tag("#")
        ),
        rest
    )
    (input)
}


#[cfg(test)]
mod comment {
    use super::*;
 
    #[test]
    fn can_parse_comment() {
        let c = parse_comment(" # this is a comment");
        assert_eq!(c, Ok(("", " this is a comment")));
    }

 
    #[test]
    fn can_parse_comment_2() {
        let c = parse_comment(" # this is a comment    ");
        assert_eq!(c, Ok(("", " this is a comment    ")));
    }
}
