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


fn main() -> io::Result<()> {
    let opt = Opt::from_args();
    
    let file = File::open(opt.input)?;
    let mut cnt = 0;
    let mut statemachine = StateMachine::new();
    for line in BufReader::new(file).lines() {
        if let Ok(line) = line {
            cnt += 1;
            match statemachine.parse(&line) {
                Ok(v) => println!("line: {} {:?}",cnt, v),
                Err(e) => {
                    println!("Error Parsing Line {}: {:?}",cnt, e);
                    std::process::exit(1);
                },
            }
        } 
    }

    Ok(())
}


