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
        // parse current line if the statemachine is in a state that has a parser
        // associated with it. If the state doesnt have an associated parser, set
        // the appropriate error.
        let parsed_line = match self.state {
            State::Start        => Ok(self.parsers.0(input)),
            State::RegexParsing => Ok(self.parsers.1(input)),
            State::NodeParsing  => Ok(self.parsers.2(input)),
            State::EdgeParsing  => Ok(self.parsers.3(input)),
            State::Done  => Err(JSPTemplateError::DoneState),
            State::Error => Err(JSPTemplateError::ErrorState),
        };
        // outer result determines whether the statemachine is in a state 
        // that can parse lines. Neither the Done state nor the Error state 
        // qualify as such.
        match parsed_line {
            Ok(result) => {
                // inner Result determines whether parsing of line is ok
                match result {
                    Ok((_, value)) => {
                        if let ParseResult::Header(ref new_state) = value {
                            let current_state = self.state.clone();
                            // get the expected next state from the
                            let next_state = self.next_state()?;
                            let new_state = match new_state {
                                Header::Node  =>  State::NodeParsing,
                                Header::Edge  =>  State::EdgeParsing,
                                Header::Regex =>  State::RegexParsing,
                                Header::Unknown(_) =>  State::Error,
                            };

                            if next_state != new_state {
                                return Err(JSPTemplateError::InvalidStateTransition(current_state, new_state))   
                            }
                            // set the new state if the transition is a valid one to make
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

    // retrieve the next state in the statemachine given the current state
    fn next_state(&self) -> Result<State, JSPTemplateError> {
        match self.state {
            State::Start        => Ok(State::RegexParsing),
            State::RegexParsing => Ok(State::NodeParsing),
            State::NodeParsing  => Ok(State::EdgeParsing),
            State::EdgeParsing  => Ok(State::Done),
            State::Done         => Err(JSPTemplateError::NoValidNextState(State::Done)),
            State::Error        => Err(JSPTemplateError::NoValidNextState(State::Error))
        }
    }
}