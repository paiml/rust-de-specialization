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
