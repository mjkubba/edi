use log::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CTX {
    pub ctx01_context_name: String,
    pub ctx02_segment_id_code: String,
    pub ctx03_segment_position_in_transaction: String,
    pub ctx04_loop_id_code: String,
    pub ctx05_position_in_segment: String,
    pub ctx06_reference_in_segment: String,
}

pub fn get_ctx(ctx_content: String) -> CTX {
    // Remove the segment identifier if present
    let content = if ctx_content.starts_with("CTX*") {
        ctx_content[4..].to_string()
    } else {
        ctx_content
    };
    
    // Special handling for formats like "CLM01:123456789"
    if !content.contains('*') {
        info!("Special CTX format detected: {}", content);
        return CTX {
            ctx01_context_name: content,
            ..CTX::default()
        };
    }
    
    let ctx_parts: Vec<&str> = content.split("*").collect();
    info!("CTX parts after split: {:?}", ctx_parts);
    
    let mut ctx = CTX::default();
    
    // CTX01 - Context Name (Required)
    if !ctx_parts.is_empty() {
        ctx.ctx01_context_name = ctx_parts[0].to_string();
    }
    
    // CTX02 - Segment ID Code (Situational)
    if ctx_parts.len() > 1 && !ctx_parts[1].is_empty() {
        ctx.ctx02_segment_id_code = ctx_parts[1].to_string();
    }
    
    // CTX03 - Segment Position in Transaction Set (Situational)
    if ctx_parts.len() > 2 && !ctx_parts[2].is_empty() {
        ctx.ctx03_segment_position_in_transaction = ctx_parts[2].to_string();
    }
    
    // CTX04 - Loop Identifier Code (Situational)
    if ctx_parts.len() > 3 && !ctx_parts[3].is_empty() {
        ctx.ctx04_loop_id_code = ctx_parts[3].to_string();
    }
    
    // CTX05 - Position in Segment (Situational)
    if ctx_parts.len() > 4 && !ctx_parts[4].is_empty() {
        // Handle special format like "5:3"
        ctx.ctx05_position_in_segment = ctx_parts[4].to_string();
    }
    
    // CTX06 - Reference in Segment (Situational)
    if ctx_parts.len() > 5 && !ctx_parts[5].is_empty() {
        // Handle special format like "C023:1325"
        ctx.ctx06_reference_in_segment = ctx_parts[5].to_string();
    }
    
    info!("Parsed CTX segment: {:?}", ctx);
    ctx
}

