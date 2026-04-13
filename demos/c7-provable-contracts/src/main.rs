use c7_provable_contracts::{FruitBatch, FruitItem, Pipeline, PriceCents, Quantity};

fn main() {
    let batch = FruitBatch {
        items: vec![
            FruitItem {
                name: "Apple".into(),
                price: PriceCents::new(150).unwrap(),
                qty: Quantity::new(40),
            },
            FruitItem {
                name: "Banana".into(),
                price: PriceCents::new(75).unwrap(),
                qty: Quantity::new(100),
            },
            FruitItem {
                name: "Cherry".into(),
                price: PriceCents::new(300).unwrap(),
                qty: Quantity::new(25),
            },
            FruitItem {
                name: "Date".into(),
                price: PriceCents::new(500).unwrap(),
                qty: Quantity::new(10),
            },
            FruitItem {
                name: "Elderberry".into(),
                price: PriceCents::new(800).unwrap(),
                qty: Quantity::new(5),
            },
        ],
    };

    let total_before = batch.total_items();
    let revenue_before = batch.total_revenue();
    println!("=== Fruit Pipeline (typestate demo) ===\n");
    println!("Before: {total_before} items, revenue {revenue_before} cents");

    // Configured -> Validated -> Processing -> Complete
    let result = Pipeline::new(batch)
        .validate()
        .expect("validation failed")
        .process()
        .complete()
        .into_batch();

    let total_after = result.total_items();
    let revenue_after = result.total_revenue();
    println!("After:  {total_after} items, revenue {revenue_after} cents (10% discount)");
    assert_eq!(total_before, total_after, "invariant: no fruit lost");
    println!("\nInvariant holds: total_items preserved across pipeline.");
}
