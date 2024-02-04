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

pub fn get_segment_contents(key:&str, contents:  &str) -> String {
    let segment_content = get_full_segment_contents(key,contents);
    info!("segment_content: {}",segment_content);
    let start_skip = key.len() + 1;
    let content = &segment_content[start_skip..];
    content.to_string()
}

pub fn get_full_segment_contents(key:&str, contents: &str) -> String {
    let nkey = key.to_string() + "*";
    let index = contents.find(&nkey).unwrap();
    let start = &contents[index..];
    let end = start.find("~").unwrap();
    let content = &start[..end];
    content.to_string()
}


pub fn content_trim(key: &str, contents:String) -> String {
    let to_remove = get_full_segment_contents(&key, &contents)+"~";
    contents.replace(&to_remove, "").trim_start_matches("~").to_string()
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

pub fn get_loop_content(contents: String, start: &str, end: &str) -> String {
    // Note why we are doing this: in some loop the start and end are the same N1 as an example as it's the start of loop 1000a and loop 1000b
    // making it safer and skipping the start and account for the skip
    let start_found = contents.find(start).unwrap();
    let skip_start = &contents[start_found+start.len()..];
    println!("******************************************************");
    println!("{}-{}",start,end);
    println!("{:?}", skip_start);
    println!("******************************************************");
    let content;
    if end == "~~" {
        content = contents[start_found..].to_string();
    } else {
        let end_found = skip_start.find(end).unwrap();
        content = contents[start_found..end_found+start.len()].to_string();
        // let end_found = contents.find(end).unwrap();
        // content = skip_start[start_found..end_found].to_string();
        println!("{:?}", end_found);
        println!("Content: {:?}", content);
    }
    content
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

pub fn stiuational_element(value: String) -> String {
    if !value.is_empty() {
        let mut to_return = String::from("*");
        to_return.push_str(&value);
        return to_return;
    } else {
        return "".to_string();
    }
}


// pub fn stiuational_segment(value: String) -> String {
//     if !value.is_empty() {
//         let mut to_return = String::from("*");
//         to_return.push_str(&value);
//         return to_return;
//     } else {
//         return "".to_string();
//     }
// }

















#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_segment_contents() {
        let key = "N1";
        let contents = "PER*BL*JANE DOE*TE*9005555555~N1*PE*BAN DDS LLC*FI*999994703~";
        let result = get_segment_contents(key, contents);
        assert_eq!(result, "PE*BAN DDS LLC*FI*999994703");
    }

    #[test]
    fn test_get_full_segment_contents() {
        let key = "DTM";
        let contents = "SVC*AD|D1110*73*49~DTM*472*20190324~CAS*CO*131*24~AMT*B6*49~";
        let result = get_full_segment_contents(key, contents);
        assert_eq!(result, "DTM*472*20190324");
    }

    #[test]
    fn test_content_trim() {
        let key = "ST";
        let contents = "~GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~ST*835*35681~BPR*I*132*C*CHK************20190331";
        let result = content_trim(key, contents.to_string());
        assert_eq!(result, "GS*HP*SENDER CODE*RECEIVER CODE*20200101*0802*1*X*005010X221A1~BPR*I*132*C*CHK************20190331");
    }
}