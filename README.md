# Rust for Data Engineering

[![CI](https://github.com/paiml/rust-de-specialization/actions/workflows/ci.yml/badge.svg)](https://github.com/paiml/rust-de-specialization/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

<p align="center">
  <img src="assets/hero.svg" alt="Rust for Data Engineering: Coursera Specialization" width="100%"/>
</p>

**Fast, Reliable & Correct Data Pipelines in Rust** — a **31-course Coursera specialization** that takes you from Rust fundamentals through embedded databases, async ETL, analytics, GUI, parallelism, and compile-time correctness, all the way to formal-contract-gated production pipelines.

Track the live curriculum at <https://www.coursera.org/teach-specialization/rust/>.

## Courses

The full 31-course catalog in Coursera dashboard order. `🟢 Launched` = live on Coursera; `⚪ Draft` = in production.

| # | Course | Status | Instructor | Companion Repo |
|---|---|---|---|---|
| 1 | Rust from Zero | 🟢 Launched | Liam Parker | — |
| 2 | SQLite for Rust | 🟢 Launched | Alfredo Deza | [paiml/rust-for-sqlite](https://github.com/paiml/rust-for-sqlite) |
| 3 | ETL Pipelines with Rust | 🟢 Launched | Noah Gift | [this repo](https://github.com/paiml/rust-de-specialization) |
| 4 | Linux Desktop from Zero | 🟢 Launched | Noah Gift | — |
| 5 | Polars from Zero | 🟢 Launched | Alfredo Deza | [paiml/polars-fundamentals](https://github.com/paiml/polars-fundamentals) |
| 6 | Rust Serverless | 🟢 Launched | Noah Gift | [paiml/rust-serverless-data-engineering](https://github.com/paiml/rust-serverless-data-engineering) |
| 7 | Data Ethics | 🟢 Launched | Noah Gift | — |
| 8 | Agile With AI | 🟢 Launched | Noah Gift | — |
| 9 | Zig from Zero | 🟢 Launched | Noah Gift | [paiml/zig-from-zero](https://github.com/paiml/zig-from-zero) |
| 10 | Rust GUI from Zero | 🟢 Launched | Noah Gift | [paiml/rust-gui-from-zero](https://github.com/paiml/rust-gui-from-zero) |
| 11 | Terminal from Zero | 🟢 Launched | Noah Gift | — |
| 12 | Rust on GCP | 🟢 Launched | Noah Gift | — |
| 13 | Shipping Rust | 🟢 Launched | Noah Gift | [paiml/shipping-rust](https://github.com/paiml/shipping-rust) |
| 14 | Claude from Zero | ⚪ Draft | Noah Gift | [paiml/claude-from-zero](https://github.com/paiml/claude-from-zero) |
| 15 | Rust CLI from Zero | 🟢 Launched | Alfredo Deza | [paiml/rust-cli](https://github.com/paiml/rust-cli) |
| 16 | Graph Algorithms with Rust | 🟢 Launched | Noah Gift | [paiml/rust-graph-algorithms](https://github.com/paiml/rust-graph-algorithms) |
| 17 | MySQL from Zero | 🟢 Launched | Alfredo Deza | [paiml/mysql-from-zero](https://github.com/paiml/mysql-from-zero) |
| 18 | Postgres from Zero | 🟢 Launched | Alfredo Deza | [paiml/postgres-from-zero](https://github.com/paiml/postgres-from-zero) |
| 19 | RAG from Zero | 🟢 Launched | Noah Gift | [paiml/rag-from-zero](https://github.com/paiml/rag-from-zero) |
| 20 | DuckDB from Zero | 🟢 Launched | Alfredo Deza | [paiml/duckdb-from-zero](https://github.com/paiml/duckdb-from-zero) |
| 21 | Valkey from Zero | ⚪ Draft | Noah Gift | [paiml/valkey-from-zero](https://github.com/paiml/valkey-from-zero) |
| 22 | Rust for Data Source Monitoring and Automation | 🟢 Launched | Alfredo Deza | [this repo](https://github.com/paiml/rust-de-specialization) |
| 23 | Rust DataOps: CI/CD and Containers for Data Pipelines | 🟢 Launched | Alfredo Deza | [this repo](https://github.com/paiml/rust-de-specialization) |
| 24 | HelixDB from Zero | ⚪ Draft | Noah Gift | [paiml/helixdb-from-zero](https://github.com/paiml/helixdb-from-zero) |
| 25 | Design by Provable Contracts | ⚪ Draft | Noah Gift | [paiml/design-by-provable-contracts](https://github.com/paiml/design-by-provable-contracts) |
| 26 | IaC from Zero | ⚪ Draft | Noah Gift | [paiml/iac-from-zero](https://github.com/paiml/iac-from-zero) |
| 27 | TUI from Zero | ⚪ Draft | Noah Gift | [paiml/tui-from-zero](https://github.com/paiml/tui-from-zero) |
| 28 | WASM from Zero | ⚪ Draft | Noah Gift | [paiml/wasm-from-zero](https://github.com/paiml/wasm-from-zero) |
| 29 | Bash to Rust: From Zero | ⚪ Draft | Noah Gift | [paiml/bashrs-from-zero](https://github.com/paiml/bashrs-from-zero) |
| 30 | Big O Notation: Python to Rust | ⚪ Draft | Noah Gift | [paiml/big-o-python-to-rust](https://github.com/paiml/big-o-python-to-rust) |
| 31 | OO: Python to Rust | ⚪ Draft | Noah Gift | [paiml/oo-python-to-rust](https://github.com/paiml/oo-python-to-rust) |

**Snapshot taken 2026-05-21** from the Coursera teach dashboard — see the curriculum URL above for the live state.

## Capstone Projects

<p align="center">
  <img src="capstones/banner.svg" alt="Capstone Projects" width="100%"/>
</p>

Each course includes a hands-on capstone project that integrates all modules into a realistic scenario. Completed capstones can be shared on LinkedIn as portfolio projects. See the [capstones/](capstones/) directory.

Two courses also ship **Playground Readings** — zero-install companion walkthroughs that run on <https://play.rust-lang.org/> so you can master the core concepts in the browser before tackling the full project capstone:

- [Course 1 Playground Reading](capstones/c01-capstone-reading.md) — six lesson-aligned exercises covering Rust fundamentals
- [Course 4 Playground Reading](capstones/c04-capstone-reading.md) — eight lesson-aligned sections on async pools, typed row deserialization, idempotent migrations, RAII-rollback transactions, and row-aligned validation

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
make test          # Validate course structure
make check         # Run lint + test
```

## Structure

Each course is roughly 25-55 items of 3–6 minute videos plus per-lesson Key-Terms reading + Reflection reading, organized as:

**Course → Module (3–5) → Lesson (1–2) → Key Terms (≤150w) + Videos (≤6 min each, max 3 per lesson) + Reflection (≤150w)**

Each course ends with **one** course-end graded 5-question quiz at 80% pass per the PAIML Course Design Standard (banned items: AI Coach, Dialogue, Role-Play, peer-review, module-level graded quizzes, two-presenter videos, supplementary readings).

## Instructors

- **Noah Gift** — Founder, Pragmatic AI Labs · Duke University
- **Alfredo Deza** — Author and content creator · Python, Rust, DevOps, ML
- **Liam Parker** — Rust educator

## License

Course content copyright Pragmatic AI Labs. Code examples are MIT licensed.
