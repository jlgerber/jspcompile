use nom::{IResult};
use crate::{ParseResult, Header, start_parser, regex_parser, node_parser, edge_parser, JSPTemplateError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum State {
    Start,
    RegexParsing,
    NodeParsing,
    EdgeParsing,
    Done,
    Error
}

impl State {

    // retrieve the next state 
    fn next(&self) -> Result<State,JSPTemplateError> {
        match self {
            State::Start => Ok(State::RegexParsing),
            State::RegexParsing => Ok(State::NodeParsing),
            State::NodeParsing => Ok(State::EdgeParsing),
            State::EdgeParsing => Ok(State::Done),
            State::Done => Err(JSPTemplateError::NoValidNextState(State::Done)),
            State::Error => Err(JSPTemplateError::NoValidNextState(State::Error))
        }
    }
}

pub struct StateMachine {
    state: State,
    parsers: (
        fn(&str)->IResult<&str, ParseResult>,
        fn(&str)->IResult<&str, ParseResult>,
        fn(&str)->IResult<&str, ParseResult>,
        fn(&str)->IResult<&str, ParseResult>  
    )
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            state: State::Start,
            parsers: (start_parser, regex_parser, node_parser, edge_parser),
        }
    }

    /// Parse teh current line of input. If the input is a Header, transition
    /// the statemachine to the next valid state.
    pub fn parse(&mut self, input: &str) -> Result<ParseResult, JSPTemplateError> {

        let cstate = match self.state {
            State::Start        => Ok(self.parsers.0(input)),
            State::RegexParsing => Ok(self.parsers.1(input)),
            State::NodeParsing  => Ok(self.parsers.2(input)),
            State::EdgeParsing  => Ok(self.parsers.3(input)),
            State::Done  => Err(JSPTemplateError::DoneState),
            State::Error => Err(JSPTemplateError::ErrorState),
        };

        match cstate {
            Ok(result) => {
                // grabbing from IResult
                //if let Ok((_, value)) = result {
                match result {
                    Ok((_, value)) => {
                        if let ParseResult::Header(ref new_state) = value {
                            let current_state = self.state.clone();
                            let next_state = self.state.next()?;
                            let new_state = match new_state {
                                Header::Node  =>  State::NodeParsing,
                                Header::Edge  =>  State::EdgeParsing,
                                Header::Regex =>  State::RegexParsing,
                                Header::Unknown(_) =>  State::Error,
                            };

                            if next_state != new_state {
                                return Err(JSPTemplateError::InvalidStateTransition(current_state, new_state))   
                            }

                            self.state = new_state;
                        }

                        return Ok(value);
                    },  
                    Err(e) => {
                        return Err(JSPTemplateError::from(e));
                    },
                }
            }, 
            Err(e) =>    Err(e),
        }
    }
}