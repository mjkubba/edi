use serde::{Deserialize, Serialize};

/// Loop2000B - Subscriber Hierarchical Level
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000b {
    /// Hierarchical Level
    pub hl: String,
    /// Subscriber Information
    pub sbr: String,
    /// Patient Information
    pub pat: Option<String>,
    /// Demographic Information
    pub dmg: Option<String>,
    /// Subscriber Name
    pub nm1_subscriber: Option<String>,
    /// Subscriber Address
    pub n3: Option<String>,
    /// Subscriber City, State, ZIP Code
    pub n4: Option<String>,
    /// Subscriber Additional Identification
    pub ref_subscriber: Vec<String>,
    /// Subscriber Date
    pub dtp: Vec<String>,
    /// Payer Name (NM1*PR)
    pub nm1_payer: Option<String>,
    /// Payer Address
    pub n3_payer: Option<String>,
    /// Payer City, State, ZIP Code
    pub n4_payer: Option<String>,
    /// Payer Contact (PER)
    pub per: Option<String>,
    /// Payer Additional Identification
    pub ref_payer: Vec<String>,
}

/// Write Loop2000B to EDI format
pub fn write_loop2000b(loop2000b: &Loop2000b) -> String {
    let mut result = String::new();

    // Write HL segment
    result.push_str(&loop2000b.hl);
    result.push_str("\n");

    // Write SBR segment
    result.push_str(&loop2000b.sbr);
    result.push_str("\n");

    // Write PAT segment if present
    if let Some(pat) = &loop2000b.pat {
        result.push_str(pat);
        result.push_str("\n");
    }

    // Write NM1 segment if present (Loop2010BA - Subscriber Name)
    if let Some(nm1) = &loop2000b.nm1_subscriber {
        result.push_str(nm1);
        result.push_str("\n");
    }

    // Write N3 segment if present
    if let Some(n3) = &loop2000b.n3 {
        result.push_str(n3);
        result.push_str("\n");
    }

    // Write N4 segment if present
    if let Some(n4) = &loop2000b.n4 {
        result.push_str(n4);
        result.push_str("\n");
    }

    // Write DMG segment if present
    if let Some(dmg) = &loop2000b.dmg {
        result.push_str(dmg);
        result.push_str("\n");
    }

    // Write REF segments
    for ref_segment in &loop2000b.ref_subscriber {
        result.push_str(ref_segment);
        result.push_str("\n");
    }

    // Write DTP segments
    for dtp in &loop2000b.dtp {
        result.push_str(dtp);
        result.push_str("\n");
    }

    // Write NM1*PR (Loop2010BB - Payer Name) if present
    if let Some(nm1_payer) = &loop2000b.nm1_payer {
        result.push_str(nm1_payer);
        result.push_str("\n");
    }

    // Write Payer N3 if present
    if let Some(n3_payer) = &loop2000b.n3_payer {
        result.push_str(n3_payer);
        result.push_str("\n");
    }

    // Write Payer N4 if present
    if let Some(n4_payer) = &loop2000b.n4_payer {
        result.push_str(n4_payer);
        result.push_str("\n");
    }

    // Write PER segment if present
    if let Some(per) = &loop2000b.per {
        result.push_str(per);
        result.push_str("\n");
    }

    // Write Payer REF segments
    for ref_segment in &loop2000b.ref_payer {
        result.push_str(ref_segment);
        result.push_str("\n");
    }

    result
}

