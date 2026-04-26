# Requirements Document

## Introduction

This feature adds a web-based user interface to the existing EDI Parser application. The current application is CLI-only, requiring users to work with command-line flags (`-f`, `-o`, `-w`, `-j`) to parse EDI files to JSON and generate EDI from JSON. The web UI will wrap the existing library functionality in an accessible browser-based interface served by the Rust application itself, making EDI parsing and generation easy for users who are not comfortable with the command line.

## Glossary

- **Web_Server**: The HTTP server component embedded in the Rust application that serves the web UI and handles API requests
- **Web_UI**: The browser-based frontend interface served by the Web_Server
- **EDI_Parser_Library**: The existing `edi_parser` Rust library (`lib.rs`) that provides parsing and generation functions for all supported X12 transaction sets
- **Parse_API**: The REST API endpoint that accepts EDI content and returns JSON
- **Generate_API**: The REST API endpoint that accepts JSON content and returns EDI
- **Transaction_Set**: A specific X12 EDI format (e.g., 835, 999, 270, 271, 276, 277, 278, 837P, 837I, 837D, 820, 834)
- **EDI_Content**: Raw X12 EDI text using segment terminators (`~`) and element separators (`*`)
- **JSON_Output**: The structured JSON representation produced by parsing EDI_Content
- **Input_Area**: The text area in the Web_UI where users paste or type EDI_Content or JSON content
- **Output_Area**: The read-only area in the Web_UI where parsed JSON_Output or generated EDI_Content is displayed
- **Demo_File_Selector**: A dropdown component in the Web_UI that lets users load bundled demo EDI files

## Requirements

### Requirement 1: Serve Web UI from the Rust Application

**User Story:** As a user, I want to access the EDI parser through a web browser, so that I can parse and generate EDI files without using the command line.

#### Acceptance Criteria

1. WHEN the application is started with a `--web` flag, THE Web_Server SHALL start an HTTP server on a configurable port (default 8080) and serve the Web_UI
2. WHILE the Web_Server is running, THE Web_Server SHALL serve the Web_UI as a single-page HTML application at the root path (`/`)
3. WHILE the Web_Server is running, THE Web_Server SHALL log the URL where the Web_UI is accessible
4. WHEN the `--web` flag is not provided, THE application SHALL behave identically to the current CLI interface, preserving full backward compatibility
5. IF the configured port is already in use, THEN THE Web_Server SHALL return a descriptive error message and exit

### Requirement 2: Parse EDI to JSON via Web UI

**User Story:** As a user, I want to paste EDI content into the web interface and get JSON output, so that I can quickly inspect EDI data in a readable format.

#### Acceptance Criteria

1. WHEN the user submits EDI_Content through the Web_UI, THE Parse_API SHALL accept the content via an HTTP POST request to `/api/parse`
2. WHEN valid EDI_Content for a supported Transaction_Set is submitted, THE Parse_API SHALL return the corresponding JSON_Output with HTTP status 200
3. WHEN the Parse_API receives EDI_Content, THE EDI_Parser_Library SHALL perform the same parsing logic as the existing CLI `read` operation
4. WHEN valid JSON_Output is returned, THE Web_UI SHALL display the JSON_Output in the Output_Area with formatted (pretty-printed) indentation
5. IF the EDI_Content does not match any supported Transaction_Set, THEN THE Parse_API SHALL return an error message with HTTP status 400 identifying that the format is not recognized
6. IF the EDI_Content is empty, THEN THE Parse_API SHALL return an error message with HTTP status 400 indicating that input is required

### Requirement 3: Generate EDI from JSON via Web UI

**User Story:** As a user, I want to paste JSON into the web interface and get EDI output, so that I can generate valid EDI files from structured data.

#### Acceptance Criteria

