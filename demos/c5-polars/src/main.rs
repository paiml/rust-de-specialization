use c5_polars::{analyze_sales, build_sample_df, top_fruits};
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = build_sample_df();
    println!("=== Fruit Sales Data ===");
    println!("{df}\n");

    let summary = analyze_sales(df.clone().lazy())?;
    println!("=== Sales by Category ===");
    println!("{summary}\n");

    let top = top_fruits(df.lazy(), 3)?;
    println!("=== Top 3 Fruits by Revenue ===");
    println!("{top}");

    Ok(())
}
