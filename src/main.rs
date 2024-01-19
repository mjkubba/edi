use std::fs::File;
use std::io::Read;

mod edi835;
use edi835::{amt::*,bpr::*,clp::*,cur::*,dtm::*,ge::*,gs::*,iea::*,isa::*,lx::*,n1::*,n3::*,n4::*,nm1::*,per::*,r#ref::*,rdm::*,st::*,se::*,trn::*};

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

    // Control Segments

    if contents.contains("ISA") {
        println!("ISA segment found, ");
        let isa_segments = get_isa(get_segment_contents("ISA", &contents));
        println!("{:?}", isa_segments);
        println!("ISA segment parsed");
        println!("\n");
    }
    
    if contents.contains("GS") {
        print!("GS segment found, ");
        let gs_segments = get_gs(get_segment_contents("GS", &contents));
        println!("{:?}", gs_segments);
        println!("GS segment parsed");
    }

    // Table 1
    // Start of header loop (ST, BPR, TRN, CUR, REF, REF, DTM)
    
    if contents.contains("ST") {
        print!("ST segment found, ");
        let _st_segments = get_st(get_segment_contents("ST", &contents));
        println!("ST segment parsed");
    }

    if contents.contains("BPR") {
        print!("BPR segment found, ");
        let _bpr_segments = get_bpr(get_segment_contents("BPR", &contents));
        println!("BPR segment parsed");
    }

    if contents.contains("TRN") {
        print!("TRN segment found, ");
        let _trn_segments = get_trn(get_segment_contents("TRN", &contents));
        println!("TRN segment parsed");
    }
    
    if contents.contains("CUR") {
        print!("CUR segment found, ");
        let _cur_segments = get_cur(get_segment_contents("CUR", &contents));
        println!("CUR segment parsed");
    }

    if contents.contains("REF") {
        print!("REF segment found, ");
        let _ref_segments = get_ref(get_segment_contents("REF", &contents));
        println!("REF segment parsed");
    }

    if contents.contains("DTM") {
        let dtm_count= contents.matches("DTM").count();
        print!("Number of DTM segments: {}, ", dtm_count);

        let mut next_segment =  &contents[contents.find("DTM").unwrap()..];
        let mut _dtm_vec = Vec::new();

        for _ in 0..dtm_count {
            let dtm_start = next_segment;
            let dtm_end = dtm_start.find("~").unwrap();
            let dtm_content = &dtm_start[4..dtm_end];
            let dtm_segments = get_dtm(dtm_content);
            _dtm_vec.push(dtm_segments);
            next_segment = &dtm_start[dtm_end+1..]
        }
        println!("DTM segment parsed");
    }

    // Loop 1000A Payer Identification (N1, N3, N4, REF, PER, PER, PER)

    if contents.contains("N1") {
        print!("N1 segment found, ");
        let _n1_segments = get_n1(get_segment_contents("N1", &contents));
        println!("N1 segment parsed");
    } 

    if contents.contains("N3") {
        print!("N3 segment found, ");
        let _n3_segments = get_n3(get_segment_contents("N3", &contents));
        println!("N3 segment parsedm");
    }

    if contents.contains("N4") {
        print!("N4 segment found, ");
        let _n4_segments = get_n4(get_segment_contents("N4", &contents));
        println!("N4 segment parsed");
    }

    if contents.contains("PER") {
        print!("PER segment found, ");
        let _per_segments = get_per(get_segment_contents("PER", &contents));
        println!("PER segment parsed");
    }

    // Loop 1000B Payee Identification (N1, N3, N4, REF, RDM)

    if contents.contains("RDM") {
        print!("RDM segment found, ");
        let _rdm_segments = get_rdm(get_segment_contents("RDM", &contents));
        println!("RDM segment parsed");
    }

    // Table 2
    // Loop 2000 Header Number (LX, TS3, TS2)

    if contents.contains("LX") {
        print!("LX segment found, ");
        let _lx_segments = get_lx(get_segment_contents("LX", &contents));
        println!("LX segment parsed");
    }

    // Loop 2100 Claim Payment Information (CLP, CAS, 7x(NM1), MIA, MOA, REF, REF, 3x(DTM), PER, AMY, QTY)

    if contents.contains("CLP") {
        println!("CLP segment found");
        let clp_segments = get_clp(get_segment_contents("CLP", &contents));
        println!("{:?}", clp_segments);
        println!("CLP segment parsed");
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

    if contents.contains("AMT") {
        println!("AMT segment found");
        let amt_segments = get_amt(get_segment_contents("AMT", &contents));
        println!("{:?}", amt_segments);
        println!("AMT segment parsed");
        println!("\n");
    }

    // Loop 2110 Service Payment Information (SVC, DTM, CAS, 4x(REF), AMY, QTY, LQ)

    // Table 3


    if contents.contains("SE") {
        println!("SE segment found");
        let se_segments = get_se(get_segment_contents("SE", &contents));
        println!("{:?}", se_segments);
        println!("SE segment parsed");
        println!("\n");
    }

    // Control Segments

    if contents.contains("GE") {
        println!("GE segment found");
        let ge_segments = get_ge(get_segment_contents("GE", &contents));
        println!("{:?}", ge_segments);
        println!("GE segment parsed");
        println!("\n");
    
    }

    if contents.contains("IEA") {
        println!("IEA segment found");
        let iea_segments = get_iea(get_segment_contents("IEA", &contents));
        println!("{:?}", iea_segments);
        println!("IEA segment parsed");
        println!("\n");
    }

}