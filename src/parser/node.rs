
use nom::{
    IResult,
    branch::alt,
    sequence::{tuple,preceded, delimited},
    bytes::complete::{tag},
    combinator::{ map, },
    //error::ErrorKind,
    character::complete::{char, space0, multispace0, },
};

use crate::helpers::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Simple(String),
    Pair{name: String, value: String},
    Regex{name: String, variable: String},
}

impl Node {

    pub fn new_pair<I>(name: I, value: I) -> Node 
    where
        I:Into<String> 
    {
        Node::Pair{
            name: name.into(),
            value: value.into()
        }
    }

    pub fn new_regex<I>(name: I, variable: I) -> Node 
    where 
        I:Into<String> 
    {
        Node::Regex {
            name: name.into(),
            variable: variable.into()
        }
    }
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
    //use nom::error::ErrorKind;

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
            let (var,_,val) = item ;
             Node::new_pair(var, val)
        } 
    ) 
    (input)
}


#[cfg(test)]
mod parse_node_pair {
    use super::*;
    //use nom::error::ErrorKind;

    #[test]
    fn can_parse_node_pair() {
        let result = parse_node_pair(r#"rd = RD "#);
        assert_eq!(result, Ok( ("", Node::new_pair("rd", "RD")) ) ) ;
    }

    #[test]
    fn can_parse_node_pair_with_return() {
        let result = parse_node_pair(r#" rd = RD
        "#);
        assert_eq!(result, Ok( ("", Node::new_pair("rd", "RD") ) ) );
    }
}


// parse simple node - that is:
// rd_node =   rd
fn parse_node_regex(input: &str) -> IResult<&str,  Node> {
    map ( 
            tuple((
                preceded(space0, variable),
                preceded(space0, char('=')), 
                delimited( space0, preceded(tag("$"),variable), multispace0) 
            )),
        | item| {
            let (var,_,val) = item ;
             Node::new_regex(var, val)
        } 
    ) 
    (input)
}


#[cfg(test)]
mod parse_node_regex {
    use super::*;
    //use nom::error::ErrorKind;

    #[test]
    fn can_parse_node_regex() {
        let result = parse_node_regex(r#"rd = $rdexpr "#);
        assert_eq!(result, Ok( ("", Node::new_regex("rd", "rdexpr")) ) ) ;
    }

    #[test]
    fn can_parse_node_pair_with_return() {
        let result = parse_node_regex(r#" rd = $rdexpr
        "#);
        assert_eq!(result, Ok( ("", Node::new_regex("rd", "rdexpr") ) ) );
    }
}