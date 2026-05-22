# Recommended Learning Sequence

The [README](./README.md) lists the 31 courses in Coursera **dashboard order** (the order they were authored). That is *not* the order you should learn them in.

This document is the recommended **learning order** — re-sequenced by dependency so each course builds on the previous, with parallelizable phases called out.

> **Status legend:** 🟢 Launched (live on Coursera) · ⚪ Draft (in production, not yet enrollable as of 2026-05-22)

## TL;DR — the 10 phases

```
Phase 1  Foundations (Rust + tooling)            Course 1, 11, 15, 13
Phase 2  Translation on-ramps (Python/Bash → Rust) Course 30, 31, 29   [all draft — skip if not coming from Python/Bash]
Phase 3  Single-node data muscle                  Course 2, 20, 5
Phase 4  Server-side databases                    Course 18, 17, 21, 24
Phase 5  Pipelines + algorithms                   Course 3, 22, 16
Phase 6  Cloud + DevOps                           Course 6, 12, 26, 23
Phase 7  AI augmentation                          Course 8, 14, 19
Phase 8  Correctness + sister language            Course 25, 9
Phase 9  UI surfaces (parallel; any order)        Course 4, 27, 10, 28
Phase 10 Capstone reflection                      Course 7
```

## Phase 1 — Rust + tooling foundations

Build the Rust mental model and the shell muscle memory you'll lean on for the rest of the spec.

| Order | # | Course | Status |
|---|---|---|---|
| 1 | 1 | [Rust from Zero](https://www.coursera.org/learn/rust) | 🟢 |
| 2 | 11 | [Terminal from Zero](https://www.coursera.org/learn/terminal) | 🟢 |
| 3 | 15 | [Rust CLI from Zero](https://www.coursera.org/learn/cli) | 🟢 |
| 4 | 13 | [Shipping Rust](https://www.coursera.org/learn/ship) | 🟢 |

## Phase 2 — Translation on-ramps (optional)

Skip if you're already comfortable in Rust. **All three courses are currently draft** — when they launch, they're the fastest path in for Python or Bash engineers.

| Order | # | Course | Status |
|---|---|---|---|
| 5 | 30 | Big O Notation: Python to Rust | ⚪ |
| 6 | 31 | OO: Python to Rust | ⚪ |
| 7 | 29 | Bash to Rust: From Zero | ⚪ |

## Phase 3 — Single-node data muscle

Embedded engines and dataframes — the toolkit for owning data without standing up a server.

| Order | # | Course | Status |
|---|---|---|---|
| 8 | 2 | [SQLite for Rust](https://www.coursera.org/learn/sqlite) | 🟢 |
| 9 | 20 | [DuckDB from Zero](https://www.coursera.org/learn/duckdb) | 🟢 |
| 10 | 5 | [Polars from Zero](https://www.coursera.org/learn/polars) | 🟢 |

## Phase 4 — Server-side databases

Compare the major server engines and KV / graph alternatives side-by-side rather than spreading them across the spec.

| Order | # | Course | Status |
|---|---|---|---|
| 11 | 18 | [Postgres from Zero](https://www.coursera.org/learn/postgres) | 🟢 |
| 12 | 17 | [MySQL from Zero](https://www.coursera.org/learn/queries) | 🟢 |
| 13 | 21 | Valkey from Zero | ⚪ |
| 14 | 24 | HelixDB from Zero | ⚪ |

## Phase 5 — Pipelines + algorithms

Move from "I can query a database" to "I can move data between systems reliably."

| Order | # | Course | Status |
|---|---|---|---|
| 15 | 3 | [ETL Pipelines with Rust](https://www.coursera.org/learn/etl) | 🟢 |
| 16 | 22 | [Rust for Data Source Monitoring and Automation](https://www.coursera.org/learn/monitoring) | 🟢 |
| 17 | 16 | [Graph Algorithms with Rust](https://www.coursera.org/learn/graph) | 🟢 |

## Phase 6 — Cloud + DevOps

Ship the pipelines built in Phase 5 to real infrastructure.

| Order | # | Course | Status |
|---|---|---|---|
| 18 | 6 | [Rust Serverless](https://www.coursera.org/learn/lambda) | 🟢 |
| 19 | 12 | [Rust on GCP](https://www.coursera.org/learn/rust-gcp) | 🟢 |
| 20 | 26 | IaC from Zero | ⚪ |
| 21 | 23 | [Rust DataOps: CI/CD and Containers for Data Pipelines](https://www.coursera.org/learn/pipeline) | 🟢 |

## Phase 7 — AI augmentation

Layer LLM-driven workflows on the pipeline foundation. RAG belongs here, not in Phase 4 with the databases — it leans on Claude + retrieval, not on SQL.

| Order | # | Course | Status |
|---|---|---|---|
| 22 | 8 | [Agile With AI](https://www.coursera.org/learn/agile-with-ai) | 🟢 |
| 23 | 14 | Claude from Zero | ⚪ |
| 24 | 19 | [RAG from Zero](https://www.coursera.org/learn/rag) | 🟢 |

## Phase 8 — Correctness + sister language

Lock in compile-time guarantees, then a comparative-systems lens.

| Order | # | Course | Status |
|---|---|---|---|
| 25 | 25 | Design by Provable Contracts | ⚪ |
| 26 | 9 | [Zig from Zero](https://www.coursera.org/learn/zig) | 🟢 |

## Phase 9 — UI surfaces (parallel)

These don't depend on each other — pick the surface you actually ship to and take only that one if time is short.

| Order | # | Course | Status |
|---|---|---|---|
| 27 | 4 | [Linux Desktop from Zero](https://www.coursera.org/learn/linux) | 🟢 |
| 28 | 27 | TUI from Zero | ⚪ |
| 29 | 10 | [Rust GUI from Zero](https://www.coursera.org/learn/gui) | 🟢 |
| 30 | 28 | WASM from Zero | ⚪ |

## Phase 10 — Capstone reflection

Save Data Ethics for the end. You've built the systems — now you have real cases to reason about.

| Order | # | Course | Status |
|---|---|---|---|
| 31 | 7 | [Data Ethics](https://www.coursera.org/learn/ethical) | 🟢 |

## Why this differs from dashboard order

Six deliberate re-orderings worth calling out:

1. **Terminal/CLI/Shipping (Phase 1) pulled forward** — these compound across every later phase.
2. **Python/Bash on-ramps held inside Phase 2** — only useful if you're coming from those languages.
3. **KV + graph DBs (Valkey, HelixDB) kept with the SQL DBs in Phase 4** — easier to compare than to scatter.
4. **DataOps CI/CD (#23) moved after ETL (#3)** — needs pipeline context to make sense.
5. **RAG (#19) moved out of the database phase into Phase 7** — it's an LLM/retrieval course, not a DB course.
6. **Data Ethics (#7) as the bookend** — most useful after you've built the systems it's reflecting on.

## Already know Rust? Two faster paths

**Path A — Just the data engineering core (~8 courses):**

> 2 SQLite → 20 DuckDB → 5 Polars → 18 Postgres → 3 ETL → 22 Monitoring → 6 Lambda → 23 DataOps

**Path B — Just the AI / RAG slice (~5 courses):**

> 1 Rust → 8 Agile With AI → 14 Claude from Zero → 19 RAG → 22 Monitoring

Both paths assume you'll fill in Phase 9 UI surfaces only if you actually ship one.
