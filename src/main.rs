use structopt::StructOpt;
use std::fs::File;
use std::io::{BufReader};
use std::{
    path::PathBuf,
};
use log::{ LevelFilter, self };
use chrono;
use colored::Colorize;
use fern::{ colors::{Color, ColoredLevelConfig}, self} ;
use jsptemplate::{JSPTemplateError, Loader, State, RegexMap, JGraphKeyMap};
use jsp::{JGraph, NIndex, diskutils};

#[derive(Debug, StructOpt)]
#[structopt(name = "jspcompile", about = "Compile a jsptemplate from a jspt file")]
struct Opt {
    /// Set logging level to one of trace, debug, info, warn, error
    #[structopt( short = "l", long = "level", default_value = "warn" )]
    level: String,

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
    //let opt = Opt::from_args();
    let (mut opt, level) = setup_cli();
    setup_logger(level).unwrap();
    
    let file = File::open(opt.input)?;
    let bufreader =  BufReader::new(file);

    // lets create stuff
    let mut graph = JGraph::new();
    let mut keymap = JGraphKeyMap::new();
    let mut regexmap = RegexMap::new();

    let mut loader = Loader::new(&mut graph, &mut keymap, &mut regexmap);
    loader.load(bufreader)?;
    if let Some(ref mut output) = opt.output {
        diskutils::write_template(output, &graph);
    }
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


#[inline]
fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    let  colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Cyan)
        .trace(Color::BrightCyan);;

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[inline]
fn setup_cli() -> (Opt, log::LevelFilter) {
    let args = Opt::from_args();
    let level = match args.level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info"  => LevelFilter::Info,
        "warn"  | "warning" => LevelFilter::Warn,
        "err" | "error" => LevelFilter::Error,
        _ => LevelFilter::Warn,
    };

    (args, level)
}