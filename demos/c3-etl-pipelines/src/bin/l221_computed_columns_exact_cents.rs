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
    // 0.29 * 100 is 28.999999999999996: truncating would give 28, rounding gives 29.
    assert_eq!(price_cents(0.29), 29);
    assert_eq!(weight_grams(0.18), 180);
    assert!((margin_pct(150, 100) - 33.333).abs() < 0.01);
    println!(
        "computed columns: {} cents, {} g",
        price_cents(1.50),
        weight_grams(0.12)
    );
}
