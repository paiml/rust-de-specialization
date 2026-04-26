# Rust for Data Engineering

[![CI](https://github.com/paiml/rust-de-specialization/actions/workflows/ci.yml/badge.svg)](https://github.com/paiml/rust-de-specialization/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

<p align="center">
  <img src="assets/hero.svg" alt="Rust for Data Engineering: 7-Course Specialization" width="100%"/>
</p>

**Fast, Reliable & Correct Data Pipelines in Rust** — a 7-course Coursera specialization
that takes you from Rust fundamentals through embedded databases, async ETL pipelines,
and DataFrame analytics to compile-time correctness guarantees via provable contracts.

## Courses

| # | Course | Phase | Core Crates | Companion Repo | Capstone |
|---|--------|-------|-------------|----------------|----------|
| 1 | **Rust from Zero** | Language | std, cargo | — | [Capstone](capstones/c01-capstone.md) · [Playground Reading](capstones/c01-capstone-reading.md) |
| 2 | **SQLite for Rust** | Local DB | rusqlite | — | [Capstone](capstones/c02-capstone.md) |
| 3 | **ETL Pipelines with Rust** | Pipelines | csv, serde, tokio, reqwest | — | [Capstone](capstones/c03-capstone.md) · [Playground Reading](capstones/c03-capstone-reading.md) |
| 4 | **SQL Databases with Rust** | Production DB | sqlx | — | [Capstone](capstones/c04-capstone.md) |
| 5 | **Polars from Zero** | Analytics | polars | — | [Capstone](capstones/c05-capstone.md) |
| 6 | **Rust Serverless** | Serverless | lambda_runtime, cargo-lambda | [paiml/rust-serverless-data-engineering](https://github.com/paiml/rust-serverless-data-engineering) | [Capstone](capstones/c06-capstone.md) |
| 7 | **Design by Provable Contracts** | Correctness | std, type-system patterns | — | [Capstone](capstones/c07-capstone.md) |

## Learning Arc

```
Language ──► Local DB ──► Pipelines ──► Production DB ──► Analytics ──► Serverless ──► Correctness
  (1)          (2)          (3)            (4)              (5)           (6)            (7)
```

| Phase | Course | Builds On | Unlocks |
|-------|--------|-----------|---------|
| Language | 1 · Rust from Zero | — | Everything |
| Local DB | 2 · SQLite for Rust | Rust basics | Data persistence patterns |
| Pipelines | 3 · ETL Pipelines with Rust | Rust basics + HTTP/async | Real-world data movement |
| Production DB | 4 · SQL Databases with Rust | SQLite patterns + async (tokio) | Type-safe DB in services |
| Analytics | 5 · Polars from Zero | ETL patterns + Parquet basics | Fast DataFrame analytics |
| Serverless | 6 · Rust Serverless | Production DB + ETL patterns | Event-driven Lambda functions in Rust |
| Correctness | 7 · Design by Provable Contracts | Full Rust type system fluency | Compile-time pipeline guarantees |

## Installation

```bash
git clone https://github.com/paiml/rust-de-specialization.git
cd rust-de-specialization
make check
```

## Usage

```bash
make help          # Show available commands
make lint          # Lint markdown files
make test          # Validate course structure (7 courses, capstone sections)
make check         # Run lint + test
```

## Capstone Projects

<p align="center">
  <img src="capstones/banner.svg" alt="Capstone Projects" width="100%"/>
</p>

Each course includes a hands-on capstone project that integrates all modules into a realistic scenario. Completed capstones can be shared on LinkedIn as portfolio projects. See the [capstones/](capstones/) directory.

Courses 1 and 3 also ship **Playground Readings** — zero-install capstones you can run in your browser at <https://play.rust-lang.org/> before tackling the full project capstone:

- [**Course 1 — Rust Fundamentals**](capstones/c01-capstone-reading.md) walks through six lesson-aligned exercises to master Rust basics.
- [**Course 3 — Minimal End-to-End ETL Pipeline**](capstones/c03-capstone-reading.md) assembles a complete, Playground-runnable ETL pipeline that integrates every stage of the course into ~270 lines of Rust.

## Structure

Each course is ~60 minutes of 3–5 minute videos organized as:

**Course → Module → Lesson (3–5 videos) → Key Terms + Reflection**

Every module ends with a **Critical Thinking Assessment** (quiz + role-play practice assignment).

## Instructors

- **Noah Gift** — Founder, Pragmatic AI Labs · Duke University (Courses 3, 4, 7)
- **Alfredo Deza** — Author and content creator · Python, Rust, DevOps, ML (Courses 2, 5, 6)
- **Liam Deza** — Rust educator (Course 1)

## License

Course content copyright Pragmatic AI Labs. Code examples are MIT licensed.
