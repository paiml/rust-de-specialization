use c3_etl_pipelines::{extract_csv, load_json, transform};

const FRUIT_CSV: &str = "\
name,price,category,weight_kg
Apple,1.50,fruit,0.2
Banana,0.75,fruit,0.15
Mango,3.25,tropical,0.4
Durian,-5.00,tropical,1.2
Mystery,2.00,unknown,0.0";

fn main() {
    let raw = extract_csv(FRUIT_CSV).expect("CSV parse failed");
    println!("Extracted {} raw records", raw.len());

    let (enriched, errors) = transform(raw);
    println!(
        "Transform: {} ok, {} errors (total {})",
        enriched.len(),
        errors.len(),
        enriched.len() + errors.len()
    );

    for e in &errors {
        eprintln!("  WARN: {e}");
    }

    let ndjson = load_json(&enriched).expect("JSON serialization failed");
    println!("\n--- NDJSON output ---");
    println!("{ndjson}");
}
