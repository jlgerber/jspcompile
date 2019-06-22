use failure::Fail;
use crate::State;

#[derive(Debug, Fail)]
pub enum JSPTemplateError {
    #[fail(display = "Invalid State Transition from: {:?} to {:?}", _0, _1)]
    InvalidStateTransition(State, State),

    #[fail(display = "Placeholder")]
    Placeholder,

    #[fail(display = "DoneState")]
    DoneState,

    #[fail(display = "ErrorState")]
    ErrorState,

    #[fail(display = "ParsingError: {}",_0)]
    ParsingError(String),
}