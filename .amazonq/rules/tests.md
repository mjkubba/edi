## When creating tests make sure the tests are in the same file as the code, this is per rust standards

## Make sure to test the full application from time to time

## When testing the application:

### Test Methodology
- Parse EDI files to JSON and verify structure
- Generate EDI files from JSON and verify structure
- Compare original and generated EDI files
- Identify unprocessed segments and structural differences

### Test Environments
- Development environment with Rust toolchain
- Command-line interface for running tests
- File comparison tools for analyzing differences

after running the tests make sure to update any tasks or status documentations based on results