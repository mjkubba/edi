# Product Overview

This is an EDI (Electronic Data Interchange) parser and processor specifically designed for healthcare X12 formats in Rust. The application provides robust bidirectional conversion between EDI and JSON formats for multiple healthcare transaction sets.

## Supported Transaction Sets

- **EDI835** - Healthcare Claim Payment/Remittance Advice (Complete)
- **EDI999** - Implementation Acknowledgment (Complete)
- **EDI270/271** - Healthcare Eligibility Benefit Inquiry/Response (Complete)
- **EDI276/277** - Healthcare Claim Status Request/Response (Functional)
- **EDI278** - Healthcare Services Review (Functional)
- **EDI837P/I/D** - Healthcare Claims (Professional/Institutional/Dental) (Functional)
- **EDI820** - Health Insurance Exchange Related Payments (Partial)
- **EDI834** - Benefit Enrollment and Maintenance (Not Implemented)

## Core Functionality

- Parse EDI files to structured JSON format
- Generate EDI files from JSON data
- Validate EDI structure and content
- Support for complex loop structures and segment relationships
- Configuration-driven segment definitions
- Comprehensive error handling and logging

## Target Users

Healthcare organizations, clearinghouses, and developers working with X12 EDI formats who need reliable parsing and generation capabilities for healthcare data interchange.