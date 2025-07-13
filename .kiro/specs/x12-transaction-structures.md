# X12 Transaction Set Structures

## 835 (Health Care Claim Payment/Advice) - ASC X12N/005010X221
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BPR - Financial Information
      TRN - Reassociation Trace Number
      DTM - Production Date
      Loop 1000A - Payer Identification
        N1 - Payer Identification
        N3 - Payer Address
        N4 - Payer City/State/ZIP
        REF - Additional Payer Identification
        PER - Payer Contact Information
      Loop 1000B - Payee Identification
        N1 - Payee Identification
        N3 - Payee Address
        N4 - Payee City/State/ZIP
        REF - Payee Additional Identification
        RDM - Remittance Delivery Method
      Loop 2000 - Header Number
        LX - Header Number
        TS3 - Provider Summary Information
        TS2 - Provider Supplemental Summary Information
        Loop 2100 - Claim Payment Information
          CLP - Claim Payment Information
          CAS - Claim Adjustment
          NM1 - Patient Name
          NM1 - Insured Name
          NM1 - Corrected Patient/Insured Name
          NM1 - Service Provider Name
          MIA - Inpatient Adjudication Information
          MOA - Outpatient Adjudication Information
          REF - Other Claim Related Identification
          DTM - Statement From or To Date
          PER - Claim Contact Information
          AMT - Claim Supplemental Information
          QTY - Claim Supplemental Information Quantity
          Loop 2110 - Service Payment Information
            SVC - Service Payment Information
            DTM - Service Date
            CAS - Service Adjustment
            REF - Service Identification
            AMT - Service Supplemental Amount
            QTY - Service Supplemental Quantity
            LQ - Health Care Remark Codes
      PLB - Provider Adjustment
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```

## 999 (Implementation Acknowledgment) - ASC X12C/005010X231
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      AK1 - Functional Group Response Header
      Loop 2000 - Transaction Set Response Header
        AK2 - Transaction Set Response Header
        Loop 2100 - Error Identification
          IK3 - Error Identification
          CTX - Context
          Loop 2110 - Implementation Data Element Note
            IK4 - Implementation Data Element Note
            CTX - Context
        IK5 - Transaction Set Response Trailer
      AK9 - Functional Group Response Trailer
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```

