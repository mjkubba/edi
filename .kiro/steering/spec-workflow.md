---
inclusion: manual
---

# Spec-Driven Development Workflow

When the user asks to create a spec, follow this structured workflow to transform a rough idea into requirements, design, and implementation tasks.

## Workflow Overview

A spec is a structured way of building and documenting a feature. It follows three phases:
1. **Requirements** → Gather and document requirements with user stories and acceptance criteria
2. **Design** → Create a technical design document with architecture, components, data models, correctness properties, and error handling
3. **Tasks** → Generate an implementation plan with ordered, checkboxed tasks

## File Structure

All spec files live in `.kiro/specs/{feature-name}/`:
- `requirements.md` — Requirements document
- `design.md` — Technical design document
- `tasks.md` — Implementation task list

Use kebab-case for `{feature-name}` (e.g., `web-ui`, `schema-validation`).

## Phase 1: Requirements

Create `requirements.md` with this structure:

```markdown
# Requirements Document

## Introduction
Brief description of the feature, current state, and what this adds.

## Glossary
Define domain-specific terms used in the requirements (e.g., Web_Server, Parse_API, Transaction_Set).

## Requirements

### Requirement N: Title

**User Story:** As a [role], I want [capability], so that [benefit].

#### Acceptance Criteria
Use EARS (Easy Approach to Requirements Syntax) patterns:
- WHEN [trigger], THE [system] SHALL [behavior]
- WHILE [state], THE [system] SHALL [behavior]
- IF [condition], THEN THE [system] SHALL [behavior]
- THE [system] SHALL [behavior] (ubiquitous)
```

Guidelines:
- Each requirement has a user story and numbered acceptance criteria
- Use EARS patterns (WHEN/WHILE/IF/THE) for acceptance criteria
- Define all domain terms in the Glossary
- Reference glossary terms consistently
- Ask the user to review before proceeding to Phase 2

## Phase 2: Design

Create `design.md` with this structure:

```markdown
# Design Document: {Feature Name}

## Overview
Summary of the design approach and key design decisions (numbered list).

## Architecture
Mermaid diagrams showing component relationships and request flows.

## Components and Interfaces
For each new module/component:
- Purpose and responsibility
- Public API (function signatures, structs)
- How it integrates with existing code

## Data Models
Request/response structs, API models, any new data structures.

## Correctness Properties
Formal properties that must hold true across all valid executions:
- Property N: Title
  - *For any* [input domain], [function/system] shall [expected behavior]
  - **Validates: Requirements X.Y**

## Error Handling
- Error response table (scenario → HTTP status → message)
- Error propagation strategy
- Input validation rules

## Testing Strategy
- Property-based tests (using proptest) for correctness properties
- Unit tests (example-based) for specific cases
- Integration tests for end-to-end flows
- What is NOT tested with PBT (and why)
```

Guidelines:
- Reference requirements by number (e.g., "Validates: Requirements 4.1")
- Include mermaid diagrams for architecture and request flows
- Define correctness properties that bridge requirements to testable assertions
- Ask the user to review before proceeding to Phase 3

## Phase 3: Tasks

Create `tasks.md` with this structure:

```markdown
# Implementation Plan: {Feature Name}

## Overview
Brief summary of what will be implemented.

## Tasks

- [ ] 1. Task group title
  - [ ] 1.1 Specific subtask
    - Description of what to do
    - _Requirements: X.Y_
  - [ ]* 1.2 Optional subtask (asterisk = optional)
    - Description
    - **Validates: Requirements X.Y**

- [ ] 2. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

## Notes
- Tasks marked with `*` are optional
- Each task references specific requirements for traceability
- Checkpoints ensure incremental validation
```

Task format:
- `- [ ]` = Not started (required task)
- `- [ ]*` = Not started (optional task)
- `- [x]` = Completed
- `- [-]` = In progress
- `- [~]` = Queued

Guidelines:
- Order tasks by dependency (foundations first, then features, then tests)
- Include checkpoint tasks between major groups
- Reference requirements in each task for traceability
- Mark test-only tasks as optional with `*`
- Group related subtasks under parent tasks

## Goal: Three Artifacts

The spec workflow aims to produce three artifacts:
1. A comprehensive **specification** including correctness properties
2. A working **implementation** that conforms to that specification
3. A **test suite** that provides evidence the software obeys the correctness properties

The specification will likely need refinement as implementation progresses — this is expected and normal.

## Prerequisite Validation

Before creating tasks, verify prerequisites exist:
- **Requirements-first**: Both `requirements.md` AND `design.md` must exist before `tasks.md`
- **Design-first**: Both `design.md` AND `requirements.md` must exist before `tasks.md`
- **Bugfix**: Both `bugfix.md` AND `design.md` must exist before `tasks.md`

If a prerequisite is missing, create it first before proceeding.

## Updating Existing Specs

When updating an existing spec, read `.kiro/specs/{feature-name}/.config.kiro` to determine the `workflowType` and `specType`, then follow the appropriate workflow.

## Executing Tasks

When the user asks to execute a task:
1. Set the task status to in-progress (`- [-]`)
2. Implement the task following the project's steering rules
3. Build and test before marking complete
4. Set the task status to completed (`- [x]`)
5. Move to the next task

## Run All Tasks

When asked to execute all tasks:
1. Read `tasks.md` and identify all incomplete required tasks
2. Mark all incomplete leaf tasks as queued (`- [~]`)
3. Execute sequentially: mark in-progress → implement → mark completed → next
4. If a task fails, stop and report to the user
5. Report progress after each task completes

## Bugfix Workflow

For bugfixes, use a **bug condition methodology** instead of the standard feature workflow:

1. **Bugfix Requirements** (`requirements.md` or `bugfix.md`):
   - Define the bug condition C(X): a predicate that is true when the bug manifests
   - Document the expected vs actual behavior
   - Define preservation properties (existing correct behavior that must not break)
   - Define fix properties (the bug condition must no longer hold after the fix)

2. **Bugfix Design**: Same structure as feature design, but focused on:
   - Root cause analysis
   - Fix strategy
   - Preservation checking (ensure fix doesn't break other things)

3. **Bugfix Tasks**: Task 1 is always a **bug condition exploration test**:
   - Write a property test that SHOULD FAIL on unfixed code (proving the bug exists)
   - If the test fails as expected → bug confirmed, proceed to fix
   - If the test passes unexpectedly → root cause may be wrong, re-investigate

## File References in Specs

Spec files support `#[[file:<relative_file_name>]]` syntax to reference external documents:
```markdown
This API follows the schema defined in #[[file:openapi.yaml]]
```
This allows OpenAPI specs, GraphQL schemas, or other docs to influence implementation.

## Design-First Workflow

When the user knows the technical approach but needs to formalize requirements:
1. Create `design.md` first (with architecture, components, data models)
2. Derive `requirements.md` from the design
3. Create `tasks.md` from both

Design artifacts can include:
- **High-Level Design**: System diagrams, components, data models
- **Low-Level Design**: Code/pseudocode, algorithms, function signatures

## Workflow Rules

- Always ask the user to review each phase before moving to the next
- The existing codebase behavior must be preserved (backward compatibility)
- Follow the project's implementation-rules.md and test-guidelines.md
- Commit after completing each task group (not individual subtasks)
