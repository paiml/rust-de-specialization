# Run the ETL Pipeline Contracts in Your Browser

Each section below is one lesson's contract, shrunk to a single file that runs on the Rust Playground with no install. Every section names its **Contract**, asserts it with `assert!` or `assert_eq!`, and links to a gist that was run on play.rust-lang.org (exit 0) before this reading shipped. Click **Run**, then break the contract on purpose and watch the assertion fail.

Each snippet uses the standard library alone, so it runs on the Playground as-is. The full `csv` + `serde` pipeline is in the capstone reading and the demo crate.

## Section 1 — Lesson 1.2.1: garbage row does not abort

**Contract:** One garbage row is rejected with its row number while every other row still loads.

```rust
#[derive(Debug)]
struct Row {
    name: String,
    price: f64,
}

fn parse(line: &str) -> Result<Row, String> {
    let (name, price) = line.split_once(',').ok_or("missing comma")?;
    let price = price
        .trim()
        .parse::<f64>()
        .map_err(|e| format!("bad price {price:?}: {e}"))?;
    Ok(Row {
        name: name.to_string(),
        price,
    })
}

fn main() {
    let input = "apple,1.50\nbanana,0.30\nOOPS,not_a_number\ncherry,6.00";
    let mut good = Vec::new();
    let mut errors = Vec::new();
    for (i, line) in input.lines().enumerate() {
        match parse(line) {
            Ok(r) => good.push(r),
            Err(e) => errors.push((i + 1, e)),
        }
    }
    assert_eq!((good[0].name.as_str(), good[2].price), ("apple", 6.0));
    assert_eq!(good.len(), 3, "one garbage row must not abort the run");
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].0, 3, "the error names the offending row number");
    println!(
        "garbage row: {} good, row {} rejected",
        good.len(),
        errors[0].0
    );
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=df234901c499253dc52882f7d731b97b>

Expected output contains: `garbage row: 3 good, row 3 rejected`

## Section 2 — Lesson 1.2.3: read trait one code path

**Contract:** One function generic over Read gives the same answer for a byte slice and an owned cursor.

```rust
use std::io::{BufRead, BufReader, Read};

fn count_rows(source: impl Read) -> usize {
    BufReader::new(source)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .count()
}

