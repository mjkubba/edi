# Implementation Plan: Web UI for EDI Parser

## Overview

Add a web-based UI to the existing EDI Parser by embedding an actix-web 4 HTTP server activated via `--web` flag. The implementation adds a `src/web/` module with REST API endpoints, an embedded HTML frontend, embedded demo files, and transaction set auto-detection — all while preserving full CLI backward compatibility. Tests use `proptest` for correctness properties and `actix-web::test` for integration testing, placed in the same files as the code per Rust conventions.

## Tasks

- [ ] 1. Add dependencies and create web module structure
  - [ ] 1.1 Add actix-web, actix-rt, and proptest dependencies to Cargo.toml
    - Add `actix-web = "4"`, `actix-rt = "2"`, and `tokio = { version = "1", features = ["full"] }` under `[dependencies]`
    - Add `proptest = "1"` and `actix-web = "4"` (for test utilities) under `[dev-dependencies]`
    - Run `cargo build` to verify dependency resolution
    - _Requirements: 1.1_

  - [ ] 1.2 Create `src/web/` module files with skeleton structure
    - Create `src/web/mod.rs` with module declarations for `server`, `handlers`, `detection`, `models`
    - Create `src/web/models.rs` with `ParseRequest`, `GenerateRequest`, `SuccessResponse`, `ErrorResponse`, `DemoFile`, `DemoListResponse`, `DemoContentResponse` structs using serde Serialize/Deserialize
    - Register `mod web;` in `src/main.rs`
    - _Requirements: 1.1, 2.1, 3.1_

- [ ] 2. Implement transaction set auto-detection
  - [ ] 2.1 Create `src/web/detection.rs` with EDI and JSON detection functions
    - Implement `detect_edi_transaction_set(contents: &str) -> Result<&'static str, String>` extracting the ST segment detection logic from `main.rs` patterns (ST*835*, ST*999*, ST*270*, ST*271*, ST*276*, ST*277*, ST*837* with 005010X222/X223/X224 variants, ST*278*, ST*820*, ST*834*)
    - Implement `detect_json_transaction_set(contents: &str) -> Result<&'static str, String>` extracting the JSON detection logic from `main.rs` patterns (transaction_set_id fields, st01 fields, 837 variant markers)
    - Both functions return `Err` with a message listing supported transaction sets when format is unrecognized
    - _Requirements: 4.1, 4.2, 4.4_

  - [ ]* 2.2 Write property test for EDI transaction set detection
    - **Property 1: EDI transaction set detection correctness**
    - Use `proptest` to generate EDI strings containing valid ST segments for each supported transaction set and verify `detect_edi_transaction_set` returns the correct type
    - Tests in `src/web/detection.rs` under `#[cfg(test)]` module
    - **Validates: Requirements 4.1**

  - [ ]* 2.3 Write property test for JSON transaction set detection
    - **Property 2: JSON transaction set detection correctness**
    - Use `proptest` to generate JSON strings containing valid transaction set identifier fields and verify `detect_json_transaction_set` returns the correct type
    - Tests in `src/web/detection.rs` under `#[cfg(test)]` module
    - **Validates: Requirements 4.2**

  - [ ]* 2.4 Write property test for unrecognized input rejection
    - **Property 5: Unrecognized input returns error with supported types**
    - Use `proptest` to generate random strings without valid ST segments or transaction set fields and verify both detection functions return `Err` containing the list of supported types
    - Tests in `src/web/detection.rs` under `#[cfg(test)]` module
    - **Validates: Requirements 2.5, 3.5, 4.4**

- [ ] 3. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 4. Implement embedded demo files and demo API handlers
  - [ ] 4.1 Add embedded demo files constant in `src/web/handlers.rs`
    - Define `DEMO_FILES: &[(&str, &str)]` array using `include_str!` for all 12 demo files from `demo/` directory (edi270, edi271, edi276, edi277, edi278, edi820, edi834, edi835, edi837D, edi837I, edi837P, edi999)
    - _Requirements: 5.1, 5.2_

  - [ ] 4.2 Implement `list_demos` and `get_demo` handler functions
    - `list_demos` returns `DemoListResponse` with all 12 filenames as JSON with HTTP 200
    - `get_demo` accepts `{filename}` path parameter, looks up in `DEMO_FILES`, returns `DemoContentResponse` with HTTP 200 or `ErrorResponse` with HTTP 404
    - _Requirements: 5.2, 5.4, 5.5_

  - [ ]* 4.3 Write property test for non-existent demo file 404
    - **Property 6: Non-existent demo file returns 404**
    - Use `proptest` to generate random filename strings that don't match any embedded demo file name and verify the lookup returns a not-found error
    - Tests in `src/web/handlers.rs` under `#[cfg(test)]` module
    - **Validates: Requirements 5.5**

  - [ ]* 4.4 Write unit tests for demo handlers
    - Test `list_demos` returns exactly 12 demo files
    - Test `get_demo` with a valid filename returns correct content
    - Test `get_demo` with invalid filename returns 404
    - Tests in `src/web/handlers.rs` under `#[cfg(test)]` module
    - _Requirements: 5.2, 5.4, 5.5_

