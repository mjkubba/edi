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

example of testing:
```bash
# Parse EDI to JSON
cargo run -- -f ./demo/edi835-1.edi -o ./demo/test835-new.json
```
```bash
# Generate EDI from JSON
cargo run -- -f ./demo/test835-new.json -o ./demo/test835-new.edi -w -j
```
```bash
# Compare files
diff ./demo/edi835-1.edi ./demo/test835-new.edi
```

you don't have to create new files for testing, it's better to run these commands one by one with less files

demo files are in the demo dir   
for 277 and 276 look in the dir 005010X212 in the name    
for 278 look in the dir 005010X217 and 008010X327 and 008010X328 and 008010X342 in the name   
for 834 look in the dir 005010X220 and 005010X307 and 005010X318 in the name   
for 835 look in the dir 005010X221 in the name   
for 837p look in the dir 005010X222 and 005010X291 and 005010X298 in the name   
for 837i look in the dir 005010X223 and 005010X292 and 005010X299 in the name   
for 837d look in the dir 005010X224 in the name   
for 999 look in the dir 005010X231  in the name   
for 270 and 271 look in the dir with 005010X279 in the name  
for 820 look in the dir with 005010X306 in the name

after running the tests make sure to update any tasks or status documentations based on results