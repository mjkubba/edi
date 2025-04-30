# Phase 3 Implementation Plan

## Overview

Phase 3 of the EDI Parser project focuses on implementing additional transaction sets, specifically the 276/277 (Health Care Claim Status Request and Response) and 837 (Health Care Claim) formats. This document outlines the implementation plan, architecture, and timeline for Phase 3.

## Goals and Objectives

- Implement Transaction Set 276 (Health Care Claim Status Request)
- Implement Transaction Set 277 (Health Care Claim Status Response)
- Implement Transaction Set 837 (Health Care Claim) with variants:
  - 837P (Professional)
  - 837I (Institutional)
  - 837D (Dental)
- Improve formatting with line breaks in generated output
- Enhance segment order logic to better match original files
- Clean up compiler warnings

## Implementation Plan for 276/277

### 1. Directory Structure

```
src/
├── edi276/                      # Health Care Claim Status Request
│   ├── controller/              # Main controller for 276
│   ├── loops/                   # Loop structures for 276
│   └── segments/                # Segment structures specific to 276
├── edi277/                      # Health Care Claim Status Response
│   ├── controller/              # Main controller for 277
│   ├── loops/                   # Loop structures for 277
│   └── segments/                # Segment structures specific to 277
```

### 2. Common Segments

Many segments will be shared between 276 and 277, so we can reuse existing segment definitions:

- BHT (Beginning of Hierarchical Transaction)
- HL (Hierarchical Level)
- TRN (Trace Number)
- REF (Reference Identification)
- DMG (Demographic Information)
- DTP (Date or Time Period)
- NM1 (Individual or Organizational Name)
- PER (Administrative Communications Contact)

For 277, we'll need to add:
- STC (Status Information)
- AAA (Request Validation)
- QTY (Quantity)
- AMT (Monetary Amount)

### 3. Loop Structures for 276

#### Loop2000A (Information Source)
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000A {
    pub hl: HL,
    pub nm1: NM1,
    pub loop2100a: Vec<Loop2100A>,
}
```

#### Loop2000B (Information Receiver)
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000B {
    pub hl: HL,
    pub nm1: NM1,
    pub loop2100b: Vec<Loop2100B>,
}
```

#### Loop2000C (Service Provider)
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000C {
    pub hl: HL,
    pub nm1: NM1,
    pub trn: TRN,
    pub ref_segments: Vec<REF>,
    pub loop2100c: Vec<Loop2100C>,
    pub loop2200c: Vec<Loop2200C>,
}
```

#### Loop2000D (Subscriber)
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000D {
    pub hl: HL,
    pub nm1: NM1,
    pub trn: TRN,
    pub ref_segments: Vec<REF>,
    pub loop2100d: Vec<Loop2100D>,
    pub loop2200d: Vec<Loop2200D>,
}
```

#### Loop2000E (Dependent)
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000E {
    pub hl: HL,
    pub nm1: NM1,
    pub trn: TRN,
    pub ref_segments: Vec<REF>,
    pub loop2100e: Vec<Loop2100E>,
    pub loop2200e: Vec<Loop2200E>,
}
```

### 4. Controller for 276

```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi276 {
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2000a: Loop2000A,
    pub loop2000b: Vec<Loop2000B>,
    pub se_segments: SE,
    pub interchange_trailer: InterchangeTrailer,
}

pub fn get_276(contents: String) -> EdiResult<Edi276> {
    // Implementation details
}

pub fn write_276(edi276: &Edi276) -> String {
    // Implementation details
}
```

### 5. Loop Structures for 277

#### Loop2000A (Information Source)
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000A {
    pub hl: HL,
    pub nm1: NM1,
    pub loop2100a: Vec<Loop2100A>,
}
```

#### Loop2000B (Information Receiver)
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000B {
    pub hl: HL,
    pub nm1: NM1,
    pub loop2100b: Vec<Loop2100B>,
}
```

#### Loop2000C (Service Provider)
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000C {
    pub hl: HL,
    pub nm1: NM1,
    pub trn: TRN,
    pub ref_segments: Vec<REF>,
    pub loop2100c: Vec<Loop2100C>,
    pub loop2200c: Vec<Loop2200C>,
}
```

