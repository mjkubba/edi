use log::{info, warn};

mod edi835;
use edi835::controller::*;
mod edi999;
use edi999::controller::*;
mod helper;
use crate::helper::helper::{set_logger, write_to_file, process_args, get_file_contents, clean_contents};
mod segments;
mod error;
mod transaction_processor;
mod segment_config;
mod loop_processor;


fn main() {
    set_logger();

    let args = process_args();
    let contents = get_file_contents(args.clone());
    let contents = clean_contents(contents.clone());

    /*
    TODO:
        refactor helper to keep generic thingies there and move specific 835 functions to another file or the controller.        
        Where I left:
        EDI835 is done, need to make sure if a supplied file is messed up or some weird input provided the program exit gracefully
        also double check all the failure points
        build unit test back again
        implement integration tests

        999 have segment loops, similar to 835, need to write the logic for these.
    */

    if args.operation == "write" {
        info!("Write EDI Operation");
        
        // Check if the content is JSON for 835 format
        if contents.contains("\"transaction_set_id\":\"835\"") || contents.contains("\"bpr_segments\":") {
            info!("Writing 835 format");
            let edi_json: Edi835 = serde_json::from_str(&contents.clone()).unwrap();
            let new_edi = write_edi(contents.clone());
            write_to_file(new_edi, args.output_file);
        } 
        // Check if the content is JSON for 999 format
        else if contents.contains("\"transaction_set_id\":\"999\"") || contents.contains("\"ak1_segments\":") {
            info!("Writing 999 format");
            let edi_json: Edi999 = serde_json::from_str(&contents.clone()).unwrap();
            let new_edi = write_999(&edi_json);
            write_to_file(new_edi, args.output_file);
        }
        // Check if the content is raw EDI for 835 format
        else if contents.contains("CLP") || contents.contains("clp") {
            info!("Writing 835 format from raw EDI");
            let new_edi = write_edi(contents.clone());
            write_to_file(new_edi, args.output_file);
        } 
        // Check if the content is raw EDI for 999 format
        else if contents.contains("AK2") || contents.contains("ak2") {
            info!("Writing 999 format from raw EDI");
            let edi999 = get_999(contents.clone());
            let new_edi = write_999(&edi999.0);
            write_to_file(new_edi, args.output_file);
        }
        else {
            warn!("Unknown format for writing");
        }
    } else if args.operation == "read" {
        info!("Read EDI Operation");
        
        if contents.contains("~ST*835*") || contents.contains("ST*835*") {
            info!("File is 835");
            let edi835 = get_835(contents.clone());
            let serialized_edi = serde_json::to_string(&edi835).unwrap();
            write_to_file(serialized_edi.clone(), args.output_file);
        } else if contents.contains("~ST*999*") || contents.contains("ST*999*") {
            info!("File is 999");
            let edi999 = get_999(contents.clone());
            let serialized_edi = serde_json::to_string(&edi999.0).unwrap();
            write_to_file(serialized_edi.clone(), args.output_file);
        } else {
            warn!("File format not recognized. Currently supporting 835 and 999 formats.");
        }
    }
}
