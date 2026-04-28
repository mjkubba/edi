// Many public functions/structs exist for API completeness but aren't called from the CLI binary.
#![allow(dead_code)]

/**
 * EDI Parser and Processor for Healthcare X12 Formats
 *
 * Supports 835, 999, 270/271, 276/277, 837, 278, 820, and 834.
 */
use crate::edi270::controller::{get_270, write_270, Edi270};
use crate::edi271::controller::{get_271, write_271, Edi271};
use crate::edi276::controller::{get_276, write_276, Edi276};
use crate::edi277::controller::{get_277, write_277, Edi277};
use crate::edi278::controller::{write_278, Edi278};
use crate::edi820::controller::{write_820, Edi820};
use crate::edi834::controller::{write_834, Edi834};
use crate::edi835::controller::{get_835, write_835, Edi835};
use crate::edi837::controller::{get_837, write_837, Edi837};
use crate::edi999::controller::{get_999, write_999, Edi999};
use crate::helper::helper::{
    clean_contents, get_file_contents, process_args, set_logger, write_to_file,
};
use crate::transaction_processor::TransactionSet;

mod edi270;
mod edi271;
mod edi276;
mod edi277;
mod edi278;
mod edi820;
mod edi834;
mod edi835;
mod edi837;
mod edi999;
mod error;
mod helper;
mod segments;
mod transaction_processor;

use log::{info, warn};

/// Detect the ST transaction set code from raw EDI content
fn detect_st_code(contents: &str) -> Option<&str> {
    // Look for ST*XXX* pattern
    let st_pos = contents.find("ST*")?;
    let after_st = &contents[st_pos + 3..];
    let end = after_st.find('*')?;
    Some(&after_st[..end])
}

/// Read path: parse raw EDI → serialize to JSON → write file
fn read_edi(contents: &str, output_file: String) {
    let serialize_and_write = |json: Result<String, serde_json::Error>| match json {
        Ok(s) => write_to_file(s, output_file.clone()),
        Err(e) => warn!("Failed to serialize: {}", e),
    };

    match detect_st_code(contents) {
        Some("835") => {
            info!("File is 835");
            serialize_and_write(serde_json::to_string(&get_835(contents)));
        }
        Some("999") => {
            info!("File is 999");
            serialize_and_write(serde_json::to_string(&get_999(contents).0));
        }
        Some("270") => {
            info!("File is 270");
            match get_270(contents) {
                Ok((edi, _)) => serialize_and_write(serde_json::to_string(&edi)),
                Err(e) => warn!("Error processing 270: {:?}", e),
            }
        }
        Some("271") => {
            info!("File is 271");
            match get_271(contents) {
                Ok((edi, _)) => serialize_and_write(serde_json::to_string(&edi)),
                Err(e) => warn!("Error processing 271: {:?}", e),
            }
        }
        Some("276") => {
            info!("File is 276");
            match get_276(contents) {
                Ok(edi) => serialize_and_write(serde_json::to_string(&edi)),
                Err(e) => warn!("Error processing 276: {:?}", e),
            }
        }
        Some("277") => {
            info!("File is 277");
            match get_277(contents) {
                Ok(edi) => serialize_and_write(serde_json::to_string(&edi)),
                Err(e) => warn!("Error processing 277: {:?}", e),
            }
        }
        Some("837") => {
            info!("File is 837");
            match get_837(contents) {
                Ok(edi) => serialize_and_write(serde_json::to_string(&edi)),
                Err(e) => warn!("Error processing 837: {:?}", e),
            }
        }
        Some("278") => {
            info!("File is 278");
            match Edi278::parse(contents.to_string()) {
                Ok((edi, _)) => serialize_and_write(serde_json::to_string(&edi)),
                Err(e) => warn!("Error processing 278: {:?}", e),
            }
        }
        Some("820") => {
            info!("File is 820");
            match Edi820::parse(contents.to_string()) {
                Ok((edi, _)) => serialize_and_write(serde_json::to_string(&edi)),
                Err(e) => warn!("Error processing 820: {:?}", e),
            }
        }
        Some("834") => {
            info!("File is 834");
            match Edi834::parse(contents.to_string()) {
                Ok((edi, _)) => serialize_and_write(serde_json::to_string(&edi)),
                Err(e) => warn!("Error processing 834: {:?}", e),
            }
        }
        Some(code) => warn!("Unsupported transaction set: {}", code),
        None => warn!("Could not detect transaction set type. Expected ST*XXX* segment."),
    }
}

