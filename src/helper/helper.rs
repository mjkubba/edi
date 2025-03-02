use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::env;
use log::{info, warn};


#[derive(Debug, Default,Clone)]
pub struct Args {
    pub input_file: String,
    pub output_file: String,
    pub operation: String,
}


pub fn set_logger(){
    env_logger::builder()
    .filter_level(log::LevelFilter::Info)
    .format_target(false)
    .format_timestamp(None)
    .init();
}


pub fn process_args() -> Args {
    let mut args_out = Args::default();
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) || args.len() == 1 {
        println!("Usage:");
        println!();
        println!("To provide EDI file use '-f'"); 
        println!("To specify the output file use '-o'"); 
        std::process::exit(0);
    }
    if args.contains(&String::from("-f")){
        info!("-f provided");
        let f_index = args.iter().position(|r| r == "-f").unwrap();
        if args.get(f_index+1).is_some() {
            info!("{:?}", args[f_index+1]);
            args_out.input_file = args[f_index+1].clone();
        } else {
            warn!("No File provided, please pass in the file name after -f");
            std::process::exit(1);
        }
    } else {
        warn!("No File provided, please use -f to pass in the file name.");
        std::process::exit(1);
    }
    if args.contains(&String::from("-o")){
        info!("-o provided");
        let o_index = args.iter().position(|r| r == "-o").unwrap();
        if args.get(o_index+1).is_some() {
            info!("{:?}", args[o_index+1]);
            args_out.output_file = args[o_index+1].clone();
        } else {
            warn!("No File provided, please pass in the file name after -o");
            std::process::exit(1);
        }
    
    } else {
        info!("Using the default output file 'out.json'");
    }
        if args.contains(&String::from("-x")){
        info!("-x provided");
        let x_index = args.iter().position(|r| r == "-x").unwrap();
        if args.get(x_index+1).is_some() {
            info!("{:?}", args[x_index+1]);
            args_out.operation = args[x_index+1].clone();
        } else {
            warn!("No File Operation, please pass in the action after -x");
            std::process::exit(1);
        }
    
    } else {
        args_out.operation = String::from("read");
        info!("Using the default operation: Create JSON from EDI");
    }
    
    args_out
}

pub fn get_file_contents(args: Args) -> String {
    let mut contents = String::new();
    let file_path = Path::new(&args.input_file);
    
    
    if file_path.exists() {
        info!("File exists");
        let mut file = File::open(file_path).unwrap();
        file.read_to_string(&mut contents).unwrap();
    } else {
        warn!("File does not exist, please use -f to pass in the file name");
        std::process::exit(1);
    }
    contents
}

pub fn clean_contents(contents: String) -> String {
    let mut clean_contents = contents.replace("\r\n", "");
    clean_contents = clean_contents.replace("\r", "");
    clean_contents = clean_contents.replace("\n", "");
    clean_contents = clean_contents.replace("~ ", "~");
    clean_contents
}


pub fn write_to_file(write_contents: String, write_file: String) {
    // setting up write file functionality
    let write_file_path;
    if write_file != "" {
        write_file_path = Path::new(&write_file);
    } else {
        info!("No File provided, Writing to default file out.json");
        write_file_path = Path::new("./out.json");
    }
    // write_file_path = Path::new("./out.json");
    let mut write_file = File::create(write_file_path).unwrap();
    write_file.write_all(write_contents.as_bytes()).unwrap();
}
