use std::fs::File;
use std::io::Read;

mod edi835;
use edi835::{bpr::*,dtm::*,gs::*,isa::*,lx::*,n1::*,n3::*,n4::*,nm1::*,per::*,r#ref::*,st::*,trn::*};

fn get_segment_contents<'a>(key:&str, contents: &'a str) -> &'a str {
    let start_skip = key.len() + 1;
    let index = contents.find(&key).unwrap();
    let start = &contents[index..];
    let end = start.find("~").unwrap();
    let content = &start[start_skip..end];
    content
}


fn main() {
    // Open File and read content
    let mut file = File::open("./src/edi835-1.edi").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // TODO: create structs for these segments
    // find the first occurrence of "ISA" in the contents of the file and extract the content between "ISA" and "~"
    if contents.contains("ISA") {
        println!("ISA segment found");
        let isa_segments = get_isa(get_segment_contents("ISA", &contents));
        println!("{:?}", isa_segments);
        println!("ISA segment parsed");
        println!("\n");
    }
    
    // find how many gs segments are in the file
    if contents.contains("GS") {
        println!("GS segment found");
        let gs_segments = get_gs(get_segment_contents("GS", &contents));
        println!("{:?}", gs_segments);
        println!("GS segment parsed");
        println!("\n");
    }
    
    if contents.contains("ST") {
        println!("ST segment found");
        let st_segments = get_st(get_segment_contents("ST", &contents));
        println!("{:?}", st_segments);
        println!("ST segment parsed");
        println!("\n");
    }

    if contents.contains("BPR") {
        println!("BPR segment found");
        let bpr_segments = get_bpr(get_segment_contents("BPR", &contents));
        println!("{:?}", bpr_segments);
        println!("BPR segment parsed");
        println!("\n");
    }

    if contents.contains("TRN") {
        println!("TRN segment found");
        let trn_segments = get_trn(get_segment_contents("TRN", &contents));
        println!("{:?}", trn_segments);
        println!("TRN segment parsed");
        println!("\n");
    }

    if contents.contains("REF") {
        println!("REF segment found");
        let ref_segments = get_ref(get_segment_contents("REF", &contents));
        println!("{:?}", ref_segments);
        println!("REF segment parsed");
        println!("\n");
    }

    if contents.contains("NM1") {
        // find how many nm1 segments are in the file
        let nm1_count= contents.matches("NM1").count();
        println!("Number of NM1 segments: {}", nm1_count);

        let mut next_segment =  &contents[contents.find("NM1").unwrap()..];
        let mut nm1_vec = Vec::new();

        for _ in 0..nm1_count {
            let nm1_start = next_segment;
            let nm1_end = nm1_start.find("~").unwrap();
            let nm1_content = &nm1_start[4..nm1_end];
            let nm1_segments = get_nm1(nm1_content);
            nm1_vec.push(nm1_segments);

            next_segment = &nm1_start[nm1_end+1..]
        }
        println!("{:?}", nm1_vec);
        println!("NM1 segment parsed");
        println!("\n");
    }

    if contents.contains("N1") {
        println!("N1 segment found");
        let n1_segments = get_n1(get_segment_contents("N1", &contents));
        println!("{:?}", n1_segments);
        println!("N1 segment parsed");
        println!("\n");
    } 

    if contents.contains("N3") {
        println!("N3 segment found");
        let n3_segments = get_n3(get_segment_contents("N3", &contents));
        println!("{:?}", n3_segments);
        println!("N3 segment parsed");
        println!("\n");
    }

    if contents.contains("N4") {
        println!("N4 segment found");
        let n4_segments = get_n4(get_segment_contents("N4", &contents));
        println!("{:?}", n4_segments);
        println!("N4 segment parsed");
        println!("\n");
    }

    if contents.contains("LX") {
        println!("LX segment found");
        let lx_segments = get_lx(get_segment_contents("LX", &contents));
        println!("{:?}", lx_segments);
        println!("LX segment parsed");
        println!("\n");
    }

    if contents.contains("PER") {
        println!("PER segment found");
        let per_segments = get_per(get_segment_contents("PER", &contents));
        println!("{:?}", per_segments);
        println!("PER segment parsed");
        println!("\n");
    
    }

    if contents.contains("DTM") {
        let dtm_count= contents.matches("DTM").count();
        println!("Number of DTM segments: {}", dtm_count);

        let mut next_segment =  &contents[contents.find("DTM").unwrap()..];
        let mut dtm_vec = Vec::new();

        for _ in 0..dtm_count {
            let dtm_start = next_segment;
            let dtm_end = dtm_start.find("~").unwrap();
            let dtm_content = &dtm_start[4..dtm_end];
            let dtm_segments = get_dtm(dtm_content);
            dtm_vec.push(dtm_segments);
            next_segment = &dtm_start[dtm_end+1..]
        }
        println!("{:?}", dtm_vec);
        println!("DTM segment parsed");
        println!("\n");
    }

}