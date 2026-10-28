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
