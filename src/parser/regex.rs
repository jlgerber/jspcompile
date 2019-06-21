use nom::{
    IResult,
    branch::alt,
    sequence::{tuple,preceded, delimited},
    bytes::complete::{tag},
    combinator::{ map, },
    //error::ErrorKind,
    character::complete::{char, space0, multispace0, alphanumeric1,},
};

use crate::helpers::*;



#[derive(Debug, PartialEq, Eq)]
pub enum Regex {
    Simple{
        name:String, 
        value: String
    },
    Complex{
        name: String, 
        positive: String, 
        negative: String
    },
}

// parse simple regex - that is:
// num_under =   "[0-9_]+"
fn parse_regex_simple(input: &str) -> IResult<&str,  Regex> {
    map ( 
            tuple((
                preceded(space0, variable),
                preceded(space0, char('=')), 
                delimited( space0, quoted_regex_str, multispace0) 
            )),
        | item| {
            println!("here");
            let (variable,_,re) = item ;
             Regex::Simple{name: variable.to_string(), value: re.to_string()}
        } 
    ) 
    (input)
}


#[cfg(test)]
mod parse_regex_simple {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn can_parse_regex_simple() {
        let result = parse_regex_simple(r#" foobar = "[a-zA-Z]" "#);
        assert_eq!(result, Ok( ("", Regex::Simple{name:"foobar".to_string(), value: "[a-zA-Z]".to_string()} ) ) );
    }

    #[test]
    fn can_parse_regex_simple_with_return() {
        let result = parse_regex_simple(r#" foobar = "[a-zA-Z]" 
        "#);
        assert_eq!(result, Ok( ("", Regex::Simple{name:"foobar".to_string(), value: "[a-zA-Z]".to_string()} ) ) );
    }

    #[test]
    fn can_parse_regex_simple_with_carriage_return() {
        let result = parse_regex_simple(r#" foobar = "[a-zA-Z]" 
        "#);
        assert_eq!(result, Ok( ("", Regex::Simple{name:"foobar".to_string(), value: "[a-zA-Z]".to_string()} ) ) );
    }

    #[test]
    fn fails_regex_simple_missing_quote() {
        let result = parse_regex_simple(r#" foobar = "[a-zA-Z] "#);
        assert_eq!(result, Err(nom::Err::Error((" ", ErrorKind::Tag)))) ;
    }

    #[test]
    fn fails_regex_simple_space() {
        let result = parse_regex_simple(r#" foobar = "[a-zA-Z] " "#);
        assert_eq!(result, Err(nom::Err::Error((" \" ", ErrorKind::Tag)))) ;
    }
}

// parse complex regex, which has positive and negative matches
// shot =   "[0-9_a-zA-Z]+" "(etc|SHARED|lib)"
fn parse_regex_complex(input: &str) -> IResult<&str,  Regex> {
    map ( 
            tuple((
                preceded(space0, variable),
                preceded(space0, char('=')), 
                preceded( space0, quoted_regex_str) ,
                delimited( space0, quoted_regex_str, multispace0) 
            )),
        | item| {
            let (variable,_,pos,neg) = item;

             Regex::Complex{
                 name: variable.to_string(), 
                 positive: pos.to_string(), 
                 negative: neg.to_string()
            }
        } 
    ) 
    (input)
}


#[cfg(test)]
mod parse_regex_complex {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn can_parse_regex_complex() {
        let result = parse_regex_complex(r#" foobar = "[a-zA-Z]" "(hello|world)" "#);
        assert_eq!(result, 
            Ok( 
                (
                    "", 
                    Regex::Complex{
                        name:"foobar".to_string(), 
                        positive: "[a-zA-Z]".to_string(), 
                        negative:"(hello|world)".to_string() 
                    } 
                ) 
            ) 
        );
    }

    #[test]
    fn can_parse_regex_complex_with_return() {
        let result = parse_regex_complex(r#" foobar = "[a-zA-Z]" "(hello|world)"
        "#);
        
        assert_eq!(result, 
            Ok( 
                (
                    "", 
                    Regex::Complex{
                        name:"foobar".to_string(), 
                        positive: "[a-zA-Z]".to_string(), 
                        negative:"(hello|world)".to_string() 
                    } 
                ) 
            ) 
        );    }


    #[test]
    fn fails_parse_regex_complex_missing_quote() {
        let result = parse_regex_complex(r#" foobar = "[a-zA-Z]" "(hello|world) "#);
        assert_eq!(result, Err(nom::Err::Error((" ", ErrorKind::Tag)))) ;
    }

    #[test]
    fn fails_parse_regex_complex_space() {
        let result = parse_regex_complex(r#" foobar = "[a-zA-Z] " "(hello|world)" "#);
        assert_eq!(result, Err(nom::Err::Error((" \" \"(hello|world)\" ", ErrorKind::Tag)))) ;
    }
}

/// Parse regex line. Could be either simple or complex
pub fn parse_regex(input: &str) -> IResult<&str,  Regex> {
    alt((
        parse_regex_complex,
        parse_regex_simple,
    ))(input)
}

#[cfg(test)]
mod parse_regex {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn can_parse_regex_complex() {
        let result = parse_regex(r#" foobar = "[a-zA-Z]" "(hello|world)" "#);
        assert_eq!(result, 
            Ok( 
                (
                    "", 
                    Regex::Complex{
                        name:"foobar".to_string(), 
                        positive: "[a-zA-Z]".to_string(), 
                        negative:"(hello|world)".to_string() 
                    } 
                ) 
            ) 
        );
    }

    #[test]
    fn can_parse_regex_simple() {
        let result = parse_regex(r#" foobar = "[a-zA-Z]" "#);
        assert_eq!(result, Ok( ("", Regex::Simple{name:"foobar".to_string(), value: "[a-zA-Z]".to_string()} ) ) );
    }
}
