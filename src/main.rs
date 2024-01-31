use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::env;
use log::{info, warn};

mod edi835;
use edi835::controller::*;
mod helper;
mod segments;

#[derive(Debug, Default)]
struct Args {
    input_file: String,
    output_file: String,
}

fn set_logger(){
    env_logger::builder()
    .filter_level(log::LevelFilter::Warn)
    .format_target(false)
    .format_timestamp(None)
    .init();
}

fn write_to_file(write_contents: String, write_file: String) {
    // setting up write file functionality
    let write_file_path;
    if write_file != "" {
        write_file_path = Path::new(&write_file);
    } else {
        warn!("No File provided, Writing to default file out.json");
        write_file_path = Path::new("./out.json");
    }
    // write_file_path = Path::new("./out.json");
    let mut write_file = File::create(write_file_path).unwrap();
    write_file.write_all(write_contents.as_bytes()).unwrap();
}

fn process_args() -> Args {
    let mut args_out = Args::default();
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("-f")){
        info!("-f provided");
        let f_index = args.iter().position(|r| r == "-f").unwrap();
        info!("{:?}", args[f_index+1]);
        args_out.input_file = args[f_index+1].clone();
    }
    if args.contains(&String::from("-o")){
        info!("-o provided");
        let o_index = args.iter().position(|r| r == "-o").unwrap();
        info!("{:?}", args[o_index+1]);
        args_out.output_file = args[o_index+1].clone();
    }
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        info!("--help provided");
        println!("To provide EDI file use '-f'"); 
        println!("To specify the output file use '-o'"); 
        std::process::exit(0);
    }
    args_out
}

fn main() {
    let args = process_args();
    println!("{:?}", args);
    // env_logger::init();
    set_logger();
    let mut file_path;
    // Open File and read content
    if args.input_file != "" {
        file_path = Path::new(&args.input_file);
    } else {
        warn!("No File provided, Loading default demo file edi835-1.edi");
        file_path = Path::new("./demo/edi835-1.edi");
    }

    if file_path.exists() {
        info!("File exists");
    } else {
        warn!("File does not exist, Loading default demo file edi835-1.edi");
        file_path = Path::new("./demo/edi835-1.edi");
    }
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();







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