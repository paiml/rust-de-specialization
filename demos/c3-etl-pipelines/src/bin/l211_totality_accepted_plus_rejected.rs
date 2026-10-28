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
