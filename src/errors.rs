use failure::Fail;
use crate::State;
use nom;
use std::{io};

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

    #[fail(display = "ErrorAtLine: {}, Line: {}, Error: {:?}", _0, _1, _2)]
    ErrorAtLine(usize, String, Box<JSPTemplateError>),

    #[fail(display = "{}", _0)]
    IoError(#[cause] io::Error),
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
        let JSPTemplateLineError::ErrorAtLine(line_num, line, err) = error;
        JSPTemplateError::ErrorAtLine(line_num, line, Box::new(err))
    }
}

/// Wrap JSPTemplateError to provide a line number associated with each error
#[derive(Debug, Fail)]
pub enum JSPTemplateLineError {
    #[fail(display = "Error at line: {} line: {} Error: {:?}", _0, _1, _2)]
    ErrorAtLine(usize, String, JSPTemplateError)
}

/// Convert from a JSPTemplateError to a JSPTemplateLineError by 
/// providing a tuple of ( line number, error ).
impl From<(usize, String, JSPTemplateError)> for JSPTemplateLineError {
    fn from(error: (usize, String,  JSPTemplateError) ) -> Self {
        JSPTemplateLineError::ErrorAtLine(error.0, error.1, error.2)
    }
} 