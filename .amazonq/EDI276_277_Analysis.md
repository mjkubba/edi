# EDI276/277 Analysis

## Overview

This document provides an analysis of the EDI276 (Health Care Claim Status Request) and EDI277 (Health Care Claim Status Response) formats based on the sample files in the demo/212 directory.

## EDI276 Structure

### Header Segments
- ISA: Interchange Control Header
- GS: Functional Group Header
- ST: Transaction Set Header
- BHT: Beginning of Hierarchical Transaction

### Hierarchical Structure
The EDI276 uses a hierarchical structure with HL segments to define the relationships between different entities:

1. **HL*1 (Information Source)**: Insurance company (Level 20)
   - NM1*PR: Payer Name

2. **HL*2 (Information Receiver)**: Service provider organization (Level 21)
   - NM1*41: Clearinghouse Name

3. **HL*3 (Service Provider)**: Hospital (Level 19)
   - NM1*1P: Provider Name

4. **HL*4 (Subscriber)**: Patient information (Level 22)
   - DMG: Demographic Information
   - NM1*IL: Subscriber Name
   - TRN: Trace Number
   - REF: Reference Information
   - AMT: Monetary Amount
   - DTP: Date or Time Period

5. **HL*5 (Subscriber)**: Another patient (Level 22)
   - Similar structure to HL*4

6. **HL*6 (Service Provider)**: Physician group (Level 19)
   - NM1*1P: Provider Name

7. **HL*7 (Subscriber)**: Another patient (Level 22)
   - NM1*IL: Subscriber Name

8. **HL*8 (Dependent)**: Dependent information (Level 23)
   - DMG: Demographic Information
   - NM1*QC: Dependent Name
   - TRN: Trace Number
   - REF: Reference Information
   - SVC: Service Information
   - DTP: Date or Time Period

### Trailer Segments
- SE: Transaction Set Trailer
- GE: Functional Group Trailer
- IEA: Interchange Control Trailer

## EDI277 Structure

### Header Segments
- ISA: Interchange Control Header
- GS: Functional Group Header
- ST: Transaction Set Header
- BHT: Beginning of Hierarchical Transaction

### Hierarchical Structure
The EDI277 follows a similar hierarchical structure to EDI276, but includes status information:

1. **HL*1 (Information Source)**: Insurance company (Level 20)
   - NM1*PR: Payer Name

2. **HL*2 (Information Receiver)**: Service provider organization (Level 21)
   - NM1*41: Clearinghouse Name

3. **HL*3 (Service Provider)**: Hospital (Level 19)
   - NM1*1P: Provider Name

4. **HL*4 (Subscriber)**: Patient information (Level 22)
   - NM1*IL: Subscriber Name
   - TRN: Trace Number
   - STC: Status Information
   - REF: Reference Information
   - DTP: Date or Time Period

5. **HL*5 (Subscriber)**: Another patient (Level 22)
   - Similar structure to HL*4

6. **HL*6 (Service Provider)**: Physician group (Level 19)
   - NM1*1P: Provider Name

7. **HL*7 (Subscriber)**: Another patient (Level 22)
   - NM1*IL: Subscriber Name

8. **HL*8 (Dependent)**: Dependent information (Level 23)
   - NM1*QC: Dependent Name
   - TRN: Trace Number
   - STC: Status Information
   - REF: Reference Information
   - SVC: Service Information
   - STC: Status Information (service-level)
   - DTP: Date or Time Period

### Trailer Segments
- SE: Transaction Set Trailer
- GE: Functional Group Trailer
- IEA: Interchange Control Trailer

## Key Differences Between EDI276 and EDI277

1. **Transaction Set Identifier**:
   - EDI276: ST*276
   - EDI277: ST*277

2. **BHT Segment**:
   - EDI276: BHT*0010*13 (13 = Request)
   - EDI277: BHT*0010*08 (08 = Response)

3. **STC Segment**:
   - EDI276: Does not contain STC segments
   - EDI277: Contains STC segments with status information

4. **TRN Segment**:
   - EDI276: TRN*1 (1 = Sender Reference Number)
   - EDI277: TRN*2 (2 = Receiver Reference Number)

## Implementation Considerations

1. **Hierarchical Structure**:
   - The HL segments define the hierarchical structure of the transaction
   - Each HL segment has a hierarchical ID, parent ID, level code, and child code
   - The level codes define the entity type (20=Information Source, 21=Information Receiver, 19=Service Provider, 22=Subscriber, 23=Dependent)

2. **Status Information**:
   - The STC segment in EDI277 provides status information about the claim
   - Format: STC*[Status Category/Code]:[Status Code]:[Entity Identifier Code]*[Date]*[Action Code]*[Monetary Amount]*[Monetary Amount]

3. **Reference Information**:
   - The REF segment provides reference information
   - Format: REF*[Reference Identification Qualifier]*[Reference Identification]

4. **Date Information**:
   - The DTP segment provides date information
   - Format: DTP*[Date/Time Qualifier]*[Date/Time Format Qualifier]*[Date]

5. **Service Information**:
   - The SVC segment provides service information
   - Format: SVC*[Service ID Qualifier]:[Procedure Code]*[Charge Amount]*[Payment Amount]

## Conclusion

The EDI276 and EDI277 transaction sets follow a hierarchical structure that represents the relationships between different healthcare entities. The EDI276 is used to request claim status information, while the EDI277 is used to provide claim status information in response. The key difference is the presence of STC segments in EDI277 that provide status information about the claims.
