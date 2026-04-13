# Capstone: SQL Databases with Rust

## Project Overview

Build a REST API service backed by PostgreSQL (or SQLite) using `sqlx` with compile-time verified queries. The service manages a multi-entity domain with async connection pooling, migrations, and transactional writes — demonstrating production-grade database integration patterns.

## Deliverables

### 1. Compile-Time Query Layer
Implement at least five `sqlx::query!` or `sqlx::query_as!` macros that are verified at compile time against a live database schema. Include SELECT with joins, INSERT with RETURNING, UPDATE with conditional logic, and DELETE with cascading constraints. Document the `DATABASE_URL` setup for `cargo check` to verify queries.

### 2. Migration System
Create a `migrations/` directory with at least three sequential migration files managed by `sqlx migrate`. Migrations must be idempotent and include both schema creation and seed data. Demonstrate running migrations programmatically on application startup using `sqlx::migrate!()`.

### 3. Connection Pool & Transaction Management
Configure `sqlx::PgPool` (or `SqlitePool`) with explicit pool size, connection timeout, and idle timeout settings. Implement at least one multi-statement transaction that atomically updates related tables. Include a test that verifies rollback on constraint violation.

## Evaluation Criteria

### Distinction
- All queries use compile-time `query!` macros with zero runtime SQL string construction
- Connection pool is instrumented with metrics (query count, pool utilization)
- Service includes a health check endpoint that verifies database connectivity and migration status

### Proficient
- At least three queries use compile-time macros and type-check against the schema
- Migrations run automatically on startup and are tested in CI
- Transactions correctly roll back on error with tested failure cases

### Developing
- Queries work but use runtime `query()` instead of compile-time macros
- Migrations exist but are applied manually rather than programmatically
- Transactions are present but rollback behavior is untested

## Share Your Work

Add this capstone to your LinkedIn profile as a portfolio project:

1. Go to your LinkedIn profile and select **Add profile section** > **Projects**
2. Title: "SQL Databases with Rust — Rust for Data Engineering Specialization"
3. Description: Summarize the project deliverables and key skills demonstrated
4. Link: Include the URL to your completed GitHub repository
