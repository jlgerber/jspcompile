use nom::{
    IResult,
    sequence::{tuple, Tuple, terminated, preceded, delimited},
    bytes::complete::{tag},
    
    combinator::{ map, all_consuming},
   
    character::complete::{space0, alphanumeric1},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Header {
    Regex,
    Nodes,
    Graph,
    Unknown(String),
}



fn header(input: &str) -> IResult<&str, Header> {
    map (
            
            tuple((
                preceded(space0,tag("[")),
                preceded(space0, alphanumeric1), 
                delimited( space0, tag("]"), space0) 
            )),
        | item| {
            println!("here");
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
mod regex_header {
    use super::*;
 
    #[test]
    fn can_parse_spaces_header() {
        let result = header(" [ regex ]    ");
        assert_eq!(result, Ok(("",Header::Regex)));
    }

    #[test]
    fn can_parse_spaces2_header() {
        let result = header("[ regex ]");
        assert_eq!(result, Ok(("",Header::Regex)));
    }

    #[test]
    fn can_parse_no_space_header() {
        let result = header("[regex]");
        assert_eq!(result, Ok(("",Header::Regex)));
    }


    #[test]
    fn can_parse_no_space_nodes() {
        let result = header("[nodes]");
        assert_eq!(result, Ok(("",Header::Nodes)));
    }

    #[test]
    fn can_parse_no_space_node() {
        let result = header("[node]");
        assert_eq!(result, Ok(("",Header::Nodes)));
    }

    #[test]
    fn can_parse_no_space_graph() {
        let result = header("[graph]");
        assert_eq!(result, Ok(("",Header::Graph)));
    }

    #[test]
    fn can_parse_no_space_unknown() {
        let result = header("[grapha]");
        assert_eq!(result, Ok(("",Header::Unknown("grapha".to_string()))));
    }
}