use structopt::StructOpt;
use std::fs::File;
use std::io::{BufReader};
use std::{
    path::PathBuf,
};

use jsptemplate::{JSPTemplateError, Loader, State, RegexMap, JGraphKeyMap};
use colored::*;
use jsp::{JGraph, NIndex};

#[derive(Debug, StructOpt)]
#[structopt(name = "jspcompile", about = "Compile a jsptemplate from a jspt file")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Input jspt file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
    
    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    output: Option<PathBuf>,
}


fn main() {
    match doit(){
        Ok(_) => (),
        Err(e) => {
            match e {
                JSPTemplateError::ErrorAtLine(line_num, line, state, error) => {
                    display_formatted_error(line_num, &line, &state, error);
                },
                
                _ => println!("{}", e.to_string()),
            }
            
            std::process::exit(1);
        }
    }
}


fn doit() -> Result<(), JSPTemplateError> {
    let opt = Opt::from_args();
    
    let file = File::open(opt.input)?;
    let bufreader =  BufReader::new(file);

    // lets create stuff
    let mut graph = JGraph::new();
    let mut keymap = JGraphKeyMap::new();
    let mut regexmap = RegexMap::new();

    let loader = Loader::new(&mut graph, &mut keymap, &mut regexmap);
    loader.load(bufreader)?;
    Ok(())
}


#[inline]
fn display_formatted_error(
    line_num: usize, 
    line: &str, 
    state: &State, 
    error: Box<JSPTemplateError>
) {
    println!("");
    let title = "Error Parsing File".red().bold();
    let error_title = "Error".bright_red();
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
