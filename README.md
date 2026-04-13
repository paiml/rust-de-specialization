# Rust for Data Engineering

<p align="center">
  <img src="assets/hero.svg" alt="Rust for Data Engineering: 7-Course Specialization" width="100%"/>
</p>

**Fast, Reliable & Correct Data Pipelines in Rust** — a 7-course Coursera specialization
that takes you from Rust fundamentals through embedded databases, async ETL pipelines,
and DataFrame analytics to compile-time correctness guarantees via provable contracts.

## Courses

| # | Course | Phase | Core Crates | Companion Repo | Capstone |
|---|--------|-------|-------------|----------------|----------|
| 1 | **Rust from Zero** | Language | std, cargo | — | [Capstone](capstones/c01-capstone.md) |
| 2 | **SQLite for Rust** | Local DB | rusqlite | — | [Capstone](capstones/c02-capstone.md) |
| 3 | **ETL Pipelines with Rust** | Pipelines | csv, serde, tokio, reqwest | — | [Capstone](capstones/c03-capstone.md) |
| 4 | **SQL Databases with Rust** | Production DB | sqlx | — | [Capstone](capstones/c04-capstone.md) |
| 5 | **Polars from Zero** | Analytics | polars | — | [Capstone](capstones/c05-capstone.md) |
| 6 | **Apache Arrow for Rust** | Interop | arrow-rs | — | [Capstone](capstones/c06-capstone.md) |
| 7 | **Design by Provable Contracts** | Correctness | std, type-system patterns | — | [Capstone](capstones/c07-capstone.md) |

## Learning Arc

```
Language ──► Local DB ──► Pipelines ──► Production DB ──► Analytics ──► Interop ──► Correctness
  (1)          (2)          (3)            (4)              (5)          (6)           (7)
```

| Phase | Course | Builds On | Unlocks |
|-------|--------|-----------|---------|
| Language | 1 · Rust from Zero | — | Everything |
| Local DB | 2 · SQLite for Rust | Rust basics | Data persistence patterns |
| Pipelines | 3 · ETL Pipelines with Rust | Rust basics + HTTP/async | Real-world data movement |
| Production DB | 4 · SQL Databases with Rust | SQLite patterns + async (tokio) | Type-safe DB in services |
| Analytics | 5 · Polars from Zero | ETL patterns + Parquet basics | Fast DataFrame analytics |
| Interop | 6 · Apache Arrow for Rust | Polars + columnar concepts | Cross-engine zero-copy sharing |
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

## Structure

Each course is ~60 minutes of 3–5 minute videos organized as:

**Course → Module → Lesson (3–5 videos) → Key Terms + Reflection**

Every module ends with a **Critical Thinking Assessment** (quiz + role-play practice assignment).

## Instructors

- **Noah Gift** — Founder, Pragmatic AI Labs · Duke University (Courses 3, 4, 6, 7)
- **Alfredo Deza** — Author and content creator · Python, Rust, DevOps, ML (Courses 2, 5)
- **Liam Deza** — Rust educator (Course 1)

## License

Course content copyright Pragmatic AI Labs. Code examples are MIT licensed.