## 270/271 (Health Care Eligibility Benefit Inquiry and Response) - ASC X12N/005010X279
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BHT - Beginning of Hierarchical Transaction
      Loop 2000A - Information Source
        HL - Information Source Level
        NM1 - Information Source Name
        PER - Information Source Contact Information
        Loop 2100A - Payer Name
          NM1 - Payer Name
          Loop 2000B - Information Receiver
            HL - Information Receiver Level
            NM1 - Information Receiver Name
            Loop 2100B - Information Receiver Name
              NM1 - Information Receiver Name
              Loop 2000C - Subscriber
                HL - Subscriber Level
                TRN - Subscriber Trace Number
                Loop 2100C - Subscriber Name
                  NM1 - Subscriber Name
                  REF - Subscriber Additional Identification
                  N3 - Subscriber Address
                  N4 - Subscriber City/State/ZIP
                  DMG - Subscriber Demographic Information
                  INS - Subscriber Relationship
                  DTP - Subscriber Date
                  Loop 2110C - Subscriber Eligibility or Benefit Information
                    EB - Subscriber Eligibility or Benefit Information
                    HSD - Health Care Services Delivery
                    REF - Subscriber Additional Identification
                    DTP - Subscriber Eligibility/Benefit Date
                    AAA - Subscriber Request Validation
                    MSG - Message Text
                    Loop 2115C - Subscriber Eligibility or Benefit Additional Information
                      III - Subscriber Eligibility or Benefit Additional Information
                      Loop 2120C - Subscriber Benefit Related Entity
                        NM1 - Subscriber Benefit Related Entity Name
                        N3 - Subscriber Benefit Related Entity Address
                        N4 - Subscriber Benefit Related Entity City/State/ZIP
                        PER - Subscriber Benefit Related Entity Contact Information
                        PRV - Subscriber Benefit Related Provider Information
                Loop 2000D - Dependent
                  HL - Dependent Level
                  TRN - Dependent Trace Number
                  Loop 2100D - Dependent Name
                    NM1 - Dependent Name
                    REF - Dependent Additional Identification
                    N3 - Dependent Address
                    N4 - Dependent City/State/ZIP
                    DMG - Dependent Demographic Information
                    INS - Dependent Relationship
                    DTP - Dependent Date
                    Loop 2110D - Dependent Eligibility or Benefit Information
                      EB - Dependent Eligibility or Benefit Information
                      HSD - Health Care Services Delivery
                      REF - Dependent Additional Identification
                      DTP - Dependent Eligibility/Benefit Date
                      AAA - Dependent Request Validation
                      MSG - Message Text
                      Loop 2115D - Dependent Eligibility or Benefit Additional Information
                        III - Dependent Eligibility or Benefit Additional Information
                        Loop 2120D - Dependent Benefit Related Entity
                          NM1 - Dependent Benefit Related Entity Name
                          N3 - Dependent Benefit Related Entity Address
                          N4 - Dependent Benefit Related Entity City/State/ZIP
                          PER - Dependent Benefit Related Entity Contact Information
                          PRV - Dependent Benefit Related Provider Information
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```

## 276/277 (Health Care Claim Status Request and Response) - ASC X12N/005010X212
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BHT - Beginning of Hierarchical Transaction
      Loop 2000A - Information Source
        HL - Information Source Level
        NM1 - Information Source Name
        Loop 2100A - Payer Name
          NM1 - Payer Name
          PER - Payer Contact Information
        Loop 2000B - Information Receiver
          HL - Information Receiver Level
          NM1 - Information Receiver Name
          Loop 2100B - Information Receiver Name
            NM1 - Information Receiver Name
          Loop 2000C - Service Provider
            HL - Service Provider Level
            NM1 - Service Provider Name
            REF - Service Provider Additional Identification
            Loop 2100C - Service Provider Name
              NM1 - Service Provider Name
              REF - Service Provider Additional Identification
            Loop 2000D - Subscriber
              HL - Subscriber Level
              NM1 - Subscriber Name
              DMG - Subscriber Demographic Information
              Loop 2100D - Subscriber Name
                NM1 - Subscriber Name
                REF - Subscriber Additional Identification
              Loop 2200D - Claim Status Tracking Number
                TRN - Claim Status Tracking Number
                STC - Claim Status Information
                REF - Payer Claim Identification Number
                DTP - Claim Service Date
                Loop 2220D - Service Line Information
                  SVC - Service Line Information
                  STC - Service Line Status Information
                  REF - Service Line Item Identification
                  DTP - Service Line Date
              Loop 2000E - Dependent
                HL - Dependent Level
                NM1 - Dependent Name
                DMG - Dependent Demographic Information
                Loop 2100E - Dependent Name
                  NM1 - Dependent Name
                  REF - Dependent Additional Identification
                Loop 2200E - Claim Status Tracking Number
                  TRN - Claim Status Tracking Number
                  STC - Claim Status Information
                  REF - Payer Claim Identification Number
                  DTP - Claim Service Date
                  Loop 2220E - Service Line Information
                    SVC - Service Line Information
                    STC - Service Line Status Information
                    REF - Service Line Item Identification
                    DTP - Service Line Date
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```

