use log::{info, warn};

mod edi835;
use edi835::controller::*;
mod helper;
use crate::helper::helper::{set_logger, write_to_file, process_args, get_file_contents};
mod segments;


fn main() {
    set_logger();

    let args = process_args();
    println!("{:?}", args);
    let contents = get_file_contents(args.clone());



    /*
    TODO:
        ~implement logger~
        ~check if the file passed is 835, this can be read from ~ST*835*~
        
        make it safer when something does not exist
        Check against the guide how many of each segment is in each loop, 
        finding some mismatches between the standard and the implementation of EDI835!!!

        Table 1: there are 3 PERs, 2 are optional and the required one may come in the middle

    */

    if contents.contains("~ST*835*"){
        info!("File is 835");
        let edi835 = get_835(contents.clone());
        let serialized_edi = serde_json::to_string(&edi835).unwrap();
        // println!("{}", serialized_edi);
        write_to_file(serialized_edi, args.output_file);
    } else {
        warn!("File is not 835, other types not implemeted yet");
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