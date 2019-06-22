use crate::{
    StateMachine,
    JSPTemplateLineError,
    JSPTemplateError,
};
use std::io::BufRead;

pub struct Loader {}

impl Loader {
    /// Load 
    pub fn load<R>(reader: R) -> Result<(), JSPTemplateError> 
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
