use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::env;
use log::{info, warn};

mod edi835;
use edi835::controller::*;
mod helper;
mod segments;


fn main() {
    env_logger::init();
    let mut file_path;
    // Open File and read content
    let args: Vec<String> = env::args().collect();
    if args.get(1).is_some() {
        file_path = Path::new(&args[1]);
    } else {
        file_path = Path::new("./demo/edi835-1.edi");
    }

    if file_path.exists() {
        info!("File exists");
    } else {
        warn!("File does not exist");
        info!("Loading default demo file edi835-1.edi");
        file_path = Path::new("./demo/edi835-1.edi");
    }
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();


    /*
    TODO:
        ~implement logger~
        make it safer when something does not exist

        Check against the guide how many of each segment is in each loop, 
        finding some mismatches between the standard and the implementation of EDI835!!!

        Table 1: there are 3 PERs, 2 are optional and the required one may come in the middle

        check if the file passed is 835, this can be read from ~ST*835*
    */

    if contents.contains("~ST*835*"){
        info!("File is 835");
        let _edi835 = get_835(contents.clone());
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