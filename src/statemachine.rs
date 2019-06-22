use nom::{IResult};
use crate::{ParseResult, Header, start_parser, regex_parser, node_parser, edge_parser, JSPTemplateError, JSPTemplateLineError};
use std::cell::Cell;

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
    line: Cell<usize>,
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
            line: Cell::new(0),
            parsers: (start_parser, regex_parser, node_parser, edge_parser),
        }
    }

    pub fn line(&self) -> usize {
        self.line.get()
    }
    /// Parse teh current line of input. If the input is a Header, transition
    /// the statemachine to the next valid state.
    pub fn parse(&mut self, input: &str) -> Result<ParseResult, JSPTemplateLineError> {
        self.line.set(self.line.get() + 1);
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
                        // If we encounter a header, we transition to the state
                        // associated with the header. We only allow valid transitions as 
                        // dictated by the next_state method.
                        if let ParseResult::Header(ref header) = value {
                            let current_state = self.state.clone();

                            // get the next allowed state from the statemachine
                            let next_valid_state = match self.next_valid_state(){
                                Ok(a) => a,
                                Err(e) => return Err(JSPTemplateLineError::from((self.line.get(), e))),
                            };

                            // get the state assocated with the header
                            let new_state = match header {
                                Header::Node  =>  State::NodeParsing,
                                Header::Edge  =>  State::EdgeParsing,
                                Header::Regex =>  State::RegexParsing,
                                Header::Unknown(_) =>  State::Error,
                            };

                            // make sure that the new state matches the next valid state in the 
                            // statemachine
                            if next_valid_state != new_state {
                                return Err(
                                    JSPTemplateLineError::from(
                                        (self.line.get(),
                                        JSPTemplateError::InvalidStateTransition(current_state, new_state))
                                        )
                                    )   
                            }

                            // set the new state if the transition is a valid one to make
                            self.state = new_state;
                        }

                        return Ok(value);
                    },  
                    Err(e) => {
                        return Err(
                            JSPTemplateLineError::from(
                                ( self.line.get(), JSPTemplateError::from(e)) )
                            );
                    },
                }
            }, 
            Err(e) =>    Err(JSPTemplateLineError::from((self.line.get(), e))),
        }
    }

    // retrieve the next state in the statemachine given the current state
    fn next_valid_state(&self) -> Result<State, JSPTemplateError> {
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