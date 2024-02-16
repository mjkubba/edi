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
To provide EDI file use `-f` then the file name.   
To specify the output file use `-o` then output the file name.     
`cargo run -f <edifilepath> -o <outputfile>` or the compiled version `./edi -f <edifilepath> -o <outputfile>` for *nix and `.\edi.exe -f <edifilepath> -o <outputfile>` for Windows.   
If no file path provided the demo file will be used as input.

### Outputs:   
If file path is provided in the 2nd place after the file name it will be used to dump the json,     
otherwise json output will be written in `out.json` file
`cargo run -f <edifilepath> -o <outputfile>`

### Local testing:
to start locally:    
`cargo run <edifilepath>` to read your edi file    
`cargo run` will attempt to run a demo file   
`RUST_LOG=info cargo run -f ./data/X221-claim-adjustment-reason-code-45.edi` example of passing in a file and setting log level   


### TODO:
* ~~implement logger~~
* ~~check if the file passed is 835, this can be read from ST*835*~~
* ~~Check against the guide how many of each segment is in each loop~~
* ~~Table 1: there are 3 PERs, 2 are optional and the required one may come in the middle~~
* ~~Adding parameterized input, -f for file -o for output etc.~~
* ~~Adding Write EDI 835 functionality~~
* ~~Finding some mismatches between the standard and the implementation of EDI835!!!~~
* Make it safer when something does not exist
* More cool things