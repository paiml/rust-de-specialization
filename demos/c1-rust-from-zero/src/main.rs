use c1_rust_from_zero::{filter_above, parse_csv, sum_by_category, to_json_lines};

fn main() {
    let csv = "\
apple,1.50,fruit
banana,0.75,fruit
carrot,1.20,vegetable
broccoli,1.80,vegetable
bad_row
grape,2.50,fruit";

    let (fruits, errors) = parse_csv(csv);
    println!("Parsed {} fruits, {} errors", fruits.len(), errors.len());
    for e in &errors {
        eprintln!("  skip: {e}");
    }

    println!("\n--- Fruits above $1.00 ---");
    for f in filter_above(&fruits, 100) {
        println!(
            "  {} = ${:.2} ({})",
            f.name,
            f.price_cents as f64 / 100.0,
            f.category
        );
    }

    println!("\n--- Totals by category ---");
    for (cat, cents) in sum_by_category(&fruits) {
        println!("  {cat}: ${:.2}", cents as f64 / 100.0);
    }

    println!("\n--- JSON ---");
    match to_json_lines(&fruits) {
        Ok(json) => println!("{json}"),
        Err(e) => eprintln!("JSON error: {e}"),
    }
}
