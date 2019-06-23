use nom::{
    IResult,
    sequence::{preceded, delimited, separated_pair},
    bytes::complete::tag,
    combinator::map,
    error::ErrorKind,
    character::complete::space0,
};

use crate::{Metadata, MetadataComponent, helpers::variable};

pub fn parse_metadata(input: &str) -> IResult<&str, Metadata> {

    Err(nom::Err::Error(("NOT IMPLEMENTED YET", ErrorKind::Tag)))
}

fn parse_volume(input: &str) -> IResult<&str, MetadataComponent> {
    map(
        delimited(space0,tag("volume"), space0),
        |_item| {
            MetadataComponent::Volume
        }
    )(input)
}

#[cfg(test)]
mod volume_tests {
    use super::*;

    #[test]
    fn can_parse_volume_no_spaces() {
       let owner = parse_volume("volume");
       assert_eq!(owner, Ok(("", MetadataComponent::Volume))) ;
    }

    #[test]
    fn can_parse_volume_spaces() {
       let owner = parse_volume("  volume   ");
       assert_eq!(owner, Ok(("", MetadataComponent::Volume))) ;
    }
}


// owner : jgerber
fn parse_owner(input: &str) -> IResult<&str, MetadataComponent> {
    map(
        delimited(
            space0,
            separated_pair(
                tag("owner"),
                 preceded(space0,tag(":")), 
                 preceded(space0, variable)
            ), 
            space0,
        ),
        |item| {
            let (_, owner_name) = item;
            MetadataComponent::Owner(owner_name.to_string())
        }
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_owner_no_spaces() {
       let owner = parse_owner("owner:fred");
       assert_eq!(owner, Ok(("", MetadataComponent::Owner("fred".to_string())))) ;
    }

    #[test]
    fn can_parse_owner_spaces() {
       let owner = parse_owner("owner : fred");
       assert_eq!(owner, Ok(("", MetadataComponent::Owner("fred".to_string())))) ;
    }
    #[test]
    fn can_parse_owner_more_spaces() {
       let owner = parse_owner("  owner : fred  ");
       assert_eq!(owner, Ok(("", MetadataComponent::Owner("fred".to_string())))) ;
    }
}