
//! Currently unused. This struct models an output device for communicating 
//! with end users via the terminal. Its aim is to provide user readable 
//! communication that is clearly targeted, unlike logging, which generally
//! is for developers and tds, and is often dismissed by end users due to 
//! its noisy appearance.
//!  
//! Jspcompile currently handles this via loose functions. 
use colored::*;

pub struct Console {
    error_color: String,
}

impl Console {

    pub fn new<S>(error_color: S) -> Console where S: Into<String> {
        Console {
            error_color: error_color.into(),
        }
    }

    pub fn error<E>(&self, error:E) where E:ToString {
        println!("");
        println!("{}", "Error".color(self.error_color.as_str()).bold());
        println!("\n\t{}", error.to_string());
        println!("");
    }

    pub fn error_at<E>(&self, line_num: usize, line: &str, state: &State, error: E ) where E: ToString {
        println!("");
        let error_title = "Error".color(self.error_color.as_str()).bold();
        let title = "Error Parsing File".color(self.error_color.as_str()).bold();
        let line_num_title = "LineNo".bright_red();
        let line_title = "Line".bright_red();
        let state_title = "State".bright_red();
        println!("{}\n\n\t{} {}\n\t{}   {}\n\t{}  {}\n\t{}  {}", 
            title,
            line_num_title,
            line_num.to_string(),
            line_title, 
            line,
            state_title,
            state, 
            error_title,        
            error.to_string());

        println!("")  
    }
}
