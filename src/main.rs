use std::fs::File;
use std::io::Read;

mod edi835;
use crate::edi835::segments::*;


#[allow(unused_variables)]
fn main() {
    // Open File and read content
    let mut file = File::open("./src/edi835-1.edi").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // TODO: create structs for these segments
    // find the first occurrence of "ISA" in the contents of the file and extract the content between "ISA" and "~"
    if contents.contains("ISA") {
        println!("ISA segment found");
        let isa_index = contents.find("ISA").unwrap();
        let isa_start = &contents[isa_index..];
        let isa_end = isa_start.find("~").unwrap();
        let isa_content = &isa_start[4..isa_end];
        let isa_segments = get_isa(isa_content);
        println!("{:?}", isa_segments);
        println!("ISA segment parsed");
        println!("\n");
    }
    
    // find how many gs segments are in the file
    if contents.contains("GS") {
        println!("GS segment found");
        // find the first occurrence of "GS" in the contents of the file and extract the content between "GS" and "~"
        let gs_index = contents.find("GS").unwrap();
        let gs_start = &contents[gs_index..];
        let gs_end = gs_start.find("~").unwrap();
        let gs_content = &gs_start[3..gs_end];
        let gs_segments = get_gs(gs_content);
        println!("{:?}", gs_segments);
        println!("GS segment parsed");
        println!("\n");
    }
    
    if contents.contains("ST") {
        println!("ST segment found");
        // find the first occurrence of "ST" in the contents of the file and extract the content between "ST" and "~"
        let st_index = contents.find("ST").unwrap();
        let st_start = &contents[st_index..];
        let st_end = st_start.find("~").unwrap();
        let st_content = &st_start[3..st_end];
        let st_segments = get_st(st_content);
        println!("{:?}", st_segments);
        println!("ST segment parsed");
        println!("\n");
    }

    if contents.contains("BPR") {
        println!("BPR segment found");
        // find the first occurrence of "BPR" in the contents of the file and extract the content between "BPR" and "~"
        let bpr_index = contents.find("BPR").unwrap();
        let bpr_start = &contents[bpr_index..];
        let bpr_end = bpr_start.find("~").unwrap();
        let bpr_content = &bpr_start[4..bpr_end];
        let bpr_segments = get_bpr(bpr_content);
        println!("{:?}", bpr_segments);
        println!("BPR segment parsed");
        println!("\n");
    }

    if contents.contains("TRN") {
        println!("TRN segment found");
        // find the first occurrence of "TRN" in the contents of the file and extract the content between "TRN" and "~"
        let trn_index = contents.find("TRN").unwrap();
        let trn_start = &contents[trn_index..];
        let trn_end = trn_start.find("~").unwrap();
        let trn_content = &trn_start[4..trn_end];
        let trn_segments = get_trn(trn_content);
        println!("{:?}", trn_segments);
        println!("TRN segment parsed");
        println!("\n");
    }

    if contents.contains("REF") {
        println!("REF segment found");
        // find the first occurrence of "REF" in the contents of the file and extract the content between "REF" and "~"
        let ref_index = contents.find("REF").unwrap();
        let ref_start = &contents[ref_index..];
        let ref_end = ref_start.find("~").unwrap();
        let ref_content = &ref_start[4..ref_end];
        let ref_segments = get_ref(ref_content);
        println!("{:?}", ref_segments);
        println!("REF segment parsed");
        println!("\n");
    }

    if contents.contains("NM1") {
        // find how many nm1 segments are in the file
        let nm1_count= contents.matches("NM1").count();
        println!("Number of NM1 segments: {}", nm1_count);

        let mut next_segment =  &contents[contents.find("NM1").unwrap()..];
        // let nm1arr = [];
        let mut nm1_vec = Vec::new();

        for n in 0..nm1_count {
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
        // find the first occurrence of "N1" in the contents of the file and extract the content between "N1" and "~"
        let n1_index = contents.find("N1").unwrap();
        let n1_start = &contents[n1_index..];
        let n1_end = n1_start.find("~").unwrap();
        let n1_content = &n1_start[3..n1_end];
        let n1_segments = get_n1(n1_content);
        println!("{:?}", n1_segments);
        println!("N1 segment parsed");
        println!("\n");
    } 

    if contents.contains("N3") {
        println!("N3 segment found");
        // find the first occurrence of "N3" in the contents of the file and extract the content between "N3" and "~"
        let n3_index = contents.find("N3").unwrap();
        let n3_start = &contents[n3_index..];
        let n3_end = n3_start.find("~").unwrap();
        let n3_content = &n3_start[3..n3_end];
        let n3_segments = get_n3(n3_content);
        println!("{:?}", n3_segments);
        println!("N3 segment parsed");
        println!("\n");
    }

    if contents.contains("N4") {
        println!("N4 segment found");
        // find the first occurrence of "N4" in the contents of the file and extract the content between "N4" and "~"
        let n4_index = contents.find("N4").unwrap();
        let n4_start = &contents[n4_index..];
        let n4_end = n4_start.find("~").unwrap();
        let n4_content = &n4_start[3..n4_end];
        let n4_segments = get_n4(n4_content);
        println!("{:?}", n4_segments);
        println!("N4 segment parsed");
        println!("\n");
    }

    if contents.contains("LX") {
        println!("LX segment found");
        // find the first occurrence of "LX" in the contents of the file and extract the content between "LX" and "~"
        let lx_index = contents.find("LX").unwrap();
        let lx_start = &contents[lx_index..];
        let lx_end = lx_start.find("~").unwrap();
        let lx_content = &lx_start[3..lx_end];
        let lx_segments = get_lx(lx_content);
        println!("{:?}", lx_segments);
        println!("LX segment parsed");
        println!("\n");
    }

    if contents.contains("PER") {
        println!("PER segment found");
        // find the first occurrence of "PER" in the contents of the file and extract the content between "PER" and "~"
        let per_index = contents.find("PER").unwrap();
        let per_start = &contents[per_index..];
        let per_end = per_start.find("~").unwrap();
        let per_content = &per_start[4..per_end];
        let per_segments = get_per(per_content);
        println!("{:?}", per_segments);
        println!("PER segment parsed");
        println!("\n");
    
    }


}