## Purpose
Playground to read EDI files, starting with 835 because why not!   
I'm not new to MMIS systems but wanted to get deeper in the claims processing part, and what a better way to learn a system than building parts of it!   
First iteration is to read the the file contents and parse it's components, get famililar with the loops and segments Then phase 2 write the file itself from other sources.

I'm writing this in rust but in python style, as I'm still learning rust!   
Make it work <= we are here   
Make it right   
Make it fast   

I'm using Amazon CodeWhisperer to help me with this new endeavour if there are any code references I'll include them here.

### Artifacts and Components:
Used example file from https://x12.org/examples/005010x221 located at the src dir, will not be included in the repo (gitignored)

### Inputs
...
All harcoded for now!

### Outputs:
...

### Local testing:
to start locally:    
`cargo run <edifilepath>` to read your edi file    
`cargo run` will attempt to run a demo file
`RUST_LOG=info cargo run ./data/X221-claim-adjustment-reason-code-45.edi` example of passing in a file and setting log level   


### TODO:
everything!!!