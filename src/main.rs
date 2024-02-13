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
        refactor helper to keep generic thingies there and move specific 835 functions to another file or the controller.        
        Where I left:
        EDI835 is done, need to make sure if a supplied file is messed up or some weird input provided the program exit gracefully
        also double check all the failure points
        build unit test back again
        implement integration tests


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


// // unit test
// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn test_main() {
//         main();
//     }
// }