pub fn write_ctx(ctx: CTX) -> String {
    let mut ctx_content = String::new();
    info!("Writing CTX segment: {:?}", ctx);
    
    // Special handling for formats like "CLM01:123456789"
    if ctx.ctx01_context_name.contains(':') && 
       ctx.ctx02_segment_id_code.is_empty() && 
       ctx.ctx03_segment_position_in_transaction.is_empty() && 
       ctx.ctx04_loop_id_code.is_empty() && 
       ctx.ctx05_position_in_segment.is_empty() && 
       ctx.ctx06_reference_in_segment.is_empty() {
        ctx_content.push_str("CTX*");
        ctx_content.push_str(&ctx.ctx01_context_name);
        ctx_content.push_str("~");
        return ctx_content;
    }
    
    ctx_content.push_str("CTX*");
    ctx_content.push_str(&ctx.ctx01_context_name);
    
    // Only include non-empty fields
    if !ctx.ctx02_segment_id_code.is_empty() {
        ctx_content.push_str("*");
        ctx_content.push_str(&ctx.ctx02_segment_id_code);
        
        if !ctx.ctx03_segment_position_in_transaction.is_empty() {
            ctx_content.push_str("*");
            ctx_content.push_str(&ctx.ctx03_segment_position_in_transaction);
            
            if !ctx.ctx04_loop_id_code.is_empty() {
                ctx_content.push_str("*");
                ctx_content.push_str(&ctx.ctx04_loop_id_code);
                
                if !ctx.ctx05_position_in_segment.is_empty() {
                    ctx_content.push_str("*");
                    ctx_content.push_str(&ctx.ctx05_position_in_segment);
                    
                    if !ctx.ctx06_reference_in_segment.is_empty() {
                        ctx_content.push_str("*");
                        ctx_content.push_str(&ctx.ctx06_reference_in_segment);
                    }
                }
            } else {
                // Add empty field for ctx04_loop_id_code if ctx05_position_in_segment is not empty
                if !ctx.ctx05_position_in_segment.is_empty() {
                    ctx_content.push_str("*");
                    
                    ctx_content.push_str("*");
                    ctx_content.push_str(&ctx.ctx05_position_in_segment);
                    
                    if !ctx.ctx06_reference_in_segment.is_empty() {
                        ctx_content.push_str("*");
                        ctx_content.push_str(&ctx.ctx06_reference_in_segment);
                    }
                }
            }
        }
    }
    
    ctx_content.push_str("~");
    ctx_content
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ctx_with_prefix() {
        let ctx_content = "CTX*SITUATIONAL TRIGGER*IK3*2*2000*1*1".to_string();
        let ctx = get_ctx(ctx_content);
        assert_eq!(ctx.ctx01_context_name, "SITUATIONAL TRIGGER");
        assert_eq!(ctx.ctx02_segment_id_code, "IK3");
        assert_eq!(ctx.ctx03_segment_position_in_transaction, "2");
        assert_eq!(ctx.ctx04_loop_id_code, "2000");
        assert_eq!(ctx.ctx05_position_in_segment, "1");
        assert_eq!(ctx.ctx06_reference_in_segment, "1");
    }
    
    #[test]
    fn test_ctx_without_prefix() {
        let ctx_content = "SITUATIONAL TRIGGER*IK3*2*2000*1*1".to_string();
        let ctx = get_ctx(ctx_content);
        assert_eq!(ctx.ctx01_context_name, "SITUATIONAL TRIGGER");
        assert_eq!(ctx.ctx02_segment_id_code, "IK3");
        assert_eq!(ctx.ctx03_segment_position_in_transaction, "2");
        assert_eq!(ctx.ctx04_loop_id_code, "2000");
        assert_eq!(ctx.ctx05_position_in_segment, "1");
        assert_eq!(ctx.ctx06_reference_in_segment, "1");
    }
    
    #[test]
    fn test_ctx_with_special_format() {
        let ctx_content = "CLM01:123456789".to_string();
        let ctx = get_ctx(ctx_content);
        assert_eq!(ctx.ctx01_context_name, "CLM01:123456789");
        assert_eq!(ctx.ctx02_segment_id_code, "");
    }
    
    #[test]
    fn test_ctx_with_complex_format() {
        let ctx_content = "SITUATIONAL TRIGGER*CLM*43**5:3*C023:1325".to_string();
        let ctx = get_ctx(ctx_content);
        assert_eq!(ctx.ctx01_context_name, "SITUATIONAL TRIGGER");
        assert_eq!(ctx.ctx02_segment_id_code, "CLM");
        assert_eq!(ctx.ctx03_segment_position_in_transaction, "43");
        assert_eq!(ctx.ctx04_loop_id_code, "");
        assert_eq!(ctx.ctx05_position_in_segment, "5:3");
        assert_eq!(ctx.ctx06_reference_in_segment, "C023:1325");
    }
    
    #[test]
    fn test_write_ctx() {
        let ctx = CTX {
            ctx01_context_name: "SITUATIONAL TRIGGER".to_string(),
            ctx02_segment_id_code: "IK3".to_string(),
            ctx03_segment_position_in_transaction: "2".to_string(),
            ctx04_loop_id_code: "2000".to_string(),
            ctx05_position_in_segment: "1".to_string(),
            ctx06_reference_in_segment: "1".to_string(),
        };
        
        let ctx_content = write_ctx(ctx);
        assert_eq!(ctx_content, "CTX*SITUATIONAL TRIGGER*IK3*2*2000*1*1~");
    }
    
    #[test]
    fn test_write_ctx_minimal() {
        let ctx = CTX {
            ctx01_context_name: "CLM01:123456789".to_string(),
            ctx02_segment_id_code: "".to_string(),
            ctx03_segment_position_in_transaction: "".to_string(),
            ctx04_loop_id_code: "".to_string(),
            ctx05_position_in_segment: "".to_string(),
            ctx06_reference_in_segment: "".to_string(),
        };
        
        let ctx_content = write_ctx(ctx);
        assert_eq!(ctx_content, "CTX*CLM01:123456789~");
    }
    
    #[test]
    fn test_write_ctx_complex() {
        let ctx = CTX {
            ctx01_context_name: "SITUATIONAL TRIGGER".to_string(),
            ctx02_segment_id_code: "CLM".to_string(),
            ctx03_segment_position_in_transaction: "43".to_string(),
            ctx04_loop_id_code: "".to_string(),
            ctx05_position_in_segment: "5:3".to_string(),
            ctx06_reference_in_segment: "C023:1325".to_string(),
        };
        
        let ctx_content = write_ctx(ctx);
        assert_eq!(ctx_content, "CTX*SITUATIONAL TRIGGER*CLM*43**5:3*C023:1325~");
    }
}
