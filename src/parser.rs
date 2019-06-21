use nom::{
    IResult,
    sequence::{tuple,preceded, delimited},
    bytes::complete::{tag},
    combinator::{ map, },
    //error::ErrorKind,
    character::complete::{char, space0, multispace0, alphanumeric1,},
};

use crate::helpers::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Header {
    Regex,
    Nodes,
    Graph,
    Unknown(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Regex {
    Simple{name:String, value: String},
}

/// Parse the section header of the format
/// [regex]
pub fn parse_section_header(input: &str) -> IResult<&str, Header> {
    map ( 
            tuple((
                preceded(space0,tag("[")),
                preceded(space0, alphanumeric1), 
                delimited( space0, tag("]"), multispace0) 
            )),
        | item| {
            let (_,header,_) = item ;
            match header {
                "regex" | "regexp" => Header::Regex,
                "nodes" | "node" => Header::Nodes,
                "graph" => Header::Graph,
                _ => Header::Unknown(header.to_string()),
            }
        } 
    ) 
    (input)
}

#[cfg(test)]
mod section_header {
    use super::*;
 
    #[test]
    fn can_parse_spaces_in_header() {
        let result = parse_section_header(" [ regex ]    ");
        assert_eq!(result, Ok(("",Header::Regex)));
    }

    #[test]
    fn can_parse_spaces_in_header_with_carriage_return_ending() {
        let result = parse_section_header(r#" [ regex ]    
        "#);
        assert_eq!(result, Ok(("",Header::Regex)));
    }

    #[test]
    fn can_parse_spaces_in_header_2() {
        let result = parse_section_header("[ regex ]");
        assert_eq!(result, Ok(("",Header::Regex)));
    }

    #[test]
    fn can_parse_no_space_header() {
        let result = parse_section_header("[regex]");
        assert_eq!(result, Ok(("",Header::Regex)));
    }


    #[test]
    fn can_parse_no_space_nodes() {
        let result = parse_section_header("[nodes]");
        assert_eq!(result, Ok(("",Header::Nodes)));
    }

    #[test]
    fn can_parse_no_space_node() {
        let result = parse_section_header("[node]");
        assert_eq!(result, Ok(("",Header::Nodes)));
    }

    #[test]
    fn can_parse_no_space_graph() {
        let result = parse_section_header("[graph]");
        assert_eq!(result, Ok(("",Header::Graph)));
    }

    #[test]
    fn can_parse_no_space_unknown() {
        let result = parse_section_header("[grapha]");
        assert_eq!(result, Ok(("",Header::Unknown("grapha".to_string()))));
    }
}

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

