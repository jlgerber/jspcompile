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