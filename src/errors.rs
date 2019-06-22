use failure::Fail;
use crate::State;
use nom;
use std::{io};
use ext_regex;

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

    #[fail(display = "NomError: {:?}", _0)]
    NomError(String),

    #[fail(display = "ErrorAtLine: {}, Line: {}, State: {}, Error: {:?}", _0, _1, _2, _3)]
    ErrorAtLine(usize, String, State, Box<JSPTemplateError>),

    #[fail(display = "{}", _0)]
    IoError(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    RegexError(#[cause] ext_regex::Error),

    #[fail(display = "Regex Map Lookup failed for: {}", _0)]
    RegexMapLookupError(String),

    #[fail(display = "key Map Lookup failed for: {}", _0)]
    KeyMapLookupError(String),
}

impl<'a> From<nom::Err<(&'a str, nom::error::ErrorKind)>> for JSPTemplateError {
    fn from(error: nom::Err<(&'a str,nom::error::ErrorKind)> ) -> Self {
        JSPTemplateError::NomError(format!("{:?}", error))
    }
} 

impl From<io::Error> for JSPTemplateError {
    fn from(error: io::Error) -> Self {
        JSPTemplateError::IoError(error)
    }
}

impl From<JSPTemplateLineError> for JSPTemplateError {
    fn from(error: JSPTemplateLineError) -> Self {
        let JSPTemplateLineError::ErrorAtLine(line_num, line, state, err) = error;
        JSPTemplateError::ErrorAtLine(line_num, line, state, Box::new(err))
    }
}

//std::convert::From<regex::error::Error>
impl From<ext_regex::Error> for JSPTemplateError {
    fn from(error: ext_regex::Error) -> Self {
        JSPTemplateError::RegexError(error)
    }
}


/// Wrap JSPTemplateError to provide a line number associated with each error
#[derive(Debug, Fail)]
pub enum JSPTemplateLineError {
    #[fail(display = "Error at line: {} line: {} State: {} Error: {:?}", _0, _1, _2, _3)]
    ErrorAtLine(usize, String, State, JSPTemplateError)
}

/// Convert from a JSPTemplateError to a JSPTemplateLineError by 
/// providing a tuple of ( line number, error ).
impl From<(usize, String, State, JSPTemplateError)> for JSPTemplateLineError {
    fn from(error: (usize, String, State, JSPTemplateError) ) -> Self {
        JSPTemplateLineError::ErrorAtLine(error.0, error.1, error.2, error.3)
    }
} 

