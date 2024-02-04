use log::{info, warn};

mod edi835;
use edi835::controller::*;
mod helper;
use crate::helper::helper::{set_logger, write_to_file, process_args, get_file_contents};
mod segments;


fn main() {
    set_logger();

    let args = process_args();
    let contents = get_file_contents(args.clone());


    /*
    TODO:
        ~implement logger~
        ~check if the file passed is 835, this can be read from ~ST*835*~
        
        make it safer when something does not exist
        Check against the guide how many of each segment is in each loop, 
        finding some mismatches between the standard and the implementation of EDI835!!!

        Table 1: there are 3 PERs, 2 are optional and the required one may come in the middle

        also noticed DTM is getting processed earlier, need to trim content based on loop segments and only send that part for processing.
        Created a function: get_loop_content in helper takes the content and start and finish of the loop, then only provide that to the get_xyz
        need to change the trimming and other thing already there, look at table1 as an example

        The method I was going with is also unsafe, if you look at table 2, few of these loops and segements are all situational!
        Need to rethink this
        Maybe have a table of all the loops and segments and send the function the part we are looking for, eg table2-loop2000 and the 
        function will return the content of that loop based on the table lookup.


    */

    if args.operation == "write" {
        info!("Write EDI Operation");
        let new_edi = write_edi(contents.clone());
        write_to_file(new_edi, args.output_file);
    } else if args.operation == "read" {
        if contents.contains("~ST*835*"){
            info!("File is 835");
            let edi835 = get_835(contents.clone());
            let serialized_edi = serde_json::to_string(&edi835).unwrap();
            // write_to_file(serialized_edi, args.output_file);
            write_to_file(serialized_edi.clone(), args.output_file);
        } else {
            warn!("File is not 835, other types not implemeted yet");
        }
    }

    
    

}


// unit test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}