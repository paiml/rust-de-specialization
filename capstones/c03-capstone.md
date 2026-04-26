# Capstone: Fruit Inventory ETL Pipeline

## The Scenario

A fintech operations team ships a 10,000-row fruit inventory CSV every night. For the last six months the incumbent pipeline has been a Python script that wraps `csv.DictReader` with `json.dumps`, and for the last six months it has been paging the on-call rotation roughly once a week. The failure mode is always the same: a single malformed row — "OOPS" in the price column, an empty name, a negative weight — raises a `ValueError` three hours into the batch, the process exits non-zero, and the other 9,999 rows are silently discarded because the script never committed its output. The finance team only notices when the quarterly reconciliation comes up 153 rows short.

You are the data engineer hired to replace that brittle extract stage with a Rust pipeline that the on-call rotation can actually trust. The analytics team consumes NDJSON via `jq`. The finance team's SQL warehouse consumes CSV. Both teams expect the same 850 valid records every night, with zero drift between formats and zero silently-dropped rows. The pipeline must run to completion against any input the operations team can throw at it, produce a structured report the operator can read on stderr, and exit with a code that Airflow can route on.

This capstone asks you to integrate everything the course has covered — the totality invariant, [`impl Read`](https://doc.rust-lang.org/std/io/trait.Read.html) and [`impl Write`](https://doc.rust-lang.org/std/io/trait.Write.html) abstractions, accumulation over short-circuit, dual-format sinks, [`thiserror`](https://docs.rs/thiserror/) enums, property-based testing, and structured reporting — into one coherent end-to-end pipeline. The course opened with the Row-847 incident pattern (Module 1.2.1). This capstone is the production-shaped replacement.

## Requirements

### Extract (from Week 1)

- Define `RawFruit` with `#[derive(Deserialize)]` for `name`, `price: f64`, `category`, `weight_kg: f64`, plus an `Option<bool>` for `organic` where the column may be empty.
- Define a typed `enum ParseError { Csv { row: usize, source: csv::Error } }` — never `Vec<String>`; this is the Module 4.1 lesson applied at the extract boundary.
- Write `fn extract<R: Read>(r: R) -> ExtractReport` where `struct ExtractReport { ok: Vec<RawFruit>, errors: Vec<ParseError>, rows_read: usize }` so one function body accepts files, [`io::stdin().lock()`](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.lock), and `&[u8]` test fixtures on the same compiled path.
- Configure [`csv::ReaderBuilder`](https://docs.rs/csv/latest/csv/struct.ReaderBuilder.html) with `.has_headers(true)`, `.delimiter(b',')`, and `.flexible(false)` — `.flexible(false)` is a data-quality lever that rejects wrong-column-count rows rather than silently accepting them.
- Iterate `rdr.deserialize::<RawFruit>()` and push every outcome into either `ok` or `errors` without calling `unwrap()` or `?` on any individual row result.
- Assert the totality invariant `report.ok.len() + report.errors.len() == report.rows_read` before returning.

### Transform (from Week 2)

- Define `EnrichedFruit` with `price_cents: u64`, `weight_grams: u32`, `name: String`, `category: String`, `organic: bool`, and a computed `cents_per_kg: u64` field — [unit-typed fields](https://doc.rust-lang.org/std/primitive.u64.html) all the way down, no `f64` leaking past the transform boundary.
- Define the **row-aligned** error channel so the totality invariant holds even when one input row produces multiple validation failures:

  ```rust
  struct InvalidRow { row: RawFruit, errors: Vec<ValidationError> }
  struct TransformReport {
      ok: Vec<EnrichedFruit>,
      errors: Vec<InvalidRow>,   // one entry per rejected input row
      input_len: usize,
  }
  ```

- Write `fn transform(input: Vec<RawFruit>) -> TransformReport` so the two output channels are compiler-enforced. The invariant `report.ok.len() + report.errors.len() == report.input_len` asserts at the stage exit — and it holds because each `InvalidRow` is **one failed row**, not one individual error.
- Normalize `price: f64` to `price_cents: u64` via `(price * 100.0).`[`round()`](https://doc.rust-lang.org/std/primitive.f64.html#method.round)` as u64` and `weight_kg: f64` to `weight_grams: u32` via `(weight_kg * 1000.0).round() as u32`. The `.round()` is load-bearing: [`as` float-to-integer casts round toward zero](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast), so without it `0.1 * 1000.0 = 99.999...` truncates to `99`.
- Apply validation rules in accumulate-over-short-circuit style: reject rows where `price <= 0.0`, reject rows where `weight_kg <= 0.0`, and reject rows where `name.trim()` or `category.trim()` is empty. A row with three defects produces **one `InvalidRow`** carrying a `Vec<ValidationError>` of length three — not three separate entries in `report.errors`. This is what keeps the invariant row-aligned.
- Compute `cents_per_kg = (price_cents * 1000) / (weight_grams as u64)` for every row that survives validation. **Parenthesize as shown** — [Rust binds `as` tighter than `*` and `/`](https://doc.rust-lang.org/reference/expressions/operator-expr.html), so `price_cents * 1000 / weight_grams as u64` works, but `price_cents / weight_grams as u64 * 1000` does integer division first and silently returns `0` for any weight above the price in its raw units.

### Load (from Week 3)

- Define the load-stage report parallel to Weeks 1 and 2:

  ```rust
  struct SkipRecord { row: usize, cause: String }
  struct LoadReport { ok: usize, skip: Vec<SkipRecord> }
  ```

- Write `fn load_both<W1: Write, W2: Write>(records: &[EnrichedFruit], ndjson: &mut W1, csv: &mut W2) -> Result<LoadReport, EtlError>`. One iteration over the slice feeds a [`serde_json::to_string(&row)?`](https://docs.rs/serde_json/latest/serde_json/fn.to_string.html) + [`writeln!`](https://doc.rust-lang.org/std/macro.writeln.html) path into the NDJSON sink and a [`csv::WriterBuilder::new().from_writer(csv)`](https://docs.rs/csv/latest/csv/struct.WriterBuilder.html) path into the CSV sink. A per-record failure (e.g., serialization of a particular row) appends a `SkipRecord` to `report.skip` and the loop continues. A fatal sink failure (disk full, permission denied) returns [`Err(EtlError)`](https://doc.rust-lang.org/std/result/enum.Result.html) and short-circuits the batch — once the sink is broken, no further writes can succeed.
- Call `wtr.flush()?` explicitly on the CSV writer before returning. Never rely on the RAII [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) flush: `fn drop(&mut self)` has no return type, so any I/O error during the final flush is silently discarded.
- Both sinks must emit the exact same record count and preserve field values under a round-trip deserialize-and-compare test.
- The load-stage totality invariant on the `Ok` path is `report.ok + report.skip.len() == records.len()`. Fatal sink errors are typed as `EtlError` and propagate through `?` — they do not live in a `fail` counter, because once the batch short-circuits there is no meaningful total to reconcile against.
- Convert a skip-rate `> 5%` into `Err(EtlError::SkipThresholdExceeded { skip_rate })` so a broken upstream feed does not silently chew through an hour of compute.

### Composition (from Week 4)

- Define the composed pipeline report:

  ```rust
  struct PipelineReport {
      extract: ExtractReport,       // ok: Vec<RawFruit>,      errors: Vec<ParseError>,  rows_read: usize
      transform: TransformReport,   // ok: Vec<EnrichedFruit>, errors: Vec<InvalidRow>,  input_len: usize
      load: LoadReport,             // ok: usize,              skip:   Vec<SkipRecord>
  }
  ```

- Write `fn run_pipeline<R: Read, W1: Write, W2: Write>(r: R, ndjson: &mut W1, csv: &mut W2) -> Result<PipelineReport, EtlError>` as the library entry point — every stage composes through this one function.
- Write `main` as a thin orchestrator: open the input file, open both output files, call `run_pipeline`, print the `PipelineReport` to stderr, return [`std::process::ExitCode`](https://doc.rust-lang.org/std/process/struct.ExitCode.html). Zero business logic in `main`.
- Define `#[derive(Debug, Error)] enum EtlError` via the [thiserror crate's `#[derive(Error)]`](https://docs.rs/thiserror/latest/thiserror/derive.Error.html) proc-macro with at least `Io(#[from] io::Error)`, `Csv(#[from] csv::Error)`, `Json(#[from] serde_json::Error)`, and a domain variant `SkipThresholdExceeded { skip_rate: f64 }`. [`#[from]`](https://doc.rust-lang.org/std/convert/trait.From.html) belongs on library errors (so [`?`](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator) auto-converts them); domain variants are constructed explicitly.
- Exit code `0` iff every stage returned `Ok` and `report.transform.errors.is_empty() && report.load.skip.is_empty()`. Exit code `1` for partial failure (some rows succeeded, some failed). Exit code `2` for total failure (extract died, sink unavailable, or zero rows made it through).
- Emit the `PipelineReport` to stderr (`extract: N ok / M errors / P rows_read`, `transform: N ok / M errors`, `load: N ok / M skip`, plus paths written). stdout stays pure data or empty; stderr is operational.

## Deliverables

1. A local Cargo project under `capstone/` with `src/lib.rs` (the library) and `src/main.rs` (the thin orchestrator). `cargo test` and `cargo clippy -- -D warnings` must pass clean.
2. A `tests/fixtures/` directory with at least two deterministic CSV inputs (one clean, one containing the Row-847 pattern plus empty-name, negative-price, and wrong-column-count rows) and matching `tests/expected/` golden files for both NDJSON and CSV outputs.
3. A `tests/integration.rs` that feeds the fixture through `run_pipeline` and compares against the golden files with [`assert_eq!`](https://doc.rust-lang.org/std/macro.assert_eq.html) on the byte vectors — gated behind `UPDATE_GOLDEN=1 cargo test` for intentional schema changes.
4. A [proptest](https://docs.rs/proptest/) property in `tests/totality_prop.rs` that generates `Vec<RawFruit>` of up to 100 rows with names including the empty string, prices in `-5.0..15.0`, and weights in `-0.5..3.0`, and asserts `transform_report.ok.len() + transform_report.errors.len() == input.len()` across 512 generated cases. Set 512 explicitly via `#![proptest_config(ProptestConfig { cases: 512, .. ProptestConfig::default() })]` — [proptest's default is 256](https://docs.rs/proptest/latest/proptest/test_runner/struct.Config.html).
5. A `report.json` fixture committed alongside the golden files, showing the totality invariant holds through every stage.
6. A short `README.md` explaining the trade-offs you chose: `Vec<Result<_, _>>` versus `DeserializeRecordsIter`, when buffering is required for multi-pass transforms like sort and dedup, why [`impl Write`](https://doc.rust-lang.org/std/io/trait.Write.html) belongs in function signatures instead of a concrete `File`, and when `#[from]` belongs on an error variant versus explicit construction.

## Evaluation Criteria

### Advanced

- The totality invariant is asserted at the exit of `extract` (`ok.len() + errors.len() == rows_read`), `transform` (`ok.len() + errors.len() == input_len`, row-aligned through `Vec<InvalidRow>`), and `load` (`report.ok + report.skip.len() == records.len()` on the `Ok` path) — three separate assertions, one per stage — and a final pipeline-level reconciliation shows every input row either shipped to both sinks, captured in `extract.errors`, captured in `transform.errors`, or captured in `load.skip`.
- `run_pipeline` is generic over `R: Read`, `W1: Write`, `W2: Write` so integration tests drive it with `&[u8]` and `Vec<u8>` on the same compiled body that production uses with [`File::open`](https://doc.rust-lang.org/std/fs/struct.File.html#method.open) and [`File::create`](https://doc.rust-lang.org/std/fs/struct.File.html#method.create), and the learner cites this abstraction by name.
- `EtlError` uses `#[from]` exclusively for library errors (`io::Error`, `csv::Error`, `serde_json::Error`), explicit construction for the domain `SkipThresholdExceeded { skip_rate }` variant, and every `?` in the pipeline body compiles into a typed error path — no `Box<dyn Error>`, no stringly-typed errors.
- The proptest property drives the real `transform` (not a mock), covers 512 cases via the explicit `ProptestConfig` override of [the 256-case default](https://docs.rs/proptest/latest/proptest/test_runner/struct.Config.html), and the learner demonstrates shrinking by committing a `buggy_transform` variant that drops empty-name rows and showing proptest converges to a 1-row counterexample.
- Exit code semantics match orchestrator expectations: 0 = full success, 1 = partial failure (most common real-world state), 2 = total failure, returned via [`ExitCode`](https://doc.rust-lang.org/std/process/struct.ExitCode.html), and the learner can name which Airflow retry policy applies to each.

### Intermediate

- `extract`, `transform`, and `load_both` are defined with the correct signatures and return `ExtractReport`, `TransformReport`, and `Result<LoadReport, EtlError>` rather than panicking or short-circuiting.
- The dollars-to-cents and kilograms-to-grams conversions use integer normalization at the transform boundary, and the `EnrichedFruit` struct exposes `price_cents: u64` rather than keeping `f64` through the pipeline.
- The `cents_per_kg` computation is **parenthesized correctly** as `(price_cents * 1000) / (weight_grams as u64)` so integer division does not silently zero the value.
- The thin `main` pattern holds: the orchestrator contains only file I/O, a single `run_pipeline` call, a `PipelineReport` print, and an `ExitCode` return.
- Integration tests read a checked-in CSV fixture and compare the NDJSON output byte-for-byte against a golden file with `assert_eq!`.
- stdout and stderr are separated: data on stdout (or into the sink writers), diagnostics and the `PipelineReport` on stderr.

### Beginner

- The pipeline runs end-to-end against a clean CSV but panics or short-circuits on the first malformed row, losing the totality contract.
- `main` contains business logic (the extract loop, the validation rules, or the sink writes) and the library surface is untestable without running the binary.
- Error handling is `Box<dyn Error>`, `Result<_, String>`, or `unwrap()` rather than a typed `thiserror` enum.
- Transform collects a flat `Vec<ValidationError>` (breaking the row-level invariant when one row produces multiple errors), or `cents_per_kg` is computed without parens and returns `0` for every realistic row.
- The pipeline produces NDJSON but not CSV, or writes CSV without calling `flush()` explicitly and leaves the truncation risk the Module 3.1.2 lesson warned about.
- No proptest property exists, or the only test is a single hand-crafted happy-path unit test that never exercises the Row-847 pattern.

## Share Your Work

Add this capstone to your LinkedIn profile as a portfolio project:

1. Go to your LinkedIn profile and select **Add profile section** > **Projects**
2. Title: "ETL Pipelines with Rust — Rust for Data Engineering Specialization"
3. Description: Summarize the project deliverables and key skills demonstrated
4. Link: Include the URL to your completed GitHub repository

## References

Rust documentation for this capstone:
- [`std::io::Read`](https://doc.rust-lang.org/std/io/trait.Read.html), [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html), [`ops::Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html)
- [`std::process::ExitCode`](https://doc.rust-lang.org/std/process/struct.ExitCode.html)
- [`f64::round`](https://doc.rust-lang.org/std/primitive.f64.html#method.round), [numeric cast (`as`) behavior](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast), [operator precedence](https://doc.rust-lang.org/reference/expressions/operator-expr.html)
- [`csv::ReaderBuilder`](https://docs.rs/csv/latest/csv/struct.ReaderBuilder.html), [`csv::WriterBuilder`](https://docs.rs/csv/latest/csv/struct.WriterBuilder.html), [`csv::Writer`](https://docs.rs/csv/latest/csv/struct.Writer.html)
- [`serde_json::to_string`](https://docs.rs/serde_json/latest/serde_json/fn.to_string.html), [`from_str`](https://docs.rs/serde_json/latest/serde_json/fn.from_str.html)
- [`thiserror` crate](https://docs.rs/thiserror/), [`#[derive(Error)]`](https://docs.rs/thiserror/latest/thiserror/derive.Error.html), [`From`](https://doc.rust-lang.org/std/convert/trait.From.html)
- [`proptest` crate](https://docs.rs/proptest/), [`proptest!` macro](https://docs.rs/proptest/latest/proptest/macro.proptest.html), [`ProptestConfig`](https://docs.rs/proptest/latest/proptest/test_runner/struct.Config.html)
