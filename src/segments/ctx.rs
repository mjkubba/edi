// use crate::helper::edihelper::stiuational_element;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default,PartialEq,Clone,Serialize, Deserialize)]
#[allow(dead_code)]
pub struct CTX {
    pub ctx01_context_id: String,
    pub ctx02_segment_id_code: String,
    pub ctx03_segment_position_in_transaction: String,
    pub ctx04_loop_id_code: String,
    pub ctx05_position_in_segment: String,
    pub ctx06_ref_in_segment: String,
    
}

pub fn get_ctx(ctx_content: String) -> CTX {
    let ctx_parts: Vec<&str> = ctx_content.split("*").collect();
    let mut ctx02_segment_id_code = String::new();
    let mut ctx03_segment_position_in_transaction = String::new();
    let mut ctx04_loop_id_code = String::new();
    let mut ctx05_position_in_segment = String::new();
    let mut ctx06_ref_in_segment = String::new();

    if ctx_parts.get(3).is_some() {
        ctx02_segment_id_code = ctx_parts[3].to_string();
    }

    if ctx_parts.get(3).is_some() {
        ctx03_segment_position_in_transaction = ctx_parts[3].to_string();
    }

    if ctx_parts.get(3).is_some() {
        ctx04_loop_id_code = ctx_parts[3].to_string();
    }
    if ctx_parts.get(4).is_some() {
        ctx05_position_in_segment = ctx_parts[3].to_string();
    }
    if ctx_parts.get(5).is_some() {
        ctx06_ref_in_segment = ctx_parts[3].to_string();
    }

    CTX {
        ctx01_context_id: ctx_parts[0].to_string(),
        ctx02_segment_id_code,
        ctx03_segment_position_in_transaction,
        ctx04_loop_id_code,
        ctx05_position_in_segment,
        ctx06_ref_in_segment,
    }
}
 


// pub fn write_ctx(ctx:CTX) -> String {
//     let mut ctx_content = String::new();
//     ctx_content.push_str("CTX*");
//     ctx_content.push_str(&ctx.ctx01_context_id);
//     ctx_content.push_str("*");
//     ctx_content.push_str(&ctx.ctx02_segment_id_code);
//     ctx_content.push_str("*");
//     ctx_content.push_str(&ctx.ctx03_segment_position_in_transaction);
//     ctx_content.push_str(&stiuational_element(ctx.ctx04_loop_id_code));
//     ctx_content.push_str(&stiuational_element(ctx.ctx05_position_in_segment));
//     ctx_content.push_str(&stiuational_element(ctx.ctx06_ref_in_segment));
//     ctx_content.push_str("~");
//     ctx_content
// }

// unit test

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ak1() {
        let ctx_content = "A*1*2~".to_string();
        let ctx = get_ctx(ctx_content);
        assert_eq!(ctx.ctx01_context_id, "A");
        assert_eq!(ctx.ctx02_segment_id_code, "1");
    }
}