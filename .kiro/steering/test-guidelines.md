---
inclusion: always
---

# Testing Guidelines

## Test Structure
When creating tests make sure the tests are in the same file as the code, this is per rust standards.

## Full Application Testing
Make sure to test the full application from time to time.

## Test Methodology
When testing the application:

- Parse EDI files to JSON and verify structure
- Generate EDI files from JSON and verify structure
- Compare original and generated EDI files
- Identify unprocessed segments and structural differences

### Example Testing Commands
```bash
# Parse EDI to JSON
cargo run -- -f ./demo/edi835-1.edi -o ./demo/test835-new.json

# Generate EDI from JSON
cargo run -- -f ./demo/test835-new.json -o ./demo/test835-new.edi -w -j

# Compare files
diff ./demo/edi835-1.edi ./demo/test835-new.edi
```

You don't have to create new files for testing, it's better to run these commands one by one with less files.

## Demo File Locations
Demo files are in the demo dir. Look for files with these patterns:
- For 270 and 271: look for `005010X279` in the name
- For 276 and 277: look for `005010X212` in the name
- For 278: look for `005010X217`, `008010X327`, `008010X328`, `008010X342` in the name
- For 820: look for `005010X306` in the name
- For 834: look for `005010X220`, `005010X307`, `005010X318` in the name
- For 835: look for `005010X221` in the name
- For 837P: look for `005010X222`, `005010X291`, `005010X298` in the name
- For 837I: look for `005010X223`, `005010X292`, `005010X299` in the name
- For 837D: look for `005010X224` in the name
- For 999: look for `005010X231` in the name
