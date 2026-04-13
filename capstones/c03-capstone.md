# Capstone: ETL Pipelines with Rust

## Project Overview

Build an async ETL pipeline that ingests data from a public HTTP API, transforms it through a multi-stage serde pipeline, and writes the output as both CSV and newline-delimited JSON. The pipeline uses `tokio` for concurrency, `reqwest` for HTTP ingestion, and implements streaming with error recovery.

## Deliverables

### 1. HTTP Ingestion Layer
Build an async data fetcher using `reqwest` that pulls paginated data from a public API (e.g., GitHub API, Open Meteo, or a government data portal). Implement retry logic with exponential backoff for transient failures. Deserialize responses into typed Rust structs using `serde::Deserialize`.

### 2. Transform Pipeline
Implement at least three transformation stages: filtering (remove invalid records), mapping (derive new fields or reshape), and aggregation (group-by with summary statistics). Each stage is a separate async function that can be composed. Use `serde` derive macros for both input and output schemas.

### 3. Dual-Format Sink with Error Recovery
Write output to both CSV (via the `csv` crate) and newline-delimited JSON. If a single record fails serialization, log the error and continue processing the remaining records. Report final counts: total ingested, transformed, written, and skipped.

## Evaluation Criteria

### Distinction
- Pipeline processes multiple API pages concurrently using `tokio::join!` or `FuturesUnordered`
- Streaming mode processes records without buffering the entire dataset in memory
- End-to-end integration test uses a mock HTTP server (`wiremock` or similar)

### Proficient
- Pipeline fetches, transforms, and writes data end-to-end with correct output
- Retry logic handles at least HTTP 429 and 5xx responses
- Error recovery skips bad records without crashing the pipeline

### Developing
- Pipeline fetches and writes data but transformation is minimal
- No retry logic — transient failures crash the pipeline
- Errors are handled with `unwrap()` rather than recovery

## Share Your Work

Add this capstone to your LinkedIn profile as a portfolio project:

1. Go to your LinkedIn profile and select **Add profile section** > **Projects**
2. Title: "ETL Pipelines with Rust — Rust for Data Engineering Specialization"
3. Description: Summarize the project deliverables and key skills demonstrated
4. Link: Include the URL to your completed GitHub repository
