# Parse the jsptemplate format
```
[regex]
num_under =   "[0-9_]+"
quicktimes =  "quicktimes"
qtsubdir   =  "[0-9_]+" 
doc_sd     =  "(agency|director_treatments|vfx_methodology|schedules|scripts|storyboards)"
chars_sd   =  "(DEVL|SHARED|etc|lib|bin|user)"
show       = "[A-Z]+[A-Z0-9]*" "(REF|SHARED|OUTSOURCE|LOCATIONS)"

[nodes]
dd  
shows
show            = $show [ owner: jobsys, perms: 751, varname: DD_SHOW ]      
refdir          = REF [ volume ]
quickimes       = $quicktimes [ perms: 751 ]
qtsubdir        = $qtsubdir
clientvault     = CLIENT_VAULT [ volume ]
clientvault_sd  = "(incoming|outgoing)"
clientvault_ssd = "[0-9_]+"

[graph] 
dd -> refdir -> quicktimes
dd -> shows -> show 
```

# Design Notes

I intend on modeling a state machine

```
parser(buffer)
StartState.parse() -> Result<RegexHeaderState, ErrorState>

RegexHeaderState -> Result<RegexState, ErrorState>

RegexState -> Result<NodeHeaderState,  ErrorState>

NodeHeaderState -> Result<NodeState, ErrorState>
NodeState -> Result<GraphHeaderState, ErrorState>
GraphStateNeaderState -> Result<DoneState,ErrorState>


let mut state : State = StartState::new();
let mut result = Vec::new();

enum Event {
    Next(State),
    Continue,
    Break(Error),
}
for line in lines {
    let mut event = Event::Next;
    match event {
        Event::Continue {
            match state.process(line, &mut result) {
                Err(e) => {pritnln!("error:{}", e.to_string()); event = Event::Break(e.clone());},
                Ok(next_state) => {
                    if next_state == state {
                        event = Event::Continue;
                        next_state = state;
                    } else {
                        event = Next(next_state);
                    }
                }
            }
        },
        Event::Next(state) => {
            match state.process(line, &mut result) {}
        }
    }
}
```

