# Capstone: SQLite for Rust

## Project Overview

Build a local inventory management system backed by SQLite using `rusqlite`. The application creates a normalized schema, performs full CRUD operations with parameterised queries, and uses transactions to ensure data integrity during bulk imports.

## Deliverables

### 1. Schema Design & Migration
Design a multi-table SQLite schema (at least three tables with foreign keys) for a domain of your choice (inventory, contacts, bookmarks). Implement a migration function that creates tables idempotently using `IF NOT EXISTS` and applies schema changes on startup.

### 2. CRUD Operations Module
Implement a Rust module with typed functions for Create, Read, Update, and Delete operations. All queries must use parameterised bindings (never string interpolation). Read operations return Rust structs with proper type mapping from SQLite columns.

### 3. Transaction-Safe Bulk Import
Implement a bulk import function that reads a CSV file and inserts all rows inside a single transaction. If any row fails validation, the entire transaction rolls back. Benchmark the import with and without transactions using `std::time::Instant` and report the speedup.

## Evaluation Criteria

### Distinction
- Schema includes CHECK constraints or triggers for domain validation
- Bulk import handles 10,000+ rows within a transaction with measured performance
- Error types are domain-specific (custom enum implementing `std::error::Error`)

### Proficient
- Schema is normalized with foreign key constraints enforced (`PRAGMA foreign_keys = ON`)
- All four CRUD operations work correctly with parameterised queries
- Transaction rollback is tested with an intentionally invalid row

### Developing
- Schema exists but lacks foreign keys or constraints
- CRUD operations work but some use string formatting instead of parameters
- Transactions are present but rollback behavior is untested

## Share Your Work

Add this capstone to your LinkedIn profile as a portfolio project:

1. Go to your LinkedIn profile and select **Add profile section** > **Projects**
2. Title: "SQLite for Rust — Rust for Data Engineering Specialization"
3. Description: Summarize the project deliverables and key skills demonstrated
4. Link: Include the URL to your completed GitHub repository
