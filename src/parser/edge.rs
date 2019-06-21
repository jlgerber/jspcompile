use nom::{
    IResult,
    sequence::{tuple,preceded, delimited},
    bytes::complete::{tag},
    combinator::{ map},
    multi::many1,
    character::complete::{ space0, alphanumeric1,},
};

use crate::{helpers::*, Edge};


/// Parse input &str into a vector of edges. 
/// 
/// ```foo -> bar-> bla```
/// 
/// produces
/// ```vec![ Edge::new(foo,bar), Edge::new(bar, bla) ];```
pub fn parse_edges(input: &str) -> IResult<&str, Vec<Edge>> {
    map(
        tuple((
            delimited(space0, variable, space0),
            many1(
                preceded(
                    tag("->"),
                    delimited(space0, alphanumeric1, space0)
                )
            ),
        )),
        |item| {
            let (first, rest) = item ;
            let mut rval = Vec::with_capacity(rest.len());
            let mut node1 = first;
            for node2 in rest {
                rval.push(
                    Edge::new(node1, node2)
                );
                node1 = node2;
            }
            rval
        }
    )(input)
}

#[cfg(test)]
mod parse_edges {
    use super::*;
 
    #[test]
    fn can_parse_edge() {
        let result = parse_edges(" foo->bar");
        assert_eq!(result, Ok(("",vec![Edge::new("foo", "bar")])));
    }

    #[test]
    fn can_parse_spaces_in_header_with_space_ending() {
        let result = parse_edges(r#" foo -> bar   "#);
        assert_eq!(result, Ok(("",vec![Edge::new("foo", "bar")])));
    }


    #[test]
    fn can_parse_edges_2() {
        let result = parse_edges(" foo->bar -> bla ");
        assert_eq!(
            result, 
            Ok(("",
                vec![
                    Edge::new("foo", "bar"),
                    Edge::new("bar", "bla"),
                ]
        )));
    }


    #[test]
    fn can_parse_edges_3() {
        let result = parse_edges(" foo->bar -> bla -> flarg  ");
        assert_eq!(
            result, 
            Ok(("",
                vec![
                    Edge::new("foo", "bar"),
                    Edge::new("bar", "bla"),
                    Edge::new("bla", "flarg"),
                ]
        )));
    }

    #[test]
    fn can_parse_edges_4() {
        let result = parse_edges(" foo->bar -> bla -> flarg  -> picklerick ");
        assert_eq!(
            result, 
            Ok(("",
                vec![
                    Edge::new("foo", "bar"),
                    Edge::new("bar", "bla"),
                    Edge::new("bla", "flarg"),
                    Edge::new("flarg", "picklerick"),

                ]
        )));
    }
    
}


/*
// foo -> bar
fn parse_edge(input: &str) -> IResult<&str, Edge> {
    map ( 
        tuple((
            preceded(space0, variable),
            preceded( space0, tag("->")),
            delimited( space0, variable, multispace0) ,
            
        )),
        | item| {
            let (from, _, to) = item;
            Edge::new(from,to)
        } 
    ) 
    (input)
}


#[cfg(test)]
mod parse_edge {
    use super::*;
 
    #[test]
    fn can_parse_edge() {
        let result = parse_edge(" foo->bar   ");
        assert_eq!(result, Ok(("",Edge::new("foo", "bar"))));
    }

    #[test]
    fn can_parse_spaces_in_header_with_carriage_return_ending() {
        let result = parse_edge(r#" foo -> bar   
        "#);
        assert_eq!(result, Ok(("",Edge::new("foo", "bar"))));
    }
}

/// Parse multiple edes 
pub fn parse_edges_(input: &str) -> IResult<&str, Vec<Edge>> {
    alt((
        parse_edges_four,
        parse_edges_three,
        parse_edges_two,
        parse_edges_one, 
    ))
    (input)
}



#[inline]
fn parse_edges_one(input: &str) -> IResult<&str, Vec<Edge>> {
    map ( 
        tuple((
            preceded(space0, variable),
            preceded( space0, tag("->")),
            delimited( space0, variable, multispace0) ,
            
        )),
        | item| {
            let (from, _, to) = item;
            vec![Edge::new(from,to)]
        } 
    ) 
    (input)
}

#[inline]
fn parse_edges_two(input: &str) -> IResult<&str, Vec<Edge>> {
    map ( 
        tuple((
            preceded(space0, variable),
            preceded(
                preceded( space0, tag("->")),
                preceded(space0, variable)
            ),
            preceded(
                preceded( space0, tag("->")),
                delimited( space0, variable, multispace0) 
            ),
            
        )),
        | item| {
            let (from, to,to2) = item;
            vec![Edge::new(from,to), Edge::new(to, to2)]
        } 
    ) 
    (input)
}

#[inline]
fn parse_edges_three(input: &str) -> IResult<&str, Vec<Edge>> {
    map ( 
        tuple((
            preceded(space0, variable),
            preceded(
                preceded( space0, tag("->")),
                preceded(space0, variable)
            ),
            preceded(
                preceded( space0, tag("->")),
                preceded(space0, variable)
            ),
            preceded(
                preceded( space0, tag("->")),
                delimited( space0, variable, multispace0) 
            ),
            
        )),
        | item| {
            let (from, to,to2,to3) = item;
            vec![Edge::new(from,to), Edge::new(to, to2), Edge::new(to2, to3)]
        } 
    ) 
    (input)
}

#[inline]
fn parse_edges_four(input: &str) -> IResult<&str, Vec<Edge>> {
    map ( 
        tuple((
            preceded(space0, variable),
            preceded(
                preceded( space0, tag("->")),
                preceded(space0, variable)
            ),
            preceded(
                preceded( space0, tag("->")),
                preceded(space0, variable)
            ),
            preceded(
                preceded( space0, tag("->")),
                preceded(space0, variable)
            ),
            preceded(
                preceded( space0, tag("->")),
                delimited( space0, variable, multispace0) 
            ),
            
        )),
        | item| {
            let (from, to,to2,to3, to4) = item;
            vec![
                Edge::new(from,to), 
                Edge::new(to, to2), 
                Edge::new(to2, to3), 
                Edge::new(to3, to4)
            ]
        } 
    ) 
    (input)
}

#[inline]
fn parse_edge_back(input: &str) -> IResult<&str, &str> {
    preceded(
        tag("->"),
        preceded(space0, alphanumeric1)
    )(input)
}
*/