- [ ] 5. Implement parse and generate API handlers
  - [ ] 5.1 Implement `parse_edi` handler in `src/web/handlers.rs`
    - Accept `ParseRequest` JSON body, validate non-empty content
    - Call `clean_contents` on input, then `detect_edi_transaction_set`
    - Route to the correct `get_*` library function based on detected type (get_835, get_999, get_270, get_271, get_276, get_277, get_837p, get_837i, get_837d, get_278 via Edi278::parse, get_820 via Edi820::parse, get_834 via Edi834::parse)
    - Serialize result with `serde_json::to_string_pretty` and return `SuccessResponse` with HTTP 200
    - Return `ErrorResponse` with HTTP 400 for empty input, unrecognized format, or library errors
    - _Requirements: 2.1, 2.2, 2.3, 2.5, 2.6, 4.1, 4.3_

  - [ ] 5.2 Implement `generate_edi` handler in `src/web/handlers.rs`
    - Accept `GenerateRequest` JSON body, validate non-empty content
    - Call `detect_json_transaction_set` on input
    - Route to the correct `write_*` library function based on detected type (write_835, write_999, write_270, write_271, write_276, write_277, write_837p, write_837i, write_837d, write_278, write_820, write_834)
    - Return `SuccessResponse` with HTTP 200
    - Return `ErrorResponse` with HTTP 400 for empty input, unrecognized format, malformed JSON, or library errors
    - _Requirements: 3.1, 3.2, 3.3, 3.5, 3.6, 4.2, 4.3_

  - [ ] 5.3 Implement `index` handler in `src/web/handlers.rs`
    - Return the embedded HTML content with `Content-Type: text/html` and HTTP 200
    - Use `include_str!("index.html")` to embed the HTML file at compile time
    - _Requirements: 1.2_

  - [ ]* 5.4 Write property test for parse API equivalence
    - **Property 3: Parse API equivalence with library**
    - For each embedded demo file, call `clean_contents` + detection + library function directly, then compare with the parse handler output to verify identical JSON
    - Tests in `src/web/handlers.rs` under `#[cfg(test)]` module
    - **Validates: Requirements 2.3**

  - [ ]* 5.5 Write property test for generate API equivalence
    - **Property 4: Generate API equivalence with library**
    - Parse each demo file to JSON first, then call the generate handler and compare with calling the `write_*` library function directly to verify identical EDI output
    - Tests in `src/web/handlers.rs` under `#[cfg(test)]` module
    - **Validates: Requirements 3.3**

  - [ ]* 5.6 Write unit tests for parse and generate handlers
    - Test empty input returns 400 for both parse and generate
    - Test each of the 12 demo files parses successfully via the handler
    - Test a valid JSON round-trip through generate for at least one transaction set
    - Test unrecognized EDI format returns 400 with supported types list
    - Tests in `src/web/handlers.rs` under `#[cfg(test)]` module
    - _Requirements: 2.2, 2.5, 2.6, 3.2, 3.5, 3.6_

- [ ] 6. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 7. Implement server setup and route configuration
  - [ ] 7.1 Implement `start_server` in `src/web/server.rs`
    - Create `WebConfig` struct with `port: u16` field
    - Implement `start_server(config: WebConfig) -> std::io::Result<()>` that creates an `actix_web::HttpServer`, configures routes (GET `/` → index, POST `/api/parse` → parse_edi, POST `/api/generate` → generate_edi, GET `/api/demos` → list_demos, GET `/api/demos/{filename}` → get_demo), binds to `0.0.0.0:{port}`, and runs the server
    - Log the URL where the Web_UI is accessible before starting
    - Handle port-in-use errors with a descriptive message
    - _Requirements: 1.1, 1.2, 1.3, 1.5_

