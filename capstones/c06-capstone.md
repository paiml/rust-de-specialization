# Capstone: DuckDB with Rust

## Project Overview

Build an analytical SQL pipeline using the `duckdb` crate that queries local CSV, Parquet, and JSON files directly — no database server required. The pipeline demonstrates Arrow result sets, Polars interop, window functions, and OLAP aggregation patterns inside a Rust process.

## Deliverables

### 1. Multi-Format SQL Query Engine
Build a Rust application that uses DuckDB's bundled feature to query CSV, Parquet, and JSON files directly via SQL without pre-loading or schema definitions. Demonstrate at least three queries that span multiple file formats in a single SQL statement (e.g., joining a CSV against a Parquet file). Configure DuckDB with appropriate memory limits and thread counts.

### 2. Arrow Result Sets & Polars Interop
Extract DuckDB query results as Arrow RecordBatches and convert them to Polars DataFrames for further analysis. Demonstrate a pipeline where DuckDB handles the SQL aggregation and Polars handles post-processing (additional transformations, plotting data preparation, or export). Verify schema fidelity across the DuckDB-to-Arrow-to-Polars boundary.

### 3. OLAP Analytics Pipeline
Build an end-to-end analytics pipeline that uses window functions (ROW_NUMBER, RANK, LAG/LEAD), GROUP BY ROLLUP or CUBE, and at least one CTE. Process a dataset with 100K+ rows and produce a summary report. Benchmark query execution time and compare against equivalent Polars-only operations.

## Evaluation Criteria

### Distinction
- Pipeline processes 1M+ rows across three file formats with sub-second query times
- Arrow-to-Polars interop is zero-copy with verified schema preservation
- Window function queries demonstrate at least three distinct analytic patterns

### Proficient
- DuckDB queries CSV and Parquet files correctly with proper type inference
- Arrow result sets convert to Polars DataFrames with matching schemas
- At least one window function and one aggregation query produce correct results

### Developing
- DuckDB connects and executes basic SELECT queries on a single file format
- Results are extracted as rows rather than Arrow batches
- No window functions or advanced OLAP patterns

## Share Your Work

Add this capstone to your LinkedIn profile as a portfolio project:

1. Go to your LinkedIn profile and select **Add profile section** > **Projects**
2. Title: "DuckDB with Rust — Rust for Data Engineering Specialization"
3. Description: Summarize the project deliverables and key skills demonstrated
4. Link: Include the URL to your completed GitHub repository