#### Loop2000D (Subscriber)
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000D {
    pub hl: HL,
    pub nm1: NM1,
    pub trn: TRN,
    pub ref_segments: Vec<REF>,
    pub loop2100d: Vec<Loop2100D>,
    pub loop2200d: Vec<Loop2200D>,
}
```

#### Loop2000E (Dependent)
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000E {
    pub hl: HL,
    pub nm1: NM1,
    pub trn: TRN,
    pub ref_segments: Vec<REF>,
    pub loop2100e: Vec<Loop2100E>,
    pub loop2200e: Vec<Loop2200E>,
}
```

#### Loop2200C/D/E (Claim Status Tracking)
```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2200C {
    pub trn: TRN,
    pub stc_segments: Vec<STC>,
    pub ref_segments: Vec<REF>,
    pub dtp_segments: Vec<DTP>,
    pub loop2220c: Vec<Loop2220C>,
}
```

### 6. Controller for 277

```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi277 {
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2000a: Loop2000A,
    pub loop2000b: Vec<Loop2000B>,
    pub se_segments: SE,
    pub interchange_trailer: InterchangeTrailer,
}

pub fn get_277(contents: String) -> EdiResult<Edi277> {
    // Implementation details
}

pub fn write_277(edi277: &Edi277) -> String {
    // Implementation details
}
```

### 7. New Segment: STC (Status Information)

```rust
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct STC {
    pub stc01_health_care_claim_status: String,
    pub stc02_status_information_effective_date: String,
    pub stc03_action_code: String,
    pub stc04_monetary_amount: String,
    pub stc05_monetary_amount: String,
    pub stc06_date: String,
    pub stc07_payment_method_code: String,
    pub stc08_date: String,
    pub stc09_check_number: String,
    pub stc10_health_care_claim_status: String,
    pub stc11_health_care_claim_status: String,
    pub stc12_free_form_message_text: String,
}

pub fn get_stc(stc_content: String) -> STC {
    // Implementation details
}

pub fn write_stc(stc: &STC) -> String {
    // Implementation details
}
```

### 8. Update Main Module

```rust
// Add support for 276/277 formats
match format {
    "835" => {
        // Existing 835 code
    },
    "999" => {
        // Existing 999 code
    },
    "270" => {
        // Existing 270 code
    },
    "271" => {
        // Existing 271 code
    },
    "276" => {
        if write_mode {
            let edi276: Edi276 = serde_json::from_str(&contents).unwrap();
            let new_edi = write_276(&edi276);
            write_to_file(output_file, new_edi)?;
        } else {
            let edi276 = get_276(contents)?;
            let json = serde_json::to_string_pretty(&edi276)?;
            write_to_file(output_file, json)?;
        }
    },
    "277" => {
        if write_mode {
            let edi277: Edi277 = serde_json::from_str(&contents).unwrap();
            let new_edi = write_277(&edi277);
            write_to_file(output_file, new_edi)?;
        } else {
            let edi277 = get_277(contents)?;
            let json = serde_json::to_string_pretty(&edi277)?;
            write_to_file(output_file, json)?;
        }
    },
    // Other formats
}
```

## Implementation Plan for 837

### 1. Directory Structure

```
src/
├── edi837/                      # Health Care Claim (Common)
│   ├── common/                  # Common structures for all 837 variants
│   │   ├── segments/            # Common segment definitions
│   │   └── loops/               # Common loop definitions
├── edi837p/                     # 837 Professional
│   ├── controller/              # Main controller for 837P
│   ├── loops/                   # Loop structures specific to 837P
│   └── segments/                # Segment structures specific to 837P
├── edi837i/                     # 837 Institutional
│   ├── controller/              # Main controller for 837I
│   ├── loops/                   # Loop structures specific to 837I
│   └── segments/                # Segment structures specific to 837I
├── edi837d/                     # 837 Dental
│   ├── controller/              # Main controller for 837D
│   ├── loops/                   # Loop structures specific to 837D
│   └── segments/                # Segment structures specific to 837D
```

### 2. Common Structures

The 837 transaction set has many common structures across its variants. We'll implement these in the common directory:

