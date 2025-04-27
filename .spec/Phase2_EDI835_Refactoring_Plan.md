# Phase 2 EDI835 Refactoring Plan

## Overview

This document outlines the plan for refactoring the EDI835 implementation to be consistent with the new transaction sets while maintaining backward compatibility. The refactoring will be done in phases to minimize disruption to existing functionality.

## Current State

The EDI835 implementation has the following characteristics:

1. **Function Naming**:
   - The main function for generating EDI835 files is named `write_edi` instead of `write_835`
   - This inconsistency makes it difficult to provide a consistent API across all transaction sets

2. **Error Handling**:
   - The EDI835 functions return default values instead of using the Result type
   - This approach doesn't allow for proper error propagation and handling

3. **Return Types**:
   - The EDI835 controller returns different types compared to the new transaction sets
   - This inconsistency makes it difficult to provide a consistent API across all transaction sets

## Refactoring Goals

1. **Maintain Backward Compatibility**:
   - Ensure existing code that uses the EDI835 implementation continues to work
   - Avoid breaking changes to the public API

2. **Improve Consistency**:
   - Make the EDI835 implementation consistent with the new transaction sets
   - Use the same function naming conventions across all transaction sets
   - Use the same error handling patterns across all transaction sets

3. **Enhance Error Handling**:
   - Update the EDI835 implementation to use the Result type for error handling
   - Provide better error messages and diagnostics

## Phased Approach

### Phase 1: Function Naming

1. **Add Wrapper Function**:
   ```rust
   pub fn write_835(edi835: &Edi835) -> String {
       // Convert Edi835 to JSON
       let json = serde_json::to_string(edi835).unwrap();
       // Call the existing write_edi function
       write_edi(json)
   }
   ```

2. **Update Re-exports**:
   ```rust
   // In lib.rs
   pub use edi835::controller::{get_835, write_edi, write_835};
   ```

3. **Update Main Application**:
   ```rust
   // In main.rs
   let edi835 = get_835(contents.clone());
   let new_edi = write_835(&edi835);
   write_to_file(new_edi, args.output_file);
   ```

### Phase 2: Error Handling

1. **Add Result Type to New Functions**:
   ```rust
   pub fn write_835(edi835: &Edi835) -> EdiResult<String> {
       // Convert Edi835 to JSON
       let json = match serde_json::to_string(edi835) {
           Ok(json) => json,
           Err(e) => return Err(EdiError::IoError(e.into())),
       };
       // Call the existing write_edi function
       Ok(write_edi(json))
   }
   ```

2. **Update Main Application to Handle Errors**:
   ```rust
   // In main.rs
   let edi835 = get_835(contents.clone());
   match write_835(&edi835) {
       Ok(new_edi) => write_to_file(new_edi, args.output_file),
       Err(e) => warn!("Error generating 835 EDI: {:?}", e),
   }
   ```

### Phase 3: Update Existing Functions

1. **Update get_835 Function**:
   ```rust
   pub fn get_835(contents: String) -> EdiResult<Edi835> {
       // Existing implementation with error handling
       let interchange_header = match get_interchange_header(contents.clone()) {
           Ok((header, new_contents)) => {
               contents = new_contents;
               header
           },
           Err(e) => return Err(e),
       };
       // ... rest of the function with error handling
       Ok(edi835)
   }
   ```

2. **Update write_edi Function**:
   ```rust
   pub fn write_edi(contents: String) -> EdiResult<String> {
       let edi_json: Edi835 = match serde_json::from_str(&contents.clone()) {
           Ok(json) => json,
           Err(e) => return Err(EdiError::IoError(e.into())),
       };
       // ... rest of the function with error handling
       Ok(new_edi)
   }
   ```

### Phase 4: Update Helper Functions

1. **Update Segment Processing Functions**:
   ```rust
   pub fn get_interchange_header(contents: String) -> EdiResult<(InterchangeHeader, String)> {
       // ... implementation with error handling
   }
   ```

2. **Update Loop Processing Functions**:
   ```rust
   pub fn get_loop_2000s(contents: String) -> EdiResult<(Vec<Table2>, String)> {
       // ... implementation with error handling
   }
   ```

## Implementation Timeline

1. **Phase 1**: Immediate - Add wrapper function and update re-exports
2. **Phase 2**: Short-term - Add Result type to new functions
3. **Phase 3**: Medium-term - Update existing functions
4. **Phase 4**: Long-term - Update helper functions

## Testing Strategy

1. **Unit Tests**:
   - Add tests for the new functions
   - Update existing tests to handle the new error handling

2. **Integration Tests**:
   - Test the entire EDI835 processing pipeline
   - Ensure backward compatibility with existing code

3. **Error Handling Tests**:
   - Test error conditions to ensure proper error propagation
   - Verify that error messages are helpful and descriptive

## Documentation Updates

1. **API Documentation**:
   - Document the new functions and error handling patterns
   - Provide examples of how to use the API correctly

2. **Migration Guide**:
   - Create a guide for migrating from the old API to the new API
   - Explain the benefits of the new error handling approach

## Conclusion

This phased approach allows us to gradually refactor the EDI835 implementation to be consistent with the new transaction sets while maintaining backward compatibility. By focusing on function naming first, we can quickly provide a consistent API across all transaction sets, and then gradually improve the error handling and return types.
