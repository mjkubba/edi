---
inclusion: manual
---

# Spec Creation Guide

This guide teaches how to create a full spec (requirements → design → tasks) for a feature, replicating the structured spec-driven development workflow.

## When to Use

Use this when the user asks to:
- "Create a spec for [feature]"
- "Spec out [feature]"
- "Plan [feature] implementation"
- "Design [feature]"

## Step 1: Classify and Gather Context

1. Determine if this is a **new feature** or a **bugfix**
2. Pick a kebab-case feature name (e.g., `schema-validation`)
3. Read the existing codebase to understand current architecture:
   - `src/main.rs`, `src/lib.rs`, `Cargo.toml` for project structure
   - Relevant existing modules for integration points
   - `README.md` for project status

## Step 2: Create Requirements (`requirements.md`)

Create `.kiro/specs/{feature-name}/requirements.md` following this template:

### Structure
1. **Introduction** — What the feature does, current state, why it's needed
2. **Glossary** — Define all domain terms used in requirements (use PascalCase_With_Underscores like `Web_Server`, `Parse_API`)
3. **Requirements** — Each with:
   - A numbered title
   - A user story: "As a [role], I want [capability], so that [benefit]"
   - Numbered acceptance criteria using EARS syntax:
     - `WHEN [trigger], THE [system] SHALL [behavior]`
     - `WHILE [state], THE [system] SHALL [behavior]`
     - `IF [condition], THEN THE [system] SHALL [behavior]`
     - `THE [system] SHALL [behavior]` (ubiquitous/always true)

### Quality Rules
- Each acceptance criterion is testable and unambiguous
- Use glossary terms consistently
- Cover happy paths, error cases, and edge cases
- Ensure backward compatibility requirements are explicit

**Present to user for review before proceeding.**

## Step 3: Create Design (`design.md`)

Create `.kiro/specs/{feature-name}/design.md` following this template:

### Structure
1. **Overview** — Design summary and key design decisions (numbered)
2. **Architecture** — Mermaid diagrams showing:
   - Component relationships (graph TB)
   - Request/data flows (sequenceDiagram)
3. **Components and Interfaces** — For each new module:
   - File location and purpose
   - Public API with Rust function signatures
   - Route table (for web APIs)
4. **Data Models** — Request/response structs with `#[derive(Serialize, Deserialize)]`
5. **Correctness Properties** — Formal properties:
   - "For any [input], [function] shall [behavior]"
   - Each validates specific requirements
6. **Error Handling** — Table of scenarios → HTTP status → error message
7. **Testing Strategy**:
   - Property-based tests (proptest) for correctness properties
   - Unit tests for specific examples
   - Integration tests for end-to-end
   - What is NOT tested with PBT

### Design Principles
- Reuse existing library functions — don't reimplement parsing/generation logic
- Embed assets at compile time (`include_str!`) for single-binary deployment
- Preserve full backward compatibility with existing CLI
- Follow existing code patterns (check steering/implementation-rules.md)

**Present to user for review before proceeding.**

## Step 4: Create Tasks (`tasks.md`)

Create `.kiro/specs/{feature-name}/tasks.md` following this template:

### Structure
```markdown
# Implementation Plan: {Feature Name}

## Overview
Brief summary.

## Tasks

- [ ] 1. Task group
  - [ ] 1.1 Subtask description
    - What to do, what files to create/modify
    - _Requirements: X.Y_
  - [ ]* 1.2 Optional test task
    - **Property N: Title**
    - **Validates: Requirements X.Y**

- [ ] 2. Checkpoint - Ensure all tests pass

## Notes
- Tasks with `*` are optional
- Checkpoints between major groups
```

### Task Ordering
1. Dependencies and module structure first
2. Core logic (detection, handlers)
3. Tests for core logic
4. Checkpoint
5. Integration (wiring, server setup, CLI args)
6. UI/frontend
7. Checkpoint
8. Integration tests
9. Final checkpoint

### Task Format
- `- [ ]` required, `- [ ]*` optional
- `- [x]` completed, `- [-]` in progress, `- [~]` queued
- Each task references requirements for traceability
- Subtasks under parent tasks

**Present to user for review.**

## Step 5: Execute Tasks

When asked to execute tasks:
1. Read `tasks.md` to find the next incomplete required task
2. Mark it `- [-]` (in progress)
3. Implement following project steering rules
4. Build (`cargo.exe build`) and test (`cargo.exe test`)
5. Format (`cargo.exe +nightly fmt`)
6. Mark `- [x]` (completed)
7. Commit with conventional commit message
8. Proceed to next task

## Config File

Also create `.kiro/specs/{feature-name}/.config.kiro`:
```json
{"specId": "<uuid>", "workflowType": "requirements-first", "specType": "feature"}
```
