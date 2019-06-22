use crate::{
    StateMachine,
    //JSPTemplateLineError,
    JSPTemplateError,
};
use std::{
    io::BufRead,
    collections::HashMap,
};
use jsp::{JGraph, NIndex};

pub type JGraphKeyMap = HashMap<String, NIndex>;
pub type RegexMap     = HashMap<String, String>;

pub struct Loader<'a> {
    graph: &'a mut JGraph,
}

impl<'a> Loader<'a> {
    pub fn new(graph: &'a mut JGraph) -> Self {
        Self {
            graph
        }
    }

    /// Load 
    pub fn load<R>(&self, reader: R) -> Result<(), JSPTemplateError> 
    where
        R: BufRead
    {
        let mut statemachine = StateMachine::new();
        for line in reader.lines() {
            if let Ok(line) = line {
                match statemachine.parse(&line) {
                    Ok(v) => println!("line: {} {:?}",statemachine.line_number(), v),
                    Err(e) => {
                        return Err(JSPTemplateError::from(e))
                    },
                }
            } 
        }
        Ok(())
    }

}

