# Capstone: Rust from Zero

## Project Overview

Build a command-line data processing tool that reads structured input (CSV or JSON), transforms it using iterators and closures, and writes the output in a different format. The tool demonstrates ownership, borrowing, error handling with `Result`/`Option`, and idiomatic module organization with Cargo.

## Deliverables

### 1. CLI Data Transformer
Build a Cargo project with at least two library modules and one binary crate. The tool accepts a file path and output format flag, reads structured data, applies a user-specified transformation (filter, map, or aggregate), and writes the result to stdout or a file. All I/O errors must propagate via `Result` with meaningful context.

### 2. Ownership & Borrowing Demonstration
Include at least three functions that illustrate distinct borrowing patterns: shared references for read-only access, mutable references for in-place transformation, and owned values for data consumption. Add doc comments explaining why each signature was chosen.

### 3. Test Suite
Write unit tests for each library module and an integration test that runs the binary end-to-end. Use `assert_eq!`, `assert!(result.is_err())`, and custom test helpers. Achieve at least 80% line coverage with `cargo llvm-cov`.

## Evaluation Criteria

### Distinction
- Zero `unwrap()` calls outside tests — all errors use `?` or explicit match with context
- Iterator chains replace manual loops wherever applicable
- Modules are split into separate files with a clean public API surface

### Proficient
- Tool compiles, runs, and produces correct output for the sample dataset
- Ownership and borrowing patterns are correct with no unnecessary `.clone()` calls
- Tests cover happy path and at least two error cases

### Developing
- Tool compiles and produces partial output
- Some borrowing issues resolved with `.clone()` rather than restructuring
- Minimal test coverage (happy path only)

## Share Your Work

Add this capstone to your LinkedIn profile as a portfolio project:

1. Go to your LinkedIn profile and select **Add profile section** > **Projects**
2. Title: "Rust from Zero — Rust for Data Engineering Specialization"
3. Description: Summarize the project deliverables and key skills demonstrated
4. Link: Include the URL to your completed GitHub repository
