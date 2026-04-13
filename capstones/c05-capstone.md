# Capstone: Polars from Zero

## Project Overview

Build an analytical data pipeline using Polars that ingests a real-world dataset (CSV or Parquet), performs exploratory analysis through the lazy evaluation API, and produces summary reports with group-by aggregations, joins, and performance comparisons between eager and lazy execution.

## Deliverables

### 1. Lazy Evaluation Pipeline
Build a Polars pipeline using `LazyFrame` that chains at least five operations: filter, with_columns (derived expressions), group_by with aggregation, sort, and limit. Collect the result and write it to Parquet. Print the optimized query plan using `explain()` and include it in your report.

### 2. Multi-Source Join Analysis
Load data from at least two sources (e.g., CSV + Parquet, or two CSVs with a shared key). Perform an inner join and a left join, demonstrating key selection, column disambiguation, and null handling in the joined result. Compare row counts before and after each join type.

### 3. Performance Benchmark Report
Measure execution time for the same analytical query using: (a) eager `DataFrame` operations, (b) lazy `LazyFrame` with `collect()`, and (c) lazy with predicate pushdown and projection pushdown enabled. Report wall-clock times for each and explain which optimizations the query planner applied.

## Evaluation Criteria

### Distinction
- Pipeline processes a dataset with 1M+ rows and completes within seconds
- Query plan analysis identifies specific optimizations (predicate pushdown, projection pruning)
- Expressions use Polars-native syntax (no `.apply()` with Python-style closures)

### Proficient
- Lazy pipeline chains five or more operations with correct output
- Join produces correct results with documented null handling strategy
- Benchmark shows measurable difference between eager and lazy execution

### Developing
- Pipeline runs but uses eager mode throughout with no lazy evaluation
- Join works but column naming conflicts are unresolved
- No performance comparison between execution strategies

## Share Your Work

Add this capstone to your LinkedIn profile as a portfolio project:

1. Go to your LinkedIn profile and select **Add profile section** > **Projects**
2. Title: "Polars from Zero — Rust for Data Engineering Specialization"
3. Description: Summarize the project deliverables and key skills demonstrated
4. Link: Include the URL to your completed GitHub repository
