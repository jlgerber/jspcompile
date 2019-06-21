use nom::{
    IResult,
    sequence::{tuple,preceded, delimited},
    bytes::complete::{tag},
    combinator::{ map, },
    character::complete::{ space0, multispace0, alphanumeric1,},
};


#[derive(Debug, PartialEq, Eq)]
pub enum Header {
    Regex,
    Nodes,
    Graph,
    Unknown(String),
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
