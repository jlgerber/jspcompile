use structopt::StructOpt;
use std::fs::File;
use std::io::{BufRead, BufReader, self};
use std::path::PathBuf;
use jsptemplate::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "jspcompile", about = "compile a jsptemplate from a jsptemplate. hu?")]
struct Opt {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    debug: bool,

    /// Input file
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
                JSPTemplateError::ErrorAtLine(line, error) => {
                    println!("");
                    println!("Error at line: {} - {}",line, error.to_string());
                    println!("")
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
    Loader::load(bufreader)?;
    Ok(())
}