/// Parse Loop2000B from EDI content
pub fn parse_loop2000b(content: &str) -> (Loop2000b, String) {
    let mut loop2000b = Loop2000b::default();
    let mut remaining_content = content.to_string();

    // Parse HL segment for subscriber
    if let Some(hl_pos) = remaining_content.find("HL*") {
        let hl_segment = &remaining_content[hl_pos..];
        if hl_segment.contains("*22*") {
            let hl_end = remaining_content[hl_pos..]
                .find('~')
                .unwrap_or(remaining_content.len())
                + hl_pos;
            loop2000b.hl = remaining_content[hl_pos..=hl_end].to_string();
            remaining_content = remaining_content[hl_end + 1..].to_string();
        } else {
            return (loop2000b, remaining_content);
        }
    }

    // Parse SBR segment
    if let Some(sbr_pos) = remaining_content.find("SBR*") {
        if !remaining_content[..sbr_pos].contains("HL*") {
            let sbr_end = remaining_content[sbr_pos..]
                .find('~')
                .unwrap_or(remaining_content.len())
                + sbr_pos;
            loop2000b.sbr = remaining_content[sbr_pos..=sbr_end].to_string();
            remaining_content = remaining_content[sbr_end + 1..].to_string();
        }
    }

    // Parse PAT segment if present (before any NM1)
    if let Some(pat_pos) = remaining_content.find("PAT*") {
        if !remaining_content[..pat_pos].contains("HL*")
            && !remaining_content[..pat_pos].contains("NM1*")
            && !remaining_content[..pat_pos].contains("CLM*")
        {
            let pat_end = remaining_content[pat_pos..]
                .find('~')
                .unwrap_or(remaining_content.len())
                + pat_pos;
            loop2000b.pat = Some(remaining_content[pat_pos..=pat_end].to_string());
            remaining_content = remaining_content[pat_end + 1..].to_string();
        }
    }

    // Parse NM1*IL (Subscriber Name - Loop2010BA)
    if let Some(nm1_pos) = remaining_content.find("NM1*IL*") {
        if !remaining_content[..nm1_pos].contains("HL*")
            && !remaining_content[..nm1_pos].contains("CLM*")
        {
            let nm1_end = remaining_content[nm1_pos..]
                .find('~')
                .unwrap_or(remaining_content.len())
                + nm1_pos;
            loop2000b.nm1_subscriber = Some(remaining_content[nm1_pos..=nm1_end].to_string());
            remaining_content = remaining_content[nm1_end + 1..].to_string();

            // Parse N3 after NM1*IL (before next NM1 or HL or CLM)
            if let Some(n3_pos) = remaining_content.find("N3*") {
                if !remaining_content[..n3_pos].contains("HL*")
                    && !remaining_content[..n3_pos].contains("NM1*")
                    && !remaining_content[..n3_pos].contains("CLM*")
                {
                    let n3_end = remaining_content[n3_pos..]
                        .find('~')
                        .unwrap_or(remaining_content.len())
                        + n3_pos;
                    loop2000b.n3 = Some(remaining_content[n3_pos..=n3_end].to_string());
                    remaining_content = remaining_content[n3_end + 1..].to_string();
                }
            }

            // Parse N4 after N3 (before next NM1 or HL or CLM)
            if let Some(n4_pos) = remaining_content.find("N4*") {
                if !remaining_content[..n4_pos].contains("HL*")
                    && !remaining_content[..n4_pos].contains("NM1*")
                    && !remaining_content[..n4_pos].contains("CLM*")
                {
                    let n4_end = remaining_content[n4_pos..]
                        .find('~')
                        .unwrap_or(remaining_content.len())
                        + n4_pos;
                    loop2000b.n4 = Some(remaining_content[n4_pos..=n4_end].to_string());
                    remaining_content = remaining_content[n4_end + 1..].to_string();
                }
            }

            // Parse DMG after N4 (before next NM1 or HL or CLM)
            if let Some(dmg_pos) = remaining_content.find("DMG*") {
                if !remaining_content[..dmg_pos].contains("HL*")
                    && !remaining_content[..dmg_pos].contains("NM1*")
                    && !remaining_content[..dmg_pos].contains("CLM*")
                {
                    let dmg_end = remaining_content[dmg_pos..]
                        .find('~')
                        .unwrap_or(remaining_content.len())
                        + dmg_pos;
                    loop2000b.dmg = Some(remaining_content[dmg_pos..=dmg_end].to_string());
                    remaining_content = remaining_content[dmg_end + 1..].to_string();
                }
            }

            // Parse REF segments for subscriber (before next NM1 or HL or CLM)
            while let Some(ref_pos) = remaining_content.find("REF*") {
                if remaining_content[..ref_pos].contains("HL*")
                    || remaining_content[..ref_pos].contains("NM1*")
                    || remaining_content[..ref_pos].contains("CLM*")
                    || remaining_content[..ref_pos].contains("LX*")
                {
                    break;
                }
                let ref_end = remaining_content[ref_pos..]
                    .find('~')
                    .unwrap_or(remaining_content.len())
                    + ref_pos;
                loop2000b
                    .ref_subscriber
                    .push(remaining_content[ref_pos..=ref_end].to_string());
                remaining_content = remaining_content[ref_end + 1..].to_string();
            }

            // Parse DTP segments for subscriber (before next NM1 or HL or CLM)
            while let Some(dtp_pos) = remaining_content.find("DTP*") {
                if remaining_content[..dtp_pos].contains("HL*")
                    || remaining_content[..dtp_pos].contains("NM1*")
                    || remaining_content[..dtp_pos].contains("CLM*")
                    || remaining_content[..dtp_pos].contains("LX*")
                {
                    break;
                }
                let dtp_end = remaining_content[dtp_pos..]
                    .find('~')
                    .unwrap_or(remaining_content.len())
                    + dtp_pos;
                loop2000b
                    .dtp
                    .push(remaining_content[dtp_pos..=dtp_end].to_string());
                remaining_content = remaining_content[dtp_end + 1..].to_string();
            }
        }
    }

    // Parse NM1*PR (Payer Name - Loop2010BB)
    if let Some(nm1_pos) = remaining_content.find("NM1*PR*") {
        if !remaining_content[..nm1_pos].contains("HL*")
            && !remaining_content[..nm1_pos].contains("CLM*")
        {
            let nm1_end = remaining_content[nm1_pos..]
                .find('~')
                .unwrap_or(remaining_content.len())
                + nm1_pos;
            loop2000b.nm1_payer = Some(remaining_content[nm1_pos..=nm1_end].to_string());
            remaining_content = remaining_content[nm1_end + 1..].to_string();

            // Parse payer N3
            if let Some(n3_pos) = remaining_content.find("N3*") {
                if !remaining_content[..n3_pos].contains("HL*")
                    && !remaining_content[..n3_pos].contains("NM1*")
                    && !remaining_content[..n3_pos].contains("CLM*")
                {
                    let n3_end = remaining_content[n3_pos..]
                        .find('~')
                        .unwrap_or(remaining_content.len())
                        + n3_pos;
                    loop2000b.n3_payer = Some(remaining_content[n3_pos..=n3_end].to_string());
                    remaining_content = remaining_content[n3_end + 1..].to_string();
                }
            }

            // Parse payer N4
            if let Some(n4_pos) = remaining_content.find("N4*") {
                if !remaining_content[..n4_pos].contains("HL*")
                    && !remaining_content[..n4_pos].contains("NM1*")
                    && !remaining_content[..n4_pos].contains("CLM*")
                {
                    let n4_end = remaining_content[n4_pos..]
                        .find('~')
                        .unwrap_or(remaining_content.len())
                        + n4_pos;
                    loop2000b.n4_payer = Some(remaining_content[n4_pos..=n4_end].to_string());
                    remaining_content = remaining_content[n4_end + 1..].to_string();
                }
            }

            // Parse payer PER
            if let Some(per_pos) = remaining_content.find("PER*") {
                if !remaining_content[..per_pos].contains("HL*")
                    && !remaining_content[..per_pos].contains("NM1*")
                    && !remaining_content[..per_pos].contains("CLM*")
                {
                    let per_end = remaining_content[per_pos..]
                        .find('~')
                        .unwrap_or(remaining_content.len())
                        + per_pos;
                    loop2000b.per = Some(remaining_content[per_pos..=per_end].to_string());
                    remaining_content = remaining_content[per_end + 1..].to_string();
                }
            }

            // Parse payer REF segments
            while let Some(ref_pos) = remaining_content.find("REF*") {
                if remaining_content[..ref_pos].contains("HL*")
                    || remaining_content[..ref_pos].contains("NM1*")
                    || remaining_content[..ref_pos].contains("CLM*")
                    || remaining_content[..ref_pos].contains("LX*")
                {
                    break;
                }
                let ref_end = remaining_content[ref_pos..]
                    .find('~')
                    .unwrap_or(remaining_content.len())
                    + ref_pos;
                loop2000b
                    .ref_payer
                    .push(remaining_content[ref_pos..=ref_end].to_string());
                remaining_content = remaining_content[ref_end + 1..].to_string();
            }
        }
    }

    (loop2000b, remaining_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_loop2000b() {
        let content = "HL*2*1*22*1~SBR*P*18*******CI~NM1*IL*1*DOE*JOHN****MI*123456789A~N3*236 N MAIN ST~N4*MIAMI*FL*33413~DMG*D8*19700501*M~NM1*PR*2*ACME INSURANCE COMPANY*****PI*999996666~HL*3*2*23*0~";

        let (loop2000b, remaining) = parse_loop2000b(content);

        assert_eq!(loop2000b.hl, "HL*2*1*22*1~");
        assert_eq!(loop2000b.sbr, "SBR*P*18*******CI~");
        assert_eq!(
            loop2000b.nm1_subscriber,
            Some("NM1*IL*1*DOE*JOHN****MI*123456789A~".to_string())
        );
        assert_eq!(loop2000b.n3, Some("N3*236 N MAIN ST~".to_string()));
        assert_eq!(loop2000b.n4, Some("N4*MIAMI*FL*33413~".to_string()));
        assert_eq!(loop2000b.dmg, Some("DMG*D8*19700501*M~".to_string()));
        assert_eq!(
            loop2000b.nm1_payer,
            Some("NM1*PR*2*ACME INSURANCE COMPANY*****PI*999996666~".to_string())
        );
        assert!(remaining.contains("HL*3*2*23*0~"));
    }

    #[test]
    fn test_parse_loop2000b_no_patient() {
        let content = "HL*2*1*22*0~SBR*P*18*******MC~NM1*IL*1*DOE*JOHN****MI*123456789A~N3*123 MAIN ST~N4*ANYTOWN*PA*17111~REF*SY*123456789~DTP*307*D8*20230101~NM1*PR*2*MEDICARE*****PI*00435~CLM*12345*100~";

        let (loop2000b, remaining) = parse_loop2000b(content);

        assert_eq!(loop2000b.hl, "HL*2*1*22*0~");
        assert_eq!(loop2000b.sbr, "SBR*P*18*******MC~");
        assert_eq!(
            loop2000b.nm1_subscriber,
            Some("NM1*IL*1*DOE*JOHN****MI*123456789A~".to_string())
        );
        assert_eq!(loop2000b.n3, Some("N3*123 MAIN ST~".to_string()));
        assert_eq!(loop2000b.n4, Some("N4*ANYTOWN*PA*17111~".to_string()));
        assert_eq!(
            loop2000b.ref_subscriber,
            vec!["REF*SY*123456789~".to_string()]
        );
        assert_eq!(loop2000b.dtp, vec!["DTP*307*D8*20230101~".to_string()]);
        assert_eq!(
            loop2000b.nm1_payer,
            Some("NM1*PR*2*MEDICARE*****PI*00435~".to_string())
        );
        assert!(remaining.contains("CLM*12345*100~"));
    }

    #[test]
    fn test_write_loop2000b() {
        let mut loop2000b = Loop2000b::default();
        loop2000b.hl = "HL*2*1*22*1~".to_string();
        loop2000b.sbr = "SBR*P*18*******CI~".to_string();
        loop2000b.nm1_subscriber = Some("NM1*IL*1*DOE*JOHN****MI*123456789A~".to_string());
        loop2000b.n3 = Some("N3*123 MAIN ST~".to_string());
        loop2000b.n4 = Some("N4*ANYTOWN*PA*17111~".to_string());
        loop2000b.dmg = Some("DMG*D8*19700501*M~".to_string());
        loop2000b.nm1_payer = Some("NM1*PR*2*MEDICARE*****PI*00435~".to_string());

        let result = write_loop2000b(&loop2000b);

        assert!(result.contains("HL*2*1*22*1~\n"));
        assert!(result.contains("SBR*P*18*******CI~\n"));
        assert!(result.contains("NM1*IL*1*DOE*JOHN****MI*123456789A~\n"));
        assert!(result.contains("N3*123 MAIN ST~\n"));
        assert!(result.contains("N4*ANYTOWN*PA*17111~\n"));
        assert!(result.contains("DMG*D8*19700501*M~\n"));
        assert!(result.contains("NM1*PR*2*MEDICARE*****PI*00435~\n"));
    }
}
