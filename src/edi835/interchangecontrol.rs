use crate::segments::isa::*;
use crate::segments::gs::*;
use crate::helper::helper::*;


pub fn get_interchange_control(mut contents:String) -> (ISA, GS, String) {
    let mut isa_segments = ISA::default();
    let mut gs_segments = GS::default();
    if contents.contains("ISA") {
        print!("ISA segment found, ");
        isa_segments = get_isa(get_segment_contents("ISA", &contents));
        println!("ISA segment parsed");

        contents = content_trim("ISA", contents);
    }
        if contents.contains("GS") {
        print!("GS segment found, ");
        gs_segments = get_gs(get_segment_contents("GS", &contents));
        println!("GS segment parsed");
 
        contents = content_trim("GS",contents);
    }
    
    println!("Interchange Control parsed\n");
    return (isa_segments, gs_segments, contents)
}