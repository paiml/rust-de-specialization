# Capstone: Apache Arrow for Rust

## Project Overview

Build a zero-copy data interchange layer using `arrow-rs` that constructs RecordBatches from raw data, serializes them via Arrow IPC, and demonstrates interoperability by reading the same data in Polars and/or DataFusion without deserialization overhead.

## Deliverables

### 1. RecordBatch Construction Pipeline
Build a pipeline that constructs Arrow `RecordBatch` instances from a structured source (CSV, JSON, or programmatic data). Use typed `ArrayBuilder` instances (StringBuilder, Float64Builder, TimestampBuilder) to populate columns. Define an explicit `Schema` with field-level metadata and nullable flags.

### 2. Arrow IPC Serialization
Serialize RecordBatches to Arrow IPC format (both file and streaming variants) using `FileWriter` and `StreamWriter`. Read them back with the corresponding readers and verify round-trip fidelity: schema equality, row counts, and spot-checked values. Measure serialization throughput in MB/s.

### 3. Cross-Engine Interop Demonstration
Write an Arrow IPC file from your Rust pipeline, then read it in at least one other engine: Polars (`read_ipc`), DataFusion (`register_ipc`), or PyArrow. Confirm that the data arrives with identical schema and values — zero-copy, no CSV re-parsing. Document the column types, null counts, and any schema negotiation required.

## Evaluation Criteria

### Distinction
- Pipeline handles at least three Arrow data types including a nested type (List, Struct, or Dictionary)
- IPC round-trip throughput exceeds 500 MB/s for a 100MB dataset
- Interop demonstration includes two different consumer engines

### Proficient
- RecordBatches are correctly constructed with typed builders and an explicit schema
- IPC round-trip preserves all data with verified equality checks
- At least one cross-engine read demonstrates zero-copy interop

### Developing
- RecordBatches are constructed but schema is inferred rather than explicit
- IPC serialization works but round-trip verification is incomplete
- No cross-engine interop — only Rust-to-Rust round-trip

## Share Your Work

Add this capstone to your LinkedIn profile as a portfolio project:

1. Go to your LinkedIn profile and select **Add profile section** > **Projects**
2. Title: "Apache Arrow for Rust — Rust for Data Engineering Specialization"
3. Description: Summarize the project deliverables and key skills demonstrated
4. Link: Include the URL to your completed GitHub repository
