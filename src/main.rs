/**
 * EDI Parser and Processor for Healthcare X12 Formats
 * 
 * This application provides functionality to parse and generate EDI files
 * for healthcare X12 formats including 835, 999, 270/271, 276/277, and 837.
 * 
 * The main module handles command line arguments and routes processing
 * to the appropriate controller based on the EDI format.
 */
use log::{info, warn};

use crate::helper::helper::*;
use crate::edi835::controller::*;
use crate::edi999::controller::*;
use crate::edi270::controller::*;
use crate::edi271::controller::*;
use crate::edi276::controller::*;
use crate::edi277::controller::*;
use crate::edi837::controller::*;

mod helper;
mod segments;
mod edi835;
mod edi999;
mod edi270;
mod edi271;
mod edi276;
mod edi277;
mod edi837;
mod error;
mod transaction_processor;
mod segment_config;
mod loop_processor;

/**
 * Main entry point for the EDI Parser application
 * 
 * Processes command line arguments and routes to the appropriate
 * controller based on the EDI format and operation.
 */
fn main() {
    set_logger();
    info!("Starting EDI Parser");
    
    let args = process_args();
    let contents = get_file_contents(args.clone());
    let _clean_contents = clean_contents(contents.clone());
    
    if args.operation == "write" {
        info!("Write EDI Operation");
        
        if args.is_json {
            info!("Input is JSON");
            
            // Check if the content is JSON for 835 format
            if contents.contains("\"transaction_set_id\":\"835\"") {
                info!("Writing 835 format");
                let _edi835: Edi835 = serde_json::from_str(&contents).unwrap();
                let new_edi = write_835(contents.clone());
                write_to_file(new_edi, args.output_file);
            }
            // Check if the content is JSON for 999 format
            else if contents.contains("\"transaction_set_id\":\"999\"") {
                info!("Writing 999 format");
                let edi999: Edi999 = serde_json::from_str(&contents).unwrap();
                let new_edi = write_999(&edi999);
                write_to_file(new_edi, args.output_file);
            }
            // Check if the content is JSON for 270 format
            else if contents.contains("\"transaction_set_id\":\"270\"") {
                info!("Writing 270 format");
                let edi270: Edi270 = serde_json::from_str(&contents).unwrap();
                let new_edi = write_270(&edi270);
                write_to_file(new_edi, args.output_file);
            }
            // Check if the content is JSON for 271 format
            else if contents.contains("\"transaction_set_id\":\"271\"") {
                info!("Writing 271 format");
                let edi271: Edi271 = serde_json::from_str(&contents).unwrap();
                let new_edi = write_271(&edi271);
                write_to_file(new_edi, args.output_file);
            }
            // Check if the content is JSON for 276 format
            else if contents.contains("\"st01_transaction_set_identifier_code\":\"276\"") {
                info!("Writing 276 format");
                let edi276: Edi276 = serde_json::from_str(&contents).unwrap();
                let new_edi = write_276(&edi276);
                write_to_file(new_edi, args.output_file);
            }
            // Check if the content is JSON for 277 format
            else if contents.contains("\"st01_transaction_set_identifier_code\":\"277\"") {
                info!("Writing 277 format");
                let edi277: Edi277 = serde_json::from_str(&contents).unwrap();
                let new_edi = write_277(&edi277);
                write_to_file(new_edi, args.output_file);
            }
            // Check if the content is JSON for 837P format
            else if contents.contains("\"transaction_set_id\":\"837P\"") {
                info!("Writing 837P format");
                let edi837p: Edi837P = serde_json::from_str(&contents).unwrap();
                match write_837p(&edi837p) {
                    Ok(new_edi) => write_to_file(new_edi, args.output_file),
                    Err(e) => warn!("Error writing 837P format: {:?}", e)
                }
            }
            // Check if the content is JSON for 837I format
            else if contents.contains("\"transaction_set_id\":\"837I\"") {
                info!("Writing 837I format");
                let edi837i: Edi837I = serde_json::from_str(&contents).unwrap();
                match write_837i(&edi837i) {
                    Ok(new_edi) => write_to_file(new_edi, args.output_file),
                    Err(e) => warn!("Error writing 837I format: {:?}", e)
                }
            }
            // Check if the content is JSON for 837P format
            else if contents.contains("\"transaction_set_id\":\"837P\"") {
                info!("Writing 837P format");
                let edi837p: Edi837P = serde_json::from_str(&contents).unwrap();
                match write_837p(&edi837p) {
                    Ok(new_edi) => write_to_file(new_edi, args.output_file),
                    Err(e) => warn!("Error writing 837P format: {:?}", e)
                }
            }
            else {
                warn!("Unknown format for writing");
            }
        } else {
            info!("Input is raw EDI");
            
            // Check if the content is raw EDI for 835 format
            if contents.contains("ST*835*") {
                info!("Writing 835 format from raw EDI");
                let edi835 = get_835(contents.clone());
                let serialized_edi = serde_json::to_string(&edi835).unwrap();
                let new_edi = write_835(serialized_edi);
                write_to_file(new_edi, args.output_file);
            }
            // Check if the content is raw EDI for 999 format
            else if contents.contains("ST*999*") {
                info!("Writing 999 format from raw EDI");
                let edi999 = get_999(contents.clone());
                let new_edi = write_999(&edi999.0);
                write_to_file(new_edi, args.output_file);
            }
            // Check if the content is raw EDI for 270 format
            else if contents.contains("BHT*0022*13") || (contents.contains("HL*") && contents.contains("*20*")) {
                info!("Writing 270 format from raw EDI");
                match get_270(contents.clone()) {
                    Ok((edi270, _)) => {
                        let new_edi = write_270(&edi270);
                        write_to_file(new_edi, args.output_file);
                    },
                    Err(e) => {
                        warn!("Error processing 270 format: {:?}", e);
                    }
                }
            }
            // Check if the content is raw EDI for 271 format
            else if contents.contains("BHT*0022*11") || (contents.contains("EB*") && contents.contains("HL*")) {
                info!("Writing 271 format from raw EDI");
                match get_271(contents.clone()) {
                    Ok((edi271, _)) => {
                        let new_edi = write_271(&edi271);
                        write_to_file(new_edi, args.output_file);
                    },
                    Err(e) => {
                        warn!("Error processing 271 format: {:?}", e);
                    }
                }
            }
            // Check if the content is raw EDI for 837P format
            else if contents.contains("ST*837*") && contents.contains("BHT*0019*00*") {
                info!("Writing 837P format from raw EDI");
                match get_837p(&contents) {
                    Ok(edi837p) => {
                        match write_837p(&edi837p) {
                            Ok(new_edi) => write_to_file(new_edi, args.output_file),
                            Err(e) => warn!("Error writing 837P format: {:?}", e)
                        }
                    },
                    Err(e) => {
                        warn!("Error processing 837P format: {:?}", e);
                    }
                }
            }
            else {
                warn!("Unknown format for writing");
            }
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
        } else if contents.contains("~ST*270*") || contents.contains("ST*270*") {
            info!("File is 270");
            match get_270(contents.clone()) {
                Ok((edi270, _)) => {
                    let serialized_edi = serde_json::to_string(&edi270).unwrap();
                    write_to_file(serialized_edi.clone(), args.output_file);
                },
                Err(e) => {
                    warn!("Error processing 270 format: {:?}", e);
                }
            }
        } else if contents.contains("~ST*271*") || contents.contains("ST*271*") {
            info!("File is 271");
            match get_271(contents.clone()) {
                Ok((edi271, _)) => {
                    let serialized_edi = serde_json::to_string(&edi271).unwrap();
                    write_to_file(serialized_edi.clone(), args.output_file);
                },
                Err(e) => {
                    warn!("Error processing 271 format: {:?}", e);
                }
            }
        } else if contents.contains("~ST*276*") || contents.contains("ST*276*") {
            info!("File is 276");
            match get_276(contents.clone()) {
                Ok(edi276) => {
                    let serialized_edi = serde_json::to_string(&edi276).unwrap();
                    write_to_file(serialized_edi.clone(), args.output_file);
                },
                Err(e) => {
                    warn!("Error processing 276 format: {:?}", e);
                }
            }
        } else if contents.contains("~ST*277*") || contents.contains("ST*277*") {
            info!("File is 277");
            match get_277(contents.clone()) {
                Ok(edi277) => {
                    let serialized_edi = serde_json::to_string(&edi277).unwrap();
                    write_to_file(serialized_edi.clone(), args.output_file);
                },
                Err(e) => {
                    warn!("Error processing 277 format: {:?}", e);
                }
            }
        } else if contents.contains("~ST*837*") || contents.contains("ST*837*") {
            info!("File is 837");
            // Try to determine the specific 837 variant
            if contents.contains("BHT*0019*00*") {
                info!("File is 837P");
                match get_837p(&contents) {
                    Ok(edi837p) => {
                        let serialized_edi = serde_json::to_string(&edi837p).unwrap();
                        write_to_file(serialized_edi.clone(), args.output_file);
                    },
                    Err(e) => {
                        warn!("Error processing 837P format: {:?}", e);
                    }
                }
            } else if contents.contains("005010X223") {
                info!("File is 837I");
                match get_837i(&contents) {
                    Ok(edi837i) => {
                        let serialized_edi = serde_json::to_string(&edi837i).unwrap();
                        write_to_file(serialized_edi.clone(), args.output_file);
                    },
                    Err(e) => {
                        warn!("Error processing 837I format: {:?}", e);
                    }
                }
            } else {
                warn!("Unable to determine specific 837 variant. Currently supporting 837P and 837I.");
            }
        } else {
            warn!("File format not recognized. Currently supporting 835, 999, 270, 271, 276, 277, and 837 formats.");
        }
    }
}
