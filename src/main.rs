use std::io::prelude::*;
use std::io::{self, SeekFrom};
use std::io::Cursor;

const foo : &'static str = r#" 
[regex]
num_under =   "[0-9_]+"
quicktimes =  "quicktimes"
qtsubdir   =  "[0-9_]+" 
doc_sd     =  "(agency|director_treatments|vfx_methodology|schedules|scripts|storyboards)"
chars_sd   =  "(DEVL|SHARED|etc|lib|bin|user)"
level       = "[A-Z]+[A-Z0-9]*" "(REF|SHARED|OUTSOURCE|LOCATIONS)"

[nodes]
dd  
shows
show            = $level   
seq             = $level 
shot            = $level 
refdir          = REF 
shared          = SHARED
img             = IMG
quickimes       = $quicktimes 
qtsubdir        = $qtsubdir
clientvault     = CLIENT_VAULT
clientvault_sd  = "(incoming|outgoing)"
clientvault_ssd = "[0-9_]+"

[graph] 
dd -> refdir -> quicktimes
dd -> shows -> show -> sequence -> shot
# speculative shared -> img | model | anim | fx 
show -> shared
seq -> shared
shot -> shared
"#;

fn main() {
    println!("Hello, world!");
}