/// Write path from JSON: deserialize JSON → generate EDI → write file
fn write_from_json(contents: &str, output_file: String) {
    // Helper: deserialize, convert, write
    fn try_write<T, F>(contents: &str, output_file: String, convert: F)
    where
        T: serde::de::DeserializeOwned,
        F: FnOnce(T) -> String,
    {
        match serde_json::from_str::<T>(contents) {
            Ok(v) => write_to_file(convert(v), output_file),
            Err(e) => warn!("Failed to parse JSON: {}", e),
        }
    }

    // Detect format from JSON content
    if contents.contains("\"transaction_set_id\":\"835\"") {
        info!("Writing 835");
        try_write::<Edi835, _>(contents, output_file, |_| write_835(contents.to_string()));
    } else if contents.contains("\"transaction_set_id\":\"999\"") {
        info!("Writing 999");
        try_write::<Edi999, _>(contents, output_file, |edi| write_999(&edi));
    } else if contents.contains("\"transaction_set_id\":\"270\"") {
        info!("Writing 270");
        try_write::<Edi270, _>(contents, output_file, |edi| write_270(&edi));
    } else if contents.contains("\"transaction_set_id\":\"271\"") {
        info!("Writing 271");
        try_write::<Edi271, _>(contents, output_file, |edi| write_271(&edi));
    } else if contents.contains("\"st01_transaction_set_identifier_code\":\"276\"") {
        info!("Writing 276");
        try_write::<Edi276, _>(contents, output_file, |edi| write_276(&edi));
    } else if contents.contains("\"st01_transaction_set_identifier_code\":\"277\"") {
        info!("Writing 277");
        try_write::<Edi277, _>(contents, output_file, |edi| write_277(&edi));
    } else if contents.contains("005010X222")
        || contents.contains("005010X223")
        || contents.contains("005010X224")
    {
        info!("Writing 837");
        match serde_json::from_str::<Edi837>(contents) {
            Ok(edi) => match write_837(&edi) {
                Ok(new_edi) => write_to_file(new_edi, output_file),
                Err(e) => warn!("Error writing 837: {:?}", e),
            },
            Err(e) => warn!("Failed to parse JSON: {}", e),
        }
    } else if contents.contains("\"transaction_set_id\":\"278\"") {
        info!("Writing 278");
        try_write::<Edi278, _>(contents, output_file, |edi| write_278(&edi));
    } else if contents.contains("\"transaction_set_id\":\"820\"") {
        info!("Writing 820");
        try_write::<Edi820, _>(contents, output_file, |edi| write_820(&edi));
    } else if contents.contains("\"transaction_set_id\":\"834\"") {
        info!("Writing 834");
        try_write::<Edi834, _>(contents, output_file, |edi| write_834(&edi));
    } else {
        warn!("Unknown JSON format for writing");
    }
}

/// Write path from raw EDI: parse → regenerate → write file
fn write_from_edi(contents: &str, output_file: String) {
    match detect_st_code(contents) {
        Some("835") => {
            info!("Writing 835 from raw EDI");
            let edi = get_835(contents);
            match serde_json::to_string(&edi) {
                Ok(json) => write_to_file(write_835(json), output_file),
                Err(e) => warn!("Failed to serialize: {}", e),
            }
        }
        Some("999") => {
            info!("Writing 999 from raw EDI");
            let (edi, _) = get_999(contents);
            write_to_file(write_999(&edi), output_file);
        }
        Some("270") => {
            info!("Writing 270 from raw EDI");
            match get_270(contents) {
                Ok((edi, _)) => write_to_file(write_270(&edi), output_file),
                Err(e) => warn!("Error processing 270: {:?}", e),
            }
        }
        Some("271") => {
            info!("Writing 271 from raw EDI");
            match get_271(contents) {
                Ok((edi, _)) => write_to_file(write_271(&edi), output_file),
                Err(e) => warn!("Error processing 271: {:?}", e),
            }
        }
        Some("837") => {
            info!("Writing 837 from raw EDI");
            match get_837(contents) {
                Ok(edi) => match write_837(&edi) {
                    Ok(new_edi) => write_to_file(new_edi, output_file),
                    Err(e) => warn!("Error writing 837: {:?}", e),
                },
                Err(e) => warn!("Error processing 837: {:?}", e),
            }
        }
        Some(code) => warn!("Raw EDI write not supported for transaction set: {}", code),
        None => warn!("Could not detect transaction set type"),
    }
}

fn main() {
    set_logger();
    info!("Starting EDI Parser");

    let args = process_args();
    let contents = get_file_contents(args.clone());
    let contents = clean_contents(contents);

    // Validate X12 envelope if this is raw EDI (not JSON)
    if !args.is_json && !contents.starts_with('{') {
        crate::helper::envelope_validation::validate_raw_envelope(&contents);
    }

    match args.operation.as_str() {
        "write" => {
            info!("Write EDI Operation");
            if args.is_json {
                info!("Input is JSON");
                write_from_json(&contents, args.output_file);
            } else {
                info!("Input is raw EDI");
                write_from_edi(&contents, args.output_file);
            }
        }
        "read" => {
            info!("Read EDI Operation");
            read_edi(&contents, args.output_file);
        }
        _ => warn!("Unknown operation: {}", args.operation),
    }
}
