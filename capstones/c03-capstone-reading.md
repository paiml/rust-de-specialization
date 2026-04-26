# Capstone Reading: A Minimal End-to-End ETL Pipeline

This reading walks through a complete, Playground-runnable ETL pipeline that integrates every stage from this course into roughly 270 lines of Rust. Open it on the Rust Playground at `https://play.rust-lang.org` (Tools menu, Share), paste the concatenated code blocks into `src/main.rs`, add the four listed dependencies to `Cargo.toml` (via the Playground's dependency picker), and run. Every snippet below is real, compilable Rust — no pseudocode, no omitted bodies.

The pipeline follows the same shape as the capstone brief: one input stream, two output sinks, a composed typed report, and the totality invariant enforced at every stage boundary.

## Dependencies

```toml
[dependencies]
csv = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
```

All four are supported out of the box by the Rust Playground. [`thiserror`](https://docs.rs/thiserror/) is what lets us build a typed `EtlError` enum instead of the stringly-typed errors Module 4.1 warned against.

## Fixture data

We use a `&'static [u8]` literal for the input, not a file. This is not a convenience — it is the single most important architectural choice in the pipeline. The `extract` function takes [`impl Read`](https://doc.rust-lang.org/std/io/trait.Read.html), which means the same compiled body runs against a byte slice in a test, [`io::stdin().lock()`](https://doc.rust-lang.org/std/io/struct.Stdin.html#method.lock) in production, and [`File::open("fruit.csv")?`](https://doc.rust-lang.org/std/fs/struct.File.html#method.open) in an Airflow job. One code path, three sources. This is the lesson from Module 1.2.3 (Files, Stdin, Byte Slices: The Read Trait) made concrete.

The fixture deliberately includes four row types: two clean rows, one "Row 847" parse failure (`not_a_number` in the price column), one transform-stage rejection with multiple defects (empty category *and* zero price — proving row-aligned accumulation), and one boundary case.

```rust
const FRUIT_CSV: &[u8] = b"\
name,price,category,weight_kg,organic
apple,1.50,pome,0.18,true
banana,0.30,tropical,0.12,false
OOPS,not_a_number,tropical,0.25,true
mango,2.75,,0.40,true
cherry,0.00,stone,0.01,true
grape,3.20,berry,0.50,false
";
```

Six input rows: three valid (apple, banana, grape), one extract error (OOPS), two transform rejections (mango with empty category, cherry with price ≤ 0). Note each row's expected fate as you read — we will verify it at the end.

## Typed errors

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum EtlError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}
```

Every `?` in the pipeline below propagates through this enum via the [auto-generated `From` impls](https://doc.rust-lang.org/std/convert/trait.From.html) that [thiserror's `#[derive(Error)]`](https://docs.rs/thiserror/latest/thiserror/derive.Error.html) produces from each `#[from]` attribute. No `Box<dyn Error>`, no stringly-typed errors — the compiler checks every failure path.

## Part 1: Extract

The extract stage reads bytes, types them into `RawFruit`, and returns an `ExtractReport` that lists every outcome. No row vanishes: a parse failure becomes a typed `ParseError` entry in `errors`, never a panic.

Three things are worth noticing. First, `RawFruit` derives `Deserialize` (Module 1.1.3) so [csv](https://docs.rs/csv/) generates the field-by-field visitor code at compile time — you never write a hand-rolled parser. Second, [`ReaderBuilder`](https://docs.rs/csv/latest/csv/struct.ReaderBuilder.html) is configured with `.has_headers(true)` and `.flexible(false)` (Module 1.1.2); `.flexible(false)` is the data-quality lever that rejects wrong-column-count rows instead of silently accepting them. Third, the accumulation loop pushes every row into either `ok` or `errors` without `unwrap()` or `?` — the totality invariant `ok.len() + errors.len() == rows_read` is asserted at the function exit.

```rust
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

#[derive(Debug, Deserialize)]
struct RawFruit {
    name: String,
    price: f64,
    category: String,
    weight_kg: f64,
    organic: Option<bool>,
}

#[derive(Debug)]
struct ParseError {
    row: usize,
    cause: String,  // source's Display message; typed wrapping also works
}

#[derive(Debug, Default)]
struct ExtractReport {
    ok: Vec<RawFruit>,
    errors: Vec<ParseError>,
    rows_read: usize,
}

fn extract<R: Read>(r: R) -> ExtractReport {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .flexible(false)
        .from_reader(r);

    let mut report = ExtractReport::default();
    for result in rdr.deserialize::<RawFruit>() {
        report.rows_read += 1;
        match result {
            Ok(row) => report.ok.push(row),
            Err(e) => report.errors.push(ParseError {
                row: report.rows_read,
                cause: e.to_string(),
            }),
        }
    }

    // Totality invariant at the extract boundary (Module 2.1.1).
    assert_eq!(report.ok.len() + report.errors.len(), report.rows_read);
    report
}
```

## Part 2: Transform

The transform stage takes typed `RawFruit` rows from extract and applies validation plus field normalization. The output type `EnrichedFruit` exposes `price_cents: u64` and `weight_grams: u32` — floating-point money is gone by the time the pipeline reaches the load stage.

The conversions are `(price * 100.0).`[`round()`](https://doc.rust-lang.org/std/primitive.f64.html#method.round)` as u64` for dollars to cents and `(weight_kg * 1000.0).round() as u32` for kilograms to grams (Module 2.1.3). The `.round()` is load-bearing: [`as` casts from float to int round toward zero](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast), so `0.1 * 1000.0 = 99.999...` silently truncates to `99` without it.

The validation pattern is **row-aligned accumulate-over-short-circuit**: a row with multiple defects produces a single `InvalidRow` carrying a `Vec<ValidationError>` of *all* its failures — mango's empty category and cherry's zero price are each reported once with full detail, but `report.errors.len()` counts **failed rows**, not individual errors. That is what keeps the invariant `ok.len() + errors.len() == input_len` holding at the row level.

Note the `cents_per_kg` formula: `(price_cents * 1000) / (weight_grams as u64)`. The parentheses matter. [Rust binds `as` tighter than `*` and `/`](https://doc.rust-lang.org/reference/expressions/operator-expr.html), so swapping the order to `price_cents / weight_grams as u64 * 1000` would do integer division first and silently return `0` for every fruit under a kilogram.

```rust
#[derive(Debug, Serialize)]
struct EnrichedFruit {
    name: String,
    price_cents: u64,
    category: String,
    weight_grams: u32,
    organic: bool,
    cents_per_kg: u64,
}

#[derive(Debug)]
struct ValidationError {
    field: &'static str,
    reason: String,
}

#[derive(Debug)]
struct InvalidRow {
    row: RawFruit,
    errors: Vec<ValidationError>,
}

fn validate(row: &RawFruit) -> Vec<ValidationError> {
    let mut errs = Vec::new();
    if row.name.trim().is_empty() {
        errs.push(ValidationError { field: "name", reason: "empty".into() });
    }
    if row.category.trim().is_empty() {
        errs.push(ValidationError { field: "category", reason: "empty".into() });
    }
    if row.price <= 0.0 {
        errs.push(ValidationError {
            field: "price",
            reason: format!("must be > 0, got {}", row.price),
        });
    }
    if row.weight_kg <= 0.0 {
        errs.push(ValidationError {
            field: "weight_kg",
            reason: format!("must be > 0, got {}", row.weight_kg),
        });
    }
    errs
}

fn enrich(row: RawFruit) -> EnrichedFruit {
    // Integer-cents conversion eliminates the 0.1 + 0.2 == 0.30000000000000004 class.
    let price_cents = (row.price * 100.0).round() as u64;
    let weight_grams = (row.weight_kg * 1000.0).round() as u32;
    // Computed column (Module 2.2.1). Parentheses REQUIRED — `as` binds tighter
    // than `*` and `/`, so omitting them reorders the arithmetic and returns 0.
    let cents_per_kg = if weight_grams > 0 {
        (price_cents * 1000) / (weight_grams as u64)
    } else {
        0
    };
    EnrichedFruit {
        name: row.name,
        price_cents,
        category: row.category,
        weight_grams,
        organic: row.organic.unwrap_or(false),
        cents_per_kg,
    }
}

#[derive(Debug, Default)]
struct TransformReport {
    ok: Vec<EnrichedFruit>,
    errors: Vec<InvalidRow>,   // row-aligned: one entry per failed input row
    input_len: usize,
}

fn transform(input: Vec<RawFruit>) -> TransformReport {
    let mut report = TransformReport { input_len: input.len(), ..Default::default() };
    for row in input {
        let errors = validate(&row);
        if errors.is_empty() {
            report.ok.push(enrich(row));
        } else {
            // All errors for this row stay together in one InvalidRow.
            report.errors.push(InvalidRow { row, errors });
        }
    }
    // Row-aligned totality: ok.len() + errors.len() == input_len, always.
    assert_eq!(report.ok.len() + report.errors.len(), report.input_len);
    report
}
```

## Part 3: Load

The load stage writes every `EnrichedFruit` to two sinks — NDJSON and CSV — in one pass over the slice. Both functions take [`&mut impl Write`](https://doc.rust-lang.org/std/io/trait.Write.html) so the caller can hand them [`File::create`](https://doc.rust-lang.org/std/fs/struct.File.html#method.create), [`io::stdout().lock()`](https://doc.rust-lang.org/std/io/struct.Stdout.html#method.lock), or `Vec<u8>` in a test (Module 3.2.3). The NDJSON writer calls [`serde_json::to_string`](https://docs.rs/serde_json/latest/serde_json/fn.to_string.html) per record and separates with `\n` (Module 3.1.1); the CSV writer uses the `Serialize` derive to auto-generate the header row from the `EnrichedFruit` field order (Module 3.1.2).

Critically, the CSV writer calls `flush()` explicitly before returning. Relying on [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) to flush silently swallows I/O errors — `fn drop(&mut self)` has no return type — and that is the truncation bug Module 3.1.2 warned against. An explicit `?` on `flush()` makes I/O errors a typed `EtlError` the caller must handle.

```rust
#[derive(Debug, Default)]
struct LoadReport {
    ok: usize,
    // skip stays empty in the reading; the capstone exercises skip-and-continue.
    skip: Vec<SkipRecord>,
}

#[derive(Debug)]
struct SkipRecord { row: usize, cause: String }

fn write_ndjson<W: Write>(w: &mut W, rows: &[EnrichedFruit]) -> Result<(), EtlError> {
    for row in rows {
        let json = serde_json::to_string(row)?;
        writeln!(w, "{}", json)?;
    }
    w.flush()?;
    Ok(())
}

fn write_csv<W: Write>(w: &mut W, rows: &[EnrichedFruit]) -> Result<(), EtlError> {
    let mut wtr = csv::WriterBuilder::new().from_writer(w);
    for row in rows {
        wtr.serialize(row)?;
    }
    // Explicit flush — never rely on Drop (Module 3.1.2).
    wtr.flush()?;
    Ok(())
}
```

## Part 4: Wire it all together

The orchestrator calls `extract`, `transform`, then both sinks, and composes a typed `PipelineReport` from the three stage reports.

```rust
#[derive(Debug)]
struct PipelineReport {
    extract: ExtractReport,
    transform: TransformReport,
    load: LoadReport,
}

fn run_pipeline<R: Read, W1: Write, W2: Write>(
    r: R,
    ndjson_sink: &mut W1,
    csv_sink: &mut W2,
) -> Result<PipelineReport, EtlError> {
    // Destructure so we can move `ok` into transform while retaining `errors` and `rows_read`.
    let ExtractReport { ok: raw_rows, errors: extract_errors, rows_read } = extract(r);

    let transform_rep = transform(raw_rows);

    write_ndjson(ndjson_sink, &transform_rep.ok)?;
    write_csv(csv_sink, &transform_rep.ok)?;

    let load_rep = LoadReport {
        ok: transform_rep.ok.len(),
        skip: Vec::new(),   // reading stays simple; capstone exercises skip-and-continue
    };

    Ok(PipelineReport {
        extract: ExtractReport {
            ok: Vec::new(),            // raw rows moved into transform; summary-only here
            errors: extract_errors,
            rows_read,
        },
        transform: transform_rep,
        load: load_rep,
    })
}
```

`main` is deliberately thin: it owns the input bytes, calls the library, prints the composed report to stderr, and returns an [`ExitCode`](https://doc.rust-lang.org/std/process/struct.ExitCode.html). Zero business logic lives here — that is the separation that keeps `run_pipeline` independently testable with `&[u8]` and `&mut Vec<u8>`.

```rust
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut ndjson_buf: Vec<u8> = Vec::new();
    let mut csv_buf: Vec<u8> = Vec::new();

    match run_pipeline(FRUIT_CSV, &mut ndjson_buf, &mut csv_buf) {
        Ok(report) => {
            eprintln!("──── report ────");
            eprintln!("extract   : {} ok / {} errors / {} rows_read",
                report.extract.ok.len(), report.extract.errors.len(), report.extract.rows_read);
            eprintln!("transform : {} ok / {} errors (input_len={})",
                report.transform.ok.len(), report.transform.errors.len(), report.transform.input_len);
            eprintln!("load      : {} ok / {} skip",
                report.load.ok, report.load.skip.len());

            // Pipeline-level totality: every input row is accounted for.
            let accounted = report.load.ok
                + report.extract.errors.len()
                + report.transform.errors.len();
            assert_eq!(accounted, report.extract.rows_read, "pipeline totality violated");
            eprintln!("totality  : {} == {} ✓", accounted, report.extract.rows_read);

            println!("──── NDJSON output ────");
            println!("{}", String::from_utf8_lossy(&ndjson_buf));
            println!("──── CSV output ────");
            println!("{}", String::from_utf8_lossy(&csv_buf));

            // Exit code semantics (Module 4.1.1).
            if report.extract.errors.is_empty() && report.transform.errors.is_empty() {
                ExitCode::SUCCESS                 // 0: every row made it through
            } else if report.load.ok > 0 {
                ExitCode::from(1)                 // 1: partial failure
            } else {
                ExitCode::from(2)                 // 2: total failure
            }
        }
        Err(e) => {
            eprintln!("FATAL: {e}");
            ExitCode::from(2)
        }
    }
}
```

Run the program. The expected behavior on the fixture above: extract reports one error (`OOPS` / `not_a_number`), transform reports two errors (mango with empty category, cherry with zero price), and the two sinks each write three records (apple, banana, grape). The totality assertion prints `6 == 6 ✓` and the process exits with code `1` — partial failure, which is the state Airflow will retry tomorrow with tomorrow's batch.

## Why this matters

The incumbent Python pipeline would have crashed on row 3 of our fixture. The `OOPS` value would raise a `ValueError` inside `float("not_a_number")`, the process would exit non-zero, and the five rows behind it — three of which are valid — would never reach either sink. That is what "silent data loss" looks like in the field: nobody notices until the quarterly reconciliation comes up short.

The Rust pipeline does something different at every boundary. Extract accumulates the `OOPS` row as a typed `ParseError` and moves on. Transform accumulates the mango and cherry rows as `InvalidRow` entries — **one entry per failed row, carrying a `Vec<ValidationError>` of every defect that row had** — and moves on. Load writes every surviving record to both sinks and returns a typed `LoadReport`. The operator reads the composed report on stderr and knows exactly which three rows need upstream attention; the three valid rows are already in the analytics team's hands. The process exits with `ExitCode::from(1)`, and tomorrow's Airflow run will retry the upstream fetch.

That is the totality invariant doing structural work: `ok.len() + errors.len() == input.len()`, row-aligned, asserted at every stage boundary, never a comment. The pipeline cannot silently drop a row because the type system does not let it — every input row lands in exactly one of `ok`, `errors`, or (at load) `skip`, and the assertion at each stage exit proves it. Extend this shape to ten thousand rows or ten million, the invariant costs nothing, and the on-call rotation can finally sleep through Wednesday nights.