```rust
// src/edi837/common/loops/loop_2000a.rs
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000A {
    pub hl: HL,
    pub nm1: NM1,
    pub per: PER,
    pub loop2010a: Vec<Loop2010A>,
}

// src/edi837/common/loops/loop_2000b.rs
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000B {
    pub hl: HL,
    pub nm1: NM1,
    pub loop2010b: Vec<Loop2010B>,
}

// src/edi837/common/loops/loop_2000c.rs
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2000C {
    pub hl: HL,
    pub pat: PAT,
    pub nm1: NM1,
    pub loop2010c: Vec<Loop2010C>,
}
```

### 3. Variant-Specific Structures

Each variant (837P, 837I, 837D) will have its own specific structures:

```rust
// src/edi837p/loops/loop_2300.rs
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2300 {
    pub clm: CLM,
    pub dtp_segments: Vec<DTP>,
    pub ref_segments: Vec<REF>,
    pub loop2310: Vec<Loop2310>,
    pub loop2320: Vec<Loop2320>,
    pub loop2400: Vec<Loop2400>,
}

// src/edi837i/loops/loop_2300.rs
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2300 {
    pub clm: CLM,
    pub dtp_segments: Vec<DTP>,
    pub ref_segments: Vec<REF>,
    pub k3_segments: Vec<K3>,
    pub hi_segments: Vec<HI>,
    pub loop2310: Vec<Loop2310>,
    pub loop2320: Vec<Loop2320>,
    pub loop2400: Vec<Loop2400>,
}

// src/edi837d/loops/loop_2300.rs
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Loop2300 {
    pub clm: CLM,
    pub dtp_segments: Vec<DTP>,
    pub ref_segments: Vec<REF>,
    pub loop2310: Vec<Loop2310>,
    pub loop2320: Vec<Loop2320>,
    pub loop2400: Vec<Loop2400>,
}
```

### 4. Controllers for 837 Variants

```rust
// src/edi837p/controller/mod.rs
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Edi837P {
    pub interchange_header: InterchangeHeader,
    pub table1: Table1,
    pub loop2000a: Loop2000A,
    pub loop2000b: Vec<Loop2000B>,
    pub se_segments: SE,
    pub interchange_trailer: InterchangeTrailer,
}

// Similar structures for Edi837I and Edi837D
```

## Formatting Improvements

### 1. Line Breaks in Generated Output

```rust
pub fn write_with_line_breaks(edi_content: String, add_line_breaks: bool) -> String {
    if add_line_breaks {
        edi_content.replace("~", "~\n")
    } else {
        edi_content
    }
}

// Update write functions to use this helper
pub fn write_276(edi276: &Edi276, add_line_breaks: bool) -> String {
    let mut new_edi = String::new();
    
    // Add segments
    
    // Add line breaks if requested
    write_with_line_breaks(new_edi, add_line_breaks)
}
```

### 2. Segment Order Logic

```rust
pub struct SegmentOrderConfig {
    pub transaction_set: String,
    pub segment_order: Vec<String>,
}

// Implement a function to sort segments based on configuration
pub fn sort_segments(segments: Vec<String>, config: &SegmentOrderConfig) -> Vec<String> {
    let mut sorted_segments = segments.clone();
    sorted_segments.sort_by(|a, b| {
        let a_id = a.split('*').next().unwrap_or("");
        let b_id = b.split('*').next().unwrap_or("");
        
        let a_pos = config.segment_order.iter().position(|s| s == a_id).unwrap_or(usize::MAX);
        let b_pos = config.segment_order.iter().position(|s| s == b_id).unwrap_or(usize::MAX);
        
        a_pos.cmp(&b_pos)
    });
    
    sorted_segments
}
```

## Timeline and Milestones

### Week 1-2: Fix Remaining Issues
- Improve formatting with line breaks
- Enhance segment order logic
- Clean up compiler warnings

### Week 3-4: Implement 276/277
- Create directory structure
- Implement segment structures
- Implement loop structures
- Create controllers
- Test with sample files

### Week 5-8: Implement 837
- Create common structures
- Implement 837P variant
- Implement 837I variant
- Implement 837D variant
- Test with sample files

### Week 9: Final Testing and Documentation
- Comprehensive testing across all transaction sets
- Update documentation
- Clean up code and address warnings

## Conclusion

This implementation plan provides a structured approach to implementing the 276/277 and 837 transaction sets in Phase 3 of the EDI Parser project. By following this plan, we can ensure a consistent implementation approach across all transaction sets and maintain the high quality of the codebase.
