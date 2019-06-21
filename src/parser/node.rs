
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

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Simple(String),
    Pair{name: String, value: String},
}

/// Parse node
pub fn parse_node(input: &str) -> IResult<&str, Node> {
    alt((
        parse_node_pair,
        parse_node_simple
    ))
    (input)
}

// parse simple node - that is:
// rd_node =   rd
fn parse_node_simple(input: &str) -> IResult<&str,  Node> {
    map ( 
        delimited( space0, variable, multispace0) ,
        | item| {
            Node::Simple(item.to_string())
        } 
    ) 
    (input)
}


#[cfg(test)]
mod parse_node_simple {
    use super::*;
    use nom::error::ErrorKind;

    #[test]
    fn can_parse_regex_simple() {
        let result = parse_node_simple(r#" rd "#);
        assert_eq!(result, Ok( ("", Node::Simple("rd".to_string())) ) ) ;
    }

    #[test]
    fn can_parse_node_simple_with_return() {
        let result = parse_node_simple(r#" rd
        "#);
        assert_eq!(result, Ok( ("", Node::Simple("rd".to_string()) ) ) );
    }
}

// parse simple node - that is:
// rd_node =   rd
fn parse_node_pair(input: &str) -> IResult<&str,  Node> {
    map ( 
            tuple((
                preceded(space0, variable),
                preceded(space0, char('=')), 
                delimited( space0, variable, multispace0) 
            )),
        | item| {
            println!("here");
            let (var,_,val) = item ;
             Node::Pair{name:var.to_string(), value: val.to_string()}
        } 
    ) 
    (input)
}