1. WHEN the user submits JSON content through the Web_UI in generate mode, THE Generate_API SHALL accept the content via an HTTP POST request to `/api/generate`
2. WHEN valid JSON content for a supported Transaction_Set is submitted, THE Generate_API SHALL return the corresponding EDI_Content with HTTP status 200
3. WHEN the Generate_API receives JSON content, THE EDI_Parser_Library SHALL perform the same generation logic as the existing CLI `write` operation
4. WHEN valid EDI_Content is returned, THE Web_UI SHALL display the EDI_Content in the Output_Area
5. IF the JSON content is malformed or does not match any supported Transaction_Set, THEN THE Generate_API SHALL return an error message with HTTP status 400
6. IF the JSON content is empty, THEN THE Generate_API SHALL return an error message with HTTP status 400 indicating that input is required

### Requirement 4: Transaction Set Auto-Detection

**User Story:** As a user, I want the parser to automatically detect which EDI format I'm working with, so that I don't have to manually specify the transaction set type.

#### Acceptance Criteria

1. WHEN EDI_Content is submitted to the Parse_API, THE Parse_API SHALL automatically detect the Transaction_Set type from the content (using ST segment identifiers and implementation guide references)
2. WHEN JSON content is submitted to the Generate_API, THE Generate_API SHALL automatically detect the Transaction_Set type from the JSON structure (using transaction_set_id fields or segment identifiers)
3. WHEN a Transaction_Set is successfully detected, THE Web_UI SHALL display the detected Transaction_Set type to the user
4. IF the Transaction_Set cannot be determined from the content, THEN THE Parse_API or Generate_API SHALL return an error message identifying the supported Transaction_Set types

### Requirement 5: Demo File Loading

**User Story:** As a user, I want to load example EDI files from the web interface, so that I can quickly try out the parser without having my own EDI files.

#### Acceptance Criteria

1. THE Web_UI SHALL include a Demo_File_Selector that lists all available demo EDI files
2. WHEN the user selects a demo file from the Demo_File_Selector, THE Web_Server SHALL serve the demo file content via an HTTP GET request to `/api/demos/{filename}`
3. WHEN a demo file is loaded, THE Web_UI SHALL populate the Input_Area with the demo file content
4. THE Web_Server SHALL serve a list of available demo files via an HTTP GET request to `/api/demos`
5. IF a requested demo file does not exist, THEN THE Web_Server SHALL return an error message with HTTP status 404

### Requirement 6: Copy and Download Output

**User Story:** As a user, I want to copy or download the output, so that I can use the parsed JSON or generated EDI in other tools.

#### Acceptance Criteria

1. THE Web_UI SHALL provide a copy-to-clipboard button adjacent to the Output_Area
2. WHEN the user clicks the copy-to-clipboard button, THE Web_UI SHALL copy the full Output_Area content to the system clipboard
3. THE Web_UI SHALL provide a download button adjacent to the Output_Area
4. WHEN the user clicks the download button after a parse operation, THE Web_UI SHALL download the output as a `.json` file
5. WHEN the user clicks the download button after a generate operation, THE Web_UI SHALL download the output as an `.edi` file

### Requirement 7: Accessible and Responsive Web UI

**User Story:** As a user, I want the web interface to be clean and easy to use on any device, so that I can work with EDI files comfortably.

#### Acceptance Criteria

1. THE Web_UI SHALL use semantic HTML elements and ARIA attributes for form controls, buttons, and output regions
2. THE Web_UI SHALL be operable using keyboard navigation alone (tab order, enter to submit, escape to clear)
3. THE Web_UI SHALL use a responsive layout that adapts to viewport widths from 320px to 1920px
4. THE Web_UI SHALL display clear labels for the Input_Area, Output_Area, mode selector, and all action buttons
5. THE Web_UI SHALL provide visible focus indicators on all interactive elements

### Requirement 8: Error Display in Web UI

**User Story:** As a user, I want to see clear error messages when something goes wrong, so that I can understand and fix the issue.

#### Acceptance Criteria

1. WHEN the Parse_API or Generate_API returns an error response, THE Web_UI SHALL display the error message in a visually distinct error region near the Output_Area
2. WHEN an error is displayed, THE Web_UI SHALL use an ARIA live region so screen readers announce the error
3. WHEN the user submits new content after an error, THE Web_UI SHALL clear the previous error message
4. IF a network error occurs while communicating with the Web_Server, THEN THE Web_UI SHALL display a message indicating the server is unreachable
