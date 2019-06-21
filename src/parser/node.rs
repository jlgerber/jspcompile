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
    ReVar{name: String, variable: String},
    RegexSimple{name: String, re: String },
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

    pub fn new_revar<I>(name: I, variable: I) -> Node 
    where 
        I:Into<String> 
    {
        Node::ReVar {
            name: name.into(),
            variable: variable.into()
        }
    }

    pub fn new_regexsimple<I>(name: I, re: I) -> Node 
    where 
        I:Into<String> 
    {
        Node::RegexSimple {
            name: name.into(),
            re: re.into()
        }
    }

}


/// Parse node
pub fn parse_node(input: &str) -> IResult<&str, Node> {
    alt((
        parse_node_pair,
        parse_node_revar,
        parse_node_regexsimple,
        parse_node_simple,
    ))
    (input)
}

#[cfg(test)]
mod parse_node {
    use super::*;
    //use nom::error::ErrorKind;

    #[test]
    fn can_parse_revar_simple() {
        let result = parse_node(r#" rd "#);
        assert_eq!(result, Ok( ("", Node::Simple("rd".to_string())) ) ) ;
    }

    #[test]
    fn can_parse_node_pair() {
        let result = parse_node(r#"rd = RD "#);
        assert_eq!(result, Ok( ("", Node::new_pair("rd", "RD")) ) ) ;
    }

    #[test]
    fn can_parse_node_revar() {
        let result = parse_node(r#"rd = $rdexpr "#);
        assert_eq!(result, Ok( ("", Node::new_revar("rd", "rdexpr")) ) ) ;
    }

    #[test]
    fn can_parse_node_regexsimple() {
        let result = parse_node(r#"rd = "(foo|bar)" "#);
        assert_eq!(result, Ok( ("", Node::new_regexsimple("rd", "(foo|bar)")) ) ) ;
    }
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
    fn can_parse_revar_simple() {
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


// parse regex variable node. regex node references a named regex
// `rd_node =   $rd`
fn parse_node_revar(input: &str) -> IResult<&str,  Node> {
    map ( 
            tuple((
                // drops zero or more spaces in front of a variable (upper lower case number _-)
                preceded(space0, variable),
                // drop zero or more spaces in front of '='
                preceded(space0, char('=')), 
                // drop zero or more spaces around variable preceded by $ and drop zero or more spaces and returns
                delimited( space0, preceded(tag("$"),variable), multispace0) 
            )),
        | item| {
            let (var,_,val) = item ;
             Node::new_revar(var, val)
        } 
    ) 
    (input)
}


#[cfg(test)]
mod parse_node_revar {
    use super::*;
    //use nom::error::ErrorKind;

    #[test]
    fn can_parse_node_revar() {
        let result = parse_node_revar(r#"rd = $rdexpr "#);
        assert_eq!(result, Ok( ("", Node::new_revar("rd", "rdexpr")) ) ) ;
    }

    #[test]
    fn can_parse_node_pair_with_return() {
        let result = parse_node_revar(r#" rd = $rdexpr
        "#);
        assert_eq!(result, Ok( ("", Node::new_revar("rd", "rdexpr") ) ) );
    }
}


// parse regex variable node. regex node references a named regex
// `rd_node =   $rd`
fn parse_node_regexsimple(input: &str) -> IResult<&str,  Node> {
    map ( 
            tuple((
                // drops zero or more spaces in front of a variable (upper lower case number _-)
                preceded(space0, variable),
                // drop zero or more spaces in front of '='
                preceded(space0, char('=')), 
                // drop zero or more spaces around variable preceded by $ and drop zero or more spaces and returns
                delimited( space0, quoted_regex_str, multispace0) 
            )),
        | item| {
            let (var,_,val) = item ;
             Node::new_regexsimple(var, val)
        } 
    ) 
    (input)
}

#[cfg(test)]
mod parse_node_regexsimple {
    use super::*;
    //use nom::error::ErrorKind;

    #[test]
    fn can_parse_node_regexsimple() {
        let result = parse_node_regexsimple(r#"rd = "(foo|bar)" "#);
        assert_eq!(result, Ok( ("", Node::new_regexsimple("rd", "(foo|bar)")) ) ) ;
    }

    #[test]
    fn can_parse_node_regexsimplewith_return() {
        let result = parse_node_regexsimple(r#" rd = "[a-zA-Z0-1_-]"
        "#);
        assert_eq!(result, Ok( ("", Node::new_regexsimple("rd", "[a-zA-Z0-1_-]") ) ) );
    }
}