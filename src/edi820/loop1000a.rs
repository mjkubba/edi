use log::info;
use serde::{Serialize, Deserialize};
use crate::helper::edihelper::*;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop1000as {
    pub n1_segments: N1,
    pub n3_segments: Option<N3>,
    pub n4_segments: Option<N4>,
    pub ref_segments: Vec<REF>,
    pub per_segments: Vec<PER>,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct N1 {
    pub entity_id: String,
    pub entity_name: String,
    pub id_code_qualifier: String,
    pub id_code: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct N3 {
    pub address: String,
    pub address2: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct N4 {
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country_code: String,
    pub location_qualifier: String,
    pub location_identifier: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct REF {
    pub reference_id_qualifier: String,
    pub reference_id: String,
    pub description: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PER {
    pub contact_function_code: String,
    pub name: String,
    pub communication_number_qualifier: String,
    pub communication_number: String,
    pub communication_number_qualifier2: String,
    pub communication_number2: String,
    pub communication_number_qualifier3: String,
    pub communication_number3: String,
}

pub fn get_1000as(mut contents: String) -> (Loop1000as, String) {
    let mut loop1000as = Loop1000as::default();
    
    // Parse N1 segment for payer identification
    if contents.contains("N1") {
        let n1_content = get_segment_contents("N1", &contents);
        let n1_parts: Vec<&str> = n1_content.split('*').collect();
        
        // Check if this is a payer identification segment (N101=PE)
        if n1_parts.len() > 1 && n1_parts[1] == "PE" {
            info!("N1 segment found for Payer Identification, ");
            
            loop1000as.n1_segments = N1 {
                entity_id: n1_parts[1].to_string(),
                entity_name: if n1_parts.len() > 2 { n1_parts[2].to_string() } else { String::new() },
                id_code_qualifier: if n1_parts.len() > 3 { n1_parts[3].to_string() } else { String::new() },
                id_code: if n1_parts.len() > 4 { n1_parts[4].to_string() } else { String::new() },
            };
            
            info!("N1 segment parsed");
            contents = content_trim("N1", contents);
            
            // Parse N3 segment (address)
            if contents.contains("N3") && check_if_segement_in_loop("N3", "N4", contents.clone()) && 
               check_if_segement_in_loop("N3", "REF", contents.clone()) && 
               check_if_segement_in_loop("N3", "PER", contents.clone()) && 
               check_if_segement_in_loop("N3", "N1", contents.clone()) {
                
                info!("N3 segment found, ");
                let n3_content = get_segment_contents("N3", &contents);
                let n3_parts: Vec<&str> = n3_content.split('*').collect();
                
                loop1000as.n3_segments = Some(N3 {
                    address: if n3_parts.len() > 1 { n3_parts[1].to_string() } else { String::new() },
                    address2: if n3_parts.len() > 2 { n3_parts[2].to_string() } else { String::new() },
                });
                
                info!("N3 segment parsed");
                contents = content_trim("N3", contents);
            }
            
            // Parse N4 segment (city, state, zip)
            if contents.contains("N4") && check_if_segement_in_loop("N4", "REF", contents.clone()) && 
               check_if_segement_in_loop("N4", "PER", contents.clone()) && 
               check_if_segement_in_loop("N4", "N1", contents.clone()) {
                
                info!("N4 segment found, ");
                let n4_content = get_segment_contents("N4", &contents);
                let n4_parts: Vec<&str> = n4_content.split('*').collect();
                
                loop1000as.n4_segments = Some(N4 {
                    city: if n4_parts.len() > 1 { n4_parts[1].to_string() } else { String::new() },
                    state: if n4_parts.len() > 2 { n4_parts[2].to_string() } else { String::new() },
                    zip: if n4_parts.len() > 3 { n4_parts[3].to_string() } else { String::new() },
                    country_code: if n4_parts.len() > 4 { n4_parts[4].to_string() } else { String::new() },
                    location_qualifier: if n4_parts.len() > 5 { n4_parts[5].to_string() } else { String::new() },
                    location_identifier: if n4_parts.len() > 6 { n4_parts[6].to_string() } else { String::new() },
                });
                
                info!("N4 segment parsed");
                contents = content_trim("N4", contents);
            }
            
            // Parse REF segments
            while contents.contains("REF") && check_if_segement_in_loop("REF", "PER", contents.clone()) && 
                  check_if_segement_in_loop("REF", "N1", contents.clone()) {
                
                info!("REF segment found, ");
                let ref_content = get_segment_contents("REF", &contents);
                let ref_parts: Vec<&str> = ref_content.split('*').collect();
                
                let ref_segment = REF {
                    reference_id_qualifier: if ref_parts.len() > 1 { ref_parts[1].to_string() } else { String::new() },
                    reference_id: if ref_parts.len() > 2 { ref_parts[2].to_string() } else { String::new() },
                    description: if ref_parts.len() > 3 { ref_parts[3].to_string() } else { String::new() },
                };
                
                loop1000as.ref_segments.push(ref_segment);
                
                info!("REF segment parsed");
                contents = content_trim("REF", contents);
            }
            
            // Parse PER segments
            while contents.contains("PER") && check_if_segement_in_loop("PER", "N1", contents.clone()) {
                info!("PER segment found, ");
                let per_content = get_segment_contents("PER", &contents);
                let per_parts: Vec<&str> = per_content.split('*').collect();
                
                let per_segment = PER {
                    contact_function_code: if per_parts.len() > 1 { per_parts[1].to_string() } else { String::new() },
                    name: if per_parts.len() > 2 { per_parts[2].to_string() } else { String::new() },
                    communication_number_qualifier: if per_parts.len() > 3 { per_parts[3].to_string() } else { String::new() },
                    communication_number: if per_parts.len() > 4 { per_parts[4].to_string() } else { String::new() },
                    communication_number_qualifier2: if per_parts.len() > 5 { per_parts[5].to_string() } else { String::new() },
                    communication_number2: if per_parts.len() > 6 { per_parts[6].to_string() } else { String::new() },
                    communication_number_qualifier3: if per_parts.len() > 7 { per_parts[7].to_string() } else { String::new() },
                    communication_number3: if per_parts.len() > 8 { per_parts[8].to_string() } else { String::new() },
                };
                
                loop1000as.per_segments.push(per_segment);
                
                info!("PER segment parsed");
                contents = content_trim("PER", contents);
            }
        }
    }
    
    info!("Loop 1000A parsed\n");
    
    return (loop1000as, contents);
}

pub fn write_loop1000a(loop1000as: Loop1000as) -> String {
    let mut result = String::new();
    
    // Write N1 segment
    if !loop1000as.n1_segments.entity_id.is_empty() {
        result.push_str("N1*");
        result.push_str(&loop1000as.n1_segments.entity_id);
        result.push_str("*");
        result.push_str(&loop1000as.n1_segments.entity_name);
        
        if !loop1000as.n1_segments.id_code_qualifier.is_empty() {
            result.push_str("*");
            result.push_str(&loop1000as.n1_segments.id_code_qualifier);
            
            if !loop1000as.n1_segments.id_code.is_empty() {
                result.push_str("*");
                result.push_str(&loop1000as.n1_segments.id_code);
            }
        }
        
        result.push_str("~\n");
    }
    
    // Write N3 segment if it exists
    if let Some(n3) = &loop1000as.n3_segments {
        if !n3.address.is_empty() {
            result.push_str("N3*");
            result.push_str(&n3.address);
            
            if !n3.address2.is_empty() {
                result.push_str("*");
                result.push_str(&n3.address2);
            }
            
            result.push_str("~\n");
        }
    }
    
    // Write N4 segment if it exists
    if let Some(n4) = &loop1000as.n4_segments {
        if !n4.city.is_empty() {
            result.push_str("N4*");
            result.push_str(&n4.city);
            
            if !n4.state.is_empty() {
                result.push_str("*");
                result.push_str(&n4.state);
                
                if !n4.zip.is_empty() {
                    result.push_str("*");
                    result.push_str(&n4.zip);
                    
                    if !n4.country_code.is_empty() {
                        result.push_str("*");
                        result.push_str(&n4.country_code);
                        
                        if !n4.location_qualifier.is_empty() {
                            result.push_str("*");
                            result.push_str(&n4.location_qualifier);
                            
                            if !n4.location_identifier.is_empty() {
                                result.push_str("*");
                                result.push_str(&n4.location_identifier);
                            }
                        }
                    }
                }
            }
            
            result.push_str("~\n");
        }
    }
    
    // Write REF segments
    for ref_segment in &loop1000as.ref_segments {
        if !ref_segment.reference_id_qualifier.is_empty() {
            result.push_str("REF*");
            result.push_str(&ref_segment.reference_id_qualifier);
            
            if !ref_segment.reference_id.is_empty() {
                result.push_str("*");
                result.push_str(&ref_segment.reference_id);
                
                if !ref_segment.description.is_empty() {
                    result.push_str("*");
                    result.push_str(&ref_segment.description);
                }
            }
            
            result.push_str("~\n");
        }
    }
    
    // Write PER segments
    for per_segment in &loop1000as.per_segments {
        if !per_segment.contact_function_code.is_empty() {
            result.push_str("PER*");
            result.push_str(&per_segment.contact_function_code);
            
            if !per_segment.name.is_empty() {
                result.push_str("*");
                result.push_str(&per_segment.name);
                
                if !per_segment.communication_number_qualifier.is_empty() {
                    result.push_str("*");
                    result.push_str(&per_segment.communication_number_qualifier);
                    
                    if !per_segment.communication_number.is_empty() {
                        result.push_str("*");
                        result.push_str(&per_segment.communication_number);
                        
                        if !per_segment.communication_number_qualifier2.is_empty() {
                            result.push_str("*");
                            result.push_str(&per_segment.communication_number_qualifier2);
                            
                            if !per_segment.communication_number2.is_empty() {
                                result.push_str("*");
                                result.push_str(&per_segment.communication_number2);
                                
                                if !per_segment.communication_number_qualifier3.is_empty() {
                                    result.push_str("*");
                                    result.push_str(&per_segment.communication_number_qualifier3);
                                    
                                    if !per_segment.communication_number3.is_empty() {
                                        result.push_str("*");
                                        result.push_str(&per_segment.communication_number3);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            result.push_str("~\n");
        }
    }
    
    return result;
}
