# Testing Guidelines

## Test Structure

When creating tests, make sure the tests are in the same file as the code - this follows Rust standards and keeps related code together.

## Application Testing

Make sure to test the full application from time to time to ensure end-to-end functionality works correctly.

## Test Methodology

When testing the application, follow this systematic approach:

1. **Parse EDI files to JSON** and verify structure
2. **Generate EDI files from JSON** and verify structure  
3. **Compare original and generated EDI files**
4. **Identify unprocessed segments** and structural differences

### Test Commands

Use these command patterns for testing:

```bash
# Parse EDI to JSON
cargo run -- -f ./demo/edi835-1.edi -o ./demo/test835-new.json

# Generate EDI from JSON
cargo run -- -f ./demo/test835-new.json -o ./demo/test835-new.edi -w -j

# Compare files
diff ./demo/edi835-1.edi ./demo/test835-new.edi
```

**Note:** You don't have to create new files for testing - it's better to run these commands one by one with existing demo files.

## Demo File Locations

Demo files are organized in the `demo/` directory by transaction set. Use these mappings to find test files:

- **EDI 276/277**: Look in directories with `005010X212` in the name
- **EDI 278**: Look in directories with `005010X217`, `008010X327`, `008010X328`, or `008010X342` in the name
- **EDI 834**: Look in directories with `005010X220`, `005010X307`, or `005010X318` in the name
- **EDI 835**: Look in directories with `005010X221` in the name
- **EDI 837P**: Look in directories with `005010X222`, `005010X291`, or `005010X298` in the name
- **EDI 837I**: Look in directories with `005010X223`, `005010X292`, or `005010X299` in the name
- **EDI 837D**: Look in directories with `005010X224` in the name
- **EDI 999**: Look in directories with `005010X231` in the name
- **EDI 270/271**: Look in directories with `005010X279` in the name
- **EDI 820**: Look in directories with `005010X306` in the name

## Post-Testing Documentation

After running tests, make sure to update any tasks or status documentation based on the results. This helps track progress and identify areas that need attention.

## Test Environment Requirements

- Development environment with Rust toolchain
- Command-line interface for running tests
- File comparison tools for analyzing differences
- Access to demo files in the appropriate directories