## 837P (Health Care Claim: Professional) - ASC X12N/005010X222
```
ISA - Interchange Control Header
  GS - Functional Group Header
    ST - Transaction Set Header
      BHT - Beginning of Hierarchical Transaction
      Loop 2000A - Billing Provider Hierarchical Level
        HL - Billing Provider Hierarchical Level
        PRV - Billing Provider Specialty Information
        CUR - Foreign Currency Information
        Loop 2010AA - Billing Provider Name
          NM1 - Billing Provider Name
          N3 - Billing Provider Address
          N4 - Billing Provider City/State/ZIP
          REF - Billing Provider Tax Identification
          PER - Billing Provider Contact Information
        Loop 2010AB - Pay-to Address
          NM1 - Pay-to Provider Name
          N3 - Pay-to Provider Address
          N4 - Pay-to Provider City/State/ZIP
        Loop 2010AC - Pay-to Plan Name
          NM1 - Pay-to Plan Name
          N3 - Pay-to Plan Address
          N4 - Pay-to Plan City/State/ZIP
          REF - Pay-to Plan Tax Identification
        Loop 2000B - Subscriber Hierarchical Level
          HL - Subscriber Hierarchical Level
          SBR - Subscriber Information
          PAT - Patient Information
          Loop 2010BA - Subscriber Name
            NM1 - Subscriber Name
            N3 - Subscriber Address
            N4 - Subscriber City/State/ZIP
            DMG - Subscriber Demographic Information
            REF - Subscriber Secondary Identification
          Loop 2010BB - Payer Name
            NM1 - Payer Name
            N3 - Payer Address
            N4 - Payer City/State/ZIP
            REF - Payer Secondary Identification
          Loop 2300 - Claim Information
            CLM - Claim Information
            DTP - Date - Onset of Current Illness or Symptom
            DTP - Date - Initial Treatment Date
            DTP - Date - Last Seen Date
            DTP - Date - Acute Manifestation
            DTP - Date - Accident
            DTP - Date - Last Menstrual Period
            DTP - Date - Last X-ray Date
            DTP - Date - Hearing and Vision Prescription Date
            DTP - Date - Disability Dates
            DTP - Date - Last Worked
            DTP - Date - Authorized Return to Work
            DTP - Date - Admission
            DTP - Date - Discharge
            DTP - Date - Assumed and Relinquished Care Dates
            DTP - Date - Property and Casualty Date of First Contact
            PWK - Claim Supplemental Information
            CN1 - Contract Information
            AMT - Patient Amount Paid
            REF - Service Authorization Exception Code
            REF - Mandatory Medicare Crossover Indicator
            REF - Mammography Certification Number
            REF - Prior Authorization
            REF - Payer Claim Control Number
            REF - Clinical Laboratory Improvement Amendment Number
            REF - Referral Number
            REF - Prior Authorization Reference Number
            K3 - File Information
            NTE - Claim Note
            CR1 - Ambulance Transport Information
            CR2 - Spinal Manipulation Service Information
            CRC - Ambulance Certification
            CRC - Patient Condition Information: Vision
            CRC - Homebound Indicator
            CRC - EPSDT Referral
            HI - Health Care Diagnosis Code
            HI - Anesthesia Related Procedure
            HI - Condition Information
            HCP - Claim Pricing/Repricing Information
            Loop 2310A - Referring Provider Name
              NM1 - Referring Provider Name
              PRV - Referring Provider Specialty Information
              REF - Referring Provider Secondary Identification
            Loop 2310B - Rendering Provider Name
              NM1 - Rendering Provider Name
              PRV - Rendering Provider Specialty Information
              REF - Rendering Provider Secondary Identification
            Loop 2310C - Service Facility Location
              NM1 - Service Facility Location
              N3 - Service Facility Location Address
              N4 - Service Facility Location City/State/ZIP
              REF - Service Facility Location Secondary Identification
            Loop 2310D - Supervising Provider Name
              NM1 - Supervising Provider Name
              REF - Supervising Provider Secondary Identification
            Loop 2310E - Ambulance Pick-up Location
              NM1 - Ambulance Pick-up Location
              N3 - Ambulance Pick-up Location Address
              N4 - Ambulance Pick-up Location City/State/ZIP
            Loop 2310F - Ambulance Drop-off Location
              NM1 - Ambulance Drop-off Location
              N3 - Ambulance Drop-off Location Address
              N4 - Ambulance Drop-off Location City/State/ZIP
            Loop 2320 - Other Subscriber Information
              SBR - Other Subscriber Information
              CAS - Claim Level Adjustments
              AMT - Coordination of Benefits (COB) Payer Paid Amount
              AMT - Coordination of Benefits (COB) Total Non-Covered Amount
              AMT - Remaining Patient Liability
              OI - Other Insurance Coverage Information
              MOA - Outpatient Adjudication Information
              Loop 2330A - Other Subscriber Name
                NM1 - Other Subscriber Name
                N3 - Other Subscriber Address
                N4 - Other Subscriber City/State/ZIP
                REF - Other Subscriber Secondary Identification
              Loop 2330B - Other Payer Name
                NM1 - Other Payer Name
                N3 - Other Payer Address
                N4 - Other Payer City/State/ZIP
                DTP - Claim Check or Remittance Date
                REF - Other Payer Secondary Identifier
              Loop 2330C - Other Payer Referring Provider
                NM1 - Other Payer Referring Provider
                REF - Other Payer Referring Provider Secondary Identification
              Loop 2330D - Other Payer Rendering Provider
                NM1 - Other Payer Rendering Provider
                REF - Other Payer Rendering Provider Secondary Identification
              Loop 2330E - Other Payer Service Facility Location
                NM1 - Other Payer Service Facility Location
                REF - Other Payer Service Facility Location Secondary Identification
              Loop 2330F - Other Payer Supervising Provider
                NM1 - Other Payer Supervising Provider
                REF - Other Payer Supervising Provider Secondary Identification
              Loop 2330G - Other Payer Billing Provider
                NM1 - Other Payer Billing Provider
                REF - Other Payer Billing Provider Secondary Identification
            Loop 2400 - Service Line
              LX - Service Line
              SV1 - Professional Service
              SV5 - Durable Medical Equipment Service
              PWK - Line Supplemental Information
              CR1 - Ambulance Transport Information
              CR3 - Durable Medical Equipment Certification
              CRC - Ambulance Certification
              CRC - Hospice Employee Indicator
              DTP - Date - Service Date
              DTP - Date - Prescription Date
              DTP - Date - Certification Revision Date
              DTP - Date - Begin Therapy Date
              DTP - Date - Last Certification Date
              QTY - Ambulance Patient Count
              MEA - Test Result
              CN1 - Contract Information
              REF - Repriced Line Item Reference Number
              REF - Line Item Control Number
              REF - Mammography Certification Number
              REF - Clinical Laboratory Improvement Amendment Number
              REF - Referring Clinical Laboratory Improvement Amendment Number
              REF - Immunization Batch Number
              REF - Referral Number
              REF - Prior Authorization
              AMT - Sales Tax Amount
              AMT - Postage Claimed Amount
              K3 - File Information
              NTE - Line Note
              PS1 - Purchased Service Information
              HCP - Line Pricing/Repricing Information
              Loop 2410 - Drug Identification
                LIN - Drug Identification
                CTP - Drug Pricing
                REF - Prescription or Compound Drug Association Number
              Loop 2420A - Rendering Provider Name
                NM1 - Rendering Provider Name
                PRV - Rendering Provider Specialty Information
                REF - Rendering Provider Secondary Identification
              Loop 2420B - Purchased Service Provider Name
                NM1 - Purchased Service Provider Name
                REF - Purchased Service Provider Secondary Identification
              Loop 2420C - Service Facility Location
                NM1 - Service Facility Location
                N3 - Service Facility Location Address
                N4 - Service Facility Location City/State/ZIP
                REF - Service Facility Location Secondary Identification
              Loop 2420D - Supervising Provider Name
                NM1 - Supervising Provider Name
                REF - Supervising Provider Secondary Identification
              Loop 2420E - Ordering Provider Name
                NM1 - Ordering Provider Name
                N3 - Ordering Provider Address
                N4 - Ordering Provider City/State/ZIP
                REF - Ordering Provider Secondary Identification
                PER - Ordering Provider Contact Information
              Loop 2420F - Referring Provider Name
                NM1 - Referring Provider Name
                REF - Referring Provider Secondary Identification
              Loop 2420G - Ambulance Pick-up Location
                NM1 - Ambulance Pick-up Location
                N3 - Ambulance Pick-up Location Address
                N4 - Ambulance Pick-up Location City/State/ZIP
              Loop 2420H - Ambulance Drop-off Location
                NM1 - Ambulance Drop-off Location
                N3 - Ambulance Drop-off Location Address
                N4 - Ambulance Drop-off Location City/State/ZIP
    SE - Transaction Set Trailer
  GE - Functional Group Trailer
IEA - Interchange Control Trailer
```