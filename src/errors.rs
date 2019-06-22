use failure::Fail;
use crate::State;
use nom;

#[derive(Debug, Fail)]
pub enum JSPTemplateError {
    #[fail(display = "Invalid State Transition from: {:?} to {:?}", _0, _1)]
    InvalidStateTransition(State, State),
    
    #[fail(display = "No valid next state for: {:?}", _0)]
    NoValidNextState(State),

    #[fail(display = "Placeholder")]
    Placeholder,

    #[fail(display = "DoneState")]
    DoneState,

    #[fail(display = "ErrorState")]
    ErrorState,

    #[fail(display = "ParsingError: {}",_0)]
    ParsingError(String),

    #[fail(display = "{:?}", _0)]
    NomError(String),
}

impl<'a> From<nom::Err<(&'a str, nom::error::ErrorKind)>> for JSPTemplateError {
    fn from(error: nom::Err<(&'a str,nom::error::ErrorKind)> ) -> Self {
        JSPTemplateError::NomError(format!("{:?}", error))
    }
} 