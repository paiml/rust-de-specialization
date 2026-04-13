//! Course 1: Rust from Zero — ownership, borrowing, iterators, error handling.
//!
//! Contract: `contracts/fruit-parser-v1.yaml`
//! Theme: Parse a fruit CSV, aggregate by category, emit JSON.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("missing field: {0}")]
    MissingField(&'static str),
    #[error("invalid price: {0:?}")]
    InvalidPrice(String),
    #[error("price must be positive")]
    NonPositivePrice,
}

/// A parsed fruit record. `price_cents` is integer cents (no floats for money).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Fruit {
    pub name: String,
    pub price_cents: u64,
    pub category: String,
}

/// CONTRACT: parse_fruit
/// Parse a CSV line "name,price,category" → `Result<Fruit, ParseError>`.
/// Invariant: Ok(f) ⟹ f.name non-empty ∧ f.price_cents > 0.
pub fn parse_line(line: &str) -> Result<Fruit, ParseError> {
    let mut fields = line.splitn(3, ',');

    let name = fields
        .next()
        .filter(|s| !s.is_empty())
        .ok_or(ParseError::MissingField("name"))?
        .to_owned();

    let price_str = fields.next().ok_or(ParseError::MissingField("price"))?;
    let price: f64 = price_str
        .trim()
        .parse()
        .map_err(|_| ParseError::InvalidPrice(price_str.to_owned()))?;

    let price_cents = (price * 100.0).round() as u64;
    if price_cents == 0 {
        return Err(ParseError::NonPositivePrice);
    }

    let category = fields
        .next()
        .ok_or(ParseError::MissingField("category"))?
        .trim()
        .to_owned();

    Ok(Fruit {
        name,
        price_cents,
        category,
    })
}

/// CONTRACT: parse_batch
/// Invariant: ok.len() + err.len() == non-empty line count.
pub fn parse_csv(input: &str) -> (Vec<Fruit>, Vec<ParseError>) {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(parse_line)
        .fold((Vec::new(), Vec::new()), |(mut ok, mut err), r| {
            match r {
                Ok(fruit) => ok.push(fruit),
                Err(e) => err.push(e),
            }
            (ok, err)
        })
}

/// CONTRACT: sum_by_category
/// Invariant: total across all categories == fruits.iter().map(|f| f.price_cents).sum().
/// Output sorted by category (BTreeMap).
pub fn sum_by_category(fruits: &[Fruit]) -> Vec<(String, u64)> {
    let mut map: BTreeMap<String, u64> = BTreeMap::new();
    for f in fruits {
        *map.entry(f.category.clone()).or_insert(0) += f.price_cents;
    }
    map.into_iter().collect()
}

/// Filter fruits above a price threshold (borrows slice → owned vec).
pub fn filter_above(fruits: &[Fruit], min_cents: u64) -> Vec<Fruit> {
    fruits
        .iter()
        .filter(|f| f.price_cents > min_cents)
        .cloned()
        .collect()
}

/// Serialize to newline-delimited JSON.
pub fn to_json_lines(fruits: &[Fruit]) -> Result<String, serde_json::Error> {
    fruits
        .iter()
        .map(serde_json::to_string)
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_line() {
        let f = parse_line("apple,1.50,fruit").unwrap();
        assert_eq!(f.name, "apple");
        assert_eq!(f.price_cents, 150);
        assert_eq!(f.category, "fruit");
    }

    #[test]
    fn parse_missing_category() {
        assert!(parse_line("apple,1.5").is_err());
    }

    #[test]
    fn parse_invalid_price() {
        assert!(parse_line("apple,abc,fruit").is_err());
    }

    #[test]
    fn parse_empty_name() {
        assert!(parse_line(",1.5,fruit").is_err());
    }

    #[test]
    fn parse_zero_price() {
        assert!(parse_line("apple,0.00,fruit").is_err());
    }

    // CONTRACT: parse_batch totality
    #[test]
    fn batch_totality() {
        let input = "apple,1.50,fruit\nbad\nbanana,2.00,fruit";
        let (ok, err) = parse_csv(input);
        let line_count = input.lines().filter(|l| !l.is_empty()).count();
        assert_eq!(ok.len() + err.len(), line_count);
    }

    // CONTRACT: sum_by_category preservation
    #[test]
    fn sum_preserves_total() {
        let fruits = vec![
            Fruit {
                name: "a".into(),
                price_cents: 100,
                category: "x".into(),
            },
            Fruit {
                name: "b".into(),
                price_cents: 200,
                category: "x".into(),
            },
            Fruit {
                name: "c".into(),
                price_cents: 300,
                category: "y".into(),
            },
        ];
        let sums = sum_by_category(&fruits);
        let total: u64 = sums.iter().map(|(_, v)| v).sum();
        let expected: u64 = fruits.iter().map(|f| f.price_cents).sum();
        assert_eq!(total, expected);
    }

    #[test]
    fn sum_sorted_by_category() {
        let fruits = vec![
            Fruit {
                name: "a".into(),
                price_cents: 100,
                category: "zebra".into(),
            },
            Fruit {
                name: "b".into(),
                price_cents: 200,
                category: "apple".into(),
            },
        ];
        let sums = sum_by_category(&fruits);
        assert_eq!(sums[0].0, "apple");
        assert_eq!(sums[1].0, "zebra");
    }

    #[test]
    fn filter_threshold() {
        let fruits = vec![
            Fruit {
                name: "a".into(),
                price_cents: 100,
                category: "x".into(),
            },
            Fruit {
                name: "b".into(),
                price_cents: 500,
                category: "x".into(),
            },
        ];
        let filtered = filter_above(&fruits, 200);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "b");
    }

    #[test]
    fn json_round_trip() {
        let fruits = vec![Fruit {
            name: "apple".into(),
            price_cents: 150,
            category: "fruit".into(),
        }];
        let json = to_json_lines(&fruits).unwrap();
        let parsed: Fruit = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, fruits[0]);
    }
}