- [ ] 8. Extend CLI arguments and wire web mode into main
  - [ ] 8.1 Extend `Args` struct and `process_args()` in `src/helper/helper.rs`
    - Add `web_mode: bool` and `port: u16` (default 8080) fields to `Args`
    - Add `--web` flag handling in `process_args()` to set `web_mode = true`
    - Add `--port <PORT>` flag handling to set custom port
    - When `web_mode` is true, skip the file_path requirement check (no `-f` needed)
    - Update `-h`/`--help` output to document `--web` and `--port` flags
    - Ensure existing CLI behavior is completely unchanged when `--web` is not provided
    - _Requirements: 1.1, 1.4_

  - [ ] 8.2 Wire web server startup into `src/main.rs`
    - Add `mod web;` declaration
    - In `main()`, after `process_args()`, check `args.web_mode`
    - If true, create a tokio/actix runtime and call `start_server` with the configured port, then return (skip all existing CLI logic)
    - If false, continue with existing CLI logic unchanged
    - _Requirements: 1.1, 1.4_

  - [ ]* 8.3 Write unit tests for extended CLI argument parsing
    - Test `--web` flag sets `web_mode = true`
    - Test `--port 3000` sets port to 3000
    - Test default port is 8080 when `--web` is provided without `--port`
    - Test that without `--web`, behavior is unchanged (web_mode = false)
    - Tests in `src/helper/helper.rs` under `#[cfg(test)]` module
    - _Requirements: 1.1, 1.4_

- [ ] 9. Create the embedded HTML frontend
  - [ ] 9.1 Create `src/web/index.html` with complete single-page application
    - Header with application title
    - Mode selector toggle (Parse EDI → JSON / Generate JSON → EDI)
    - Demo file dropdown selector that fetches from `/api/demos` and loads content via `/api/demos/{filename}`
    - Input text area with descriptive label
    - Action button (Parse / Generate) that POSTs to `/api/parse` or `/api/generate`
    - Output text area (read-only) with label, displaying pretty-printed results
    - Copy-to-clipboard button adjacent to output area
    - Download button adjacent to output area (`.json` for parse, `.edi` for generate)
    - Error display region with `aria-live="polite"` for screen reader announcements
    - Display detected transaction set type in the response
    - Semantic HTML (`<main>`, `<form>`, `<label>`, `<button>`, `<output>`)
    - ARIA attributes on dynamic regions (`role="status"`)
    - Keyboard navigation support (tab order, Enter to submit)
    - Visible focus indicators via CSS `:focus-visible`
    - Responsive layout using CSS flexbox with media queries (320px to 1920px)
    - All CSS and JavaScript inline within the single HTML file
    - Clear previous error when new content is submitted
    - Network error handling displaying "server unreachable" message
    - _Requirements: 1.2, 2.4, 3.4, 4.3, 5.1, 5.3, 6.1, 6.2, 6.3, 6.4, 6.5, 7.1, 7.2, 7.3, 7.4, 7.5, 8.1, 8.2, 8.3, 8.4_

- [ ] 10. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 11. Integration testing with actix-web test utilities
  - [ ]* 11.1 Write integration tests for the full HTTP request/response cycle
    - Use `actix_web::test::init_service` and `actix_web::test::TestRequest` to test all endpoints
    - Test GET `/` returns HTML with 200
    - Test POST `/api/parse` with each demo file returns 200 with valid JSON
    - Test POST `/api/generate` with valid JSON returns 200 with EDI content
    - Test GET `/api/demos` returns list of 12 demo files
    - Test GET `/api/demos/{valid_name}` returns demo content
    - Test GET `/api/demos/{invalid_name}` returns 404
    - Test POST `/api/parse` with empty body returns 400
    - Test POST `/api/generate` with empty body returns 400
    - Tests in `src/web/server.rs` under `#[cfg(test)]` module
    - _Requirements: 1.2, 2.1, 2.2, 2.5, 2.6, 3.1, 3.2, 3.5, 3.6, 5.2, 5.4, 5.5_

- [ ] 12. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes

- Tasks marked with `*` are optional and can be skipped for faster MVP
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
- Property tests validate the 6 correctness properties from the design document using `proptest`
- Unit and integration tests use `actix_web::test` utilities for HTTP-level testing
- All tests are placed in the same file as the code per Rust conventions (`#[cfg(test)]` modules)
- The existing CLI code path in `main.rs` must remain completely untouched — web mode is an additive branch
- Demo files are embedded at compile time via `include_str!` — no runtime filesystem access needed
