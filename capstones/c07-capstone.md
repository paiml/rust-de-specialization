# Capstone: Design by Provable Contracts

## Project Overview

Build a type-safe data pipeline library that encodes correctness invariants at compile time using newtype wrappers, phantom types, and typestate patterns. The library makes illegal pipeline states unrepresentable and includes property-based tests that verify contracts hold under random inputs.

## Deliverables

### 1. Newtype & Phantom Type Layer
Define at least four newtype wrappers that prevent unit confusion in your pipeline domain (e.g., `Bytes(u64)` vs `Rows(u64)`, `Validated<T>` vs `Raw<T>`). Use phantom type parameters to distinguish pipeline stages at the type level — a `Pipeline<Unchecked>` cannot call `.execute()`, only `Pipeline<Validated>` can.

### 2. Typestate Machine
Implement a state machine for a data pipeline lifecycle (e.g., `Configured → Connected → Streaming → Completed`) where each transition is a method that consumes `self` and returns the next state. Invalid transitions (e.g., calling `.stream()` on a `Configured` pipeline) must be compile-time errors, not runtime checks.

### 3. Property-Based Test Suite
Write property-based tests using `proptest` or `quickcheck` that generate random inputs and verify pipeline invariants: monotonic progress (row count never decreases), schema preservation (output schema matches declared contract), and error containment (invalid inputs produce `Err`, never panic). Include at least five property tests.

## Evaluation Criteria

### Distinction
- Type system prevents at least three categories of bugs that would be runtime errors in a dynamically typed language
- Typestate machine has four or more states with at least one branching transition
- Property tests cover edge cases including empty inputs, maximum values, and unicode strings

### Proficient
- Newtype wrappers prevent at least two categories of type confusion
- Typestate machine enforces valid transitions at compile time (demonstrated by commented-out failing code)
- At least three property tests verify meaningful invariants

### Developing
- Newtypes exist but are thin wrappers with public inner fields (invariants not enforced)
- State machine uses runtime checks (enums + match) rather than compile-time typestate
- Tests are example-based rather than property-based

## Share Your Work

Add this capstone to your LinkedIn profile as a portfolio project:

1. Go to your LinkedIn profile and select **Add profile section** > **Projects**
2. Title: "Design by Provable Contracts — Rust for Data Engineering Specialization"
3. Description: Summarize the project deliverables and key skills demonstrated
4. Link: Include the URL to your completed GitHub repository