fn main() {
    let bytes: &[u8] = b"apple,1.50\nbanana,0.30\ncherry,6.00\n";
    // One generic function, two sources: a byte slice and an owned cursor.
    let from_slice = count_rows(bytes);
    let from_cursor = count_rows(std::io::Cursor::new(bytes.to_vec()));
    assert_eq!(from_slice, 3);
    assert_eq!(
        from_slice, from_cursor,
        "every Read source must give the same answer"
    );
    println!("read trait: {from_slice} rows from both sources");
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=d19bc9c07ddd6a0aced4a30eb18d7335>

Expected output contains: `read trait: 3 rows from both sources`

## Section 3 — Lesson 2.1.1: totality accepted plus rejected

**Contract:** Accepted plus rejected always equals the input row count; no row is silently dropped.

```rust
fn parse_price(s: &str) -> Result<f64, String> {
    let p: f64 = s.parse().map_err(|_| format!("bad price {s:?}"))?;
    if p > 0.0 {
        Ok(p)
    } else {
        Err(format!("non-positive price {p}"))
    }
}

struct Report {
    accepted: Vec<f64>,
    rejected: Vec<(usize, String)>,
}

fn run(rows: &[&str]) -> Report {
    let mut r = Report {
        accepted: Vec::new(),
        rejected: Vec::new(),
    };
    for (i, row) in rows.iter().enumerate() {
        match parse_price(row) {
            Ok(p) => r.accepted.push(p),
            Err(e) => r.rejected.push((i, e)),
        }
    }
    r
}

fn main() {
    let rows = ["1.50", "oops", "0.30", "0", "-2", "6.00"];
    let r = run(&rows);
    assert_eq!(
        r.accepted.len() + r.rejected.len(),
        rows.len(),
        "totality: accepted + rejected must equal input"
    );
    println!(
        "totality: {} + {} = {}",
        r.accepted.len(),
        r.rejected.len(),
        rows.len()
    );
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=2b1a1c42a47266c97cf0976a56390a21>

Expected output contains: `totality: 3 + 3 = 6`

## Section 4 — Lesson 2.1.2: validation reports all defects

**Contract:** Validation reports every defect on a row, not only the first.

```rust
#[derive(Debug, PartialEq)]
enum Defect {
    EmptyCategory,
    ZeroPrice,
}

fn validate(category: &str, price_cents: u64) -> Vec<Defect> {
    let mut defects = Vec::new();
    if category.trim().is_empty() {
        defects.push(Defect::EmptyCategory);
    }
    if price_cents == 0 {
        defects.push(Defect::ZeroPrice);
    }
    defects
}

fn main() {
    assert!(validate("pome", 150).is_empty());
    assert_eq!(validate("", 150), vec![Defect::EmptyCategory]);
    assert_eq!(validate("pome", 0), vec![Defect::ZeroPrice]);
    // A row with two defects reports both, not just the first.
    assert_eq!(
        validate("  ", 0),
        vec![Defect::EmptyCategory, Defect::ZeroPrice]
    );
    println!("validation: all defects reported per row");
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=038c9d725d1f849d0d2cb27833237c69>

Expected output contains: `validation: all defects reported per row`

## Section 5 — Lesson 2.1.3: normalization idempotent

**Contract:** Normalizing a value twice gives the same result as normalizing it once.

```rust
fn normalize_name(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn normalize_category(s: &str) -> String {
    s.trim().to_lowercase()
}

fn main() {
    assert_eq!(normalize_name("  Green   APPLE "), "green apple");
    assert_eq!(normalize_category(" Tropical\t"), "tropical");
    // Idempotence: normalizing twice equals normalizing once.
    for raw in ["  Green   APPLE ", "banana", "  ", "MiXeD  Case"] {
        let once = normalize_name(raw);
        assert_eq!(
            normalize_name(&once),
            once,
            "normalize must be idempotent for {raw:?}"
        );
    }
    println!("normalize: idempotent");
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=feb76679b1fbe7a80e09b280c8291d1b>

Expected output contains: `normalize: idempotent`

## Section 6 — Lesson 2.2.1: computed columns exact cents

**Contract:** Computed columns round to exact integer cents and grams so float noise like 1.10*100 cannot leak into the output.

```rust
fn price_cents(price: f64) -> u64 {
    (price * 100.0).round() as u64
}

fn weight_grams(kg: f64) -> u32 {
    (kg * 1000.0).round() as u32
}

fn margin_pct(price_cents: u64, cost_cents: u64) -> f64 {
    (price_cents as f64 - cost_cents as f64) / price_cents as f64 * 100.0
}

fn main() {
    // 1.10 * 100 is 110.00000000000001 in f64: rounding keeps it exact.
    assert_eq!(price_cents(1.10), 110);
    assert_eq!(price_cents(0.30), 30);
    assert_eq!(weight_grams(0.18), 180);
    assert!((margin_pct(150, 100) - 33.333).abs() < 0.01);
    println!(
        "computed columns: {} cents, {} g",
        price_cents(1.50),
        weight_grams(0.12)
    );
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e2918d57ae63a6f6a8d482ae947a8627>

Expected output contains: `computed columns: 150 cents, 120 g`

## Contract summary

| # | Lesson | Contract | Enforced by | Playground output |
|---|--------|----------|-------------|-------------------|
| 1 | 1.2.1 | garbage row does not abort | `assert!`/`assert_eq!` in the snippet | `garbage row: 3 good, row 3 rejected` |
| 2 | 1.2.3 | read trait one code path | `assert!`/`assert_eq!` in the snippet | `read trait: 3 rows from both sources` |
| 3 | 2.1.1 | totality accepted plus rejected | `assert!`/`assert_eq!` in the snippet | `totality: 3 + 3 = 6` |
| 4 | 2.1.2 | validation reports all defects | `assert!`/`assert_eq!` in the snippet | `validation: all defects reported per row` |
| 5 | 2.1.3 | normalization idempotent | `assert!`/`assert_eq!` in the snippet | `normalize: idempotent` |
| 6 | 2.2.1 | computed columns exact cents | `assert!`/`assert_eq!` in the snippet | `computed columns: 150 cents, 120 g` |
