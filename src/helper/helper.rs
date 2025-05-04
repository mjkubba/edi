/**
 * Helper module for EDI processing
 * 
 * This module provides utility functions for handling EDI files, including:
 * - Command line argument processing
 * - File operations (reading and writing)
 * - Content cleaning and formatting
 * - Logging setup
 */
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use log::{info, warn};

/**
 * Command line arguments structure
 * 
 * Holds the parsed command line arguments for the EDI processor
 */
#[derive(Debug, Default,Clone)]
pub struct Args {
    pub file_path: String,
    pub output_file: String,
    pub operation: String,
    pub is_json: bool,
}

/**
 * Set up the logger for the application
 * 
 * Configures the logger to display info level messages without timestamps or targets
 */
pub fn set_logger(){
    env_logger::builder()
    .filter_level(log::LevelFilter::Info)
    .format_target(false)
    .format_timestamp(None)
    .init();
}

/**
 * Process command line arguments
 * 
 * Parses the command line arguments and returns an Args structure
 * 
 * Arguments:
 * - -f: Input file path
 * - -o: Output file path
 * - -w: Write mode (convert JSON to EDI)
 * - -j: Input is JSON
 * - -h/--help: Show help information
 * 
 * Returns:
 * - Args structure with parsed arguments
 */
pub fn process_args() -> Args {
    let mut args = Args::default();
    let mut args_iter = std::env::args().skip(1);
    let mut operation = String::from("read");
    
    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "-f" => {
                info!("-f provided");
                if let Some(file_path) = args_iter.next() {
                    info!("{:?}", file_path);
                    args.file_path = file_path;
                } else {
                    warn!("No file provided after -f");
                    std::process::exit(1);
                }
            }
            "-o" => {
                info!("-o provided");
                if let Some(output_file) = args_iter.next() {
                    info!("{:?}", output_file);
                    args.output_file = output_file;
                } else {
                    warn!("No file provided after -o");
                    std::process::exit(1);
                }
            }
            "-w" => {
                info!("-w provided");
                operation = String::from("write");
            }
            "-j" => {
                info!("-j provided");
                args.is_json = true;
            }
            "-h" | "--help" => {
                println!("Usage:");
                println!();
                println!("To provide EDI file use '-f'"); 
                println!("To specify the output file use '-o'");
                println!("To write EDI from JSON use '-w'");
                println!("To specify input is JSON use '-j'");
                std::process::exit(0);
            }
            _ => {}
        }
    }
    
    args.operation = operation;
    
    if args.operation == "write" {
        info!("Using operation: Write EDI from JSON");
    } else {
        info!("Using operation: Create JSON from EDI");
    }
    
    if args.file_path.is_empty() {
        warn!("No file provided, please use -f to pass in the file name");
        std::process::exit(1);
    }
    
    if args.output_file.is_empty() {
        if args.operation == "read" {
            args.output_file = String::from("out.json");
        } else {
            args.output_file = String::from("out.edi");
        }
        info!("Using default output file: {}", args.output_file);
    }
    
    args
}

/**
 * Read file contents
 * 
 * Reads the contents of a file specified in the Args structure
 * 
 * Parameters:
 * - args: Args structure with file_path
 * 
 * Returns:
 * - String containing the file contents
 */
pub fn get_file_contents(args: Args) -> String {
    let mut contents = String::new();
    let file_path = Path::new(&args.file_path);
    
    if file_path.exists() {
        info!("File exists");
        let mut file = File::open(file_path).unwrap();
        file.read_to_string(&mut contents).unwrap();
    } else {
        warn!("File does not exist: {}", args.file_path);
        std::process::exit(1);
    }
    contents
}

/**
 * Clean file contents
 * 
 * Removes line breaks and normalizes delimiters in EDI content
 * 
 * Parameters:
 * - contents: String to clean
 * 
 * Returns:
 * - Cleaned string
 */
pub fn clean_contents(contents: String) -> String {
    let mut clean_contents = contents.replace("\r\n", "");
    clean_contents = clean_contents.replace("\r", "");
    clean_contents = clean_contents.replace("\n", "");
    clean_contents = clean_contents.replace("~ ", "~");
    clean_contents
}

/**
 * Write content to file
 * 
 * Writes the provided content to the specified file
 * 
 * Parameters:
 * - write_contents: Content to write
 * - write_file: Path to the output file
 */
pub fn write_to_file(write_contents: String, write_file: String) {
    let write_file_path = if write_file.is_empty() {
        info!("No output file specified, writing to default file");
        Path::new("./out.json")
    } else {
        Path::new(&write_file)
    };
    
    match File::create(write_file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(write_contents.as_bytes()) {
                warn!("Failed to write to file: {}", e);
                std::process::exit(1);
            }
            info!("Successfully wrote to file: {:?}", write_file_path);
        },
        Err(e) => {
            warn!("Failed to create file: {}", e);
            std::process::exit(1);
        }
    }
}
