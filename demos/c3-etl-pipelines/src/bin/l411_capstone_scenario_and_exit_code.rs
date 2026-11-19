//! Capstone scenario numbers and exit-code rule, run for real.
//!
//! Provable contract: a 10,000-row feed with 153 malformed rows leaves
//! 9,847 valid records (10,000 - 153); the exit code is 0 only when
//! extract, transform and load are all clean; a csv deserialize error
//! carries the row position and names the offending field.
use csv::ReaderBuilder;
use serde::Deserialize;
use std::process::ExitCode;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct RawFruit {
    name: String,
    price: f64,
    category: String,
    weight_kg: f64,
}

fn exit_code(
    extract_errors: usize,
    transform_errors: usize,
    load_skips: usize,
    load_ok: usize,
) -> u8 {
    if extract_errors == 0 && transform_errors == 0 && load_skips == 0 {
        0
    } else if load_ok > 0 {
        1
    } else {
        2
    }
}

fn main() -> ExitCode {
    let mut csv = String::from("name,price,category,weight_kg\n");
    for i in 0..10_000 {
        if i % 65 == 0 && i / 65 < 153 {
            csv.push_str("OOPS,not_a_number,fruit,1.0\n");
        } else {
            csv.push_str(&format!("fruit{i},1.50,fruit,0.5\n"));
        }
    }
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(false)
        .from_reader(csv.as_bytes());
    let (mut ok, mut errs, mut rows_read) = (0usize, 0usize, 0usize);
    let mut first_err = String::new();
    for r in rdr.deserialize::<RawFruit>() {
        rows_read += 1;
        match r {
            Ok(_) => ok += 1,
            Err(e) => {
                if first_err.is_empty() {
                    first_err = e.to_string();
                }
                errs += 1;
            }
        }
    }
    println!("rows_read={rows_read} ok={ok} errors={errs}");
    println!("first csv error: {first_err}");
    assert_eq!(ok + errs, rows_read, "totality");
    assert_eq!((rows_read, errs, ok), (10_000, 153, 9_847));
    assert!(
        first_err.contains("record 1") && first_err.contains("field 1"),
        "error names row and field"
    );
    // Exit-code rule: extract.errors must count (the old rule ignored it).
    assert_eq!(exit_code(0, 0, 0, 9_847), 0);
    assert_eq!(
        exit_code(153, 0, 0, 9_847),
        1,
        "extract.errors alone is a partial failure"
    );
    assert_eq!(exit_code(0, 2, 0, 5), 1);
    assert_eq!(exit_code(0, 0, 1, 5), 1);
    assert_eq!(exit_code(1, 0, 0, 0), 2);
    println!("contract: capstone scenario numbers and exit-code rule OK");
    ExitCode::SUCCESS
}
