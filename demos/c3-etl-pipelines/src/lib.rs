use serde::{Deserialize, Serialize};

// --- Types (fruit-etl-v1.yaml) ---

#[derive(Debug, Deserialize, Clone)]
pub struct RawFruit {
    pub name: String,
    pub price: f64,
    pub category: String,
    pub weight_kg: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EnrichedFruit {
    pub name: String,
    pub price_cents: u64,
    pub category: String,
    pub weight_grams: u32,
    pub margin_pct: f64,
}

#[derive(Debug, thiserror::Error)]
pub enum EtlError {
    #[error("CSV parse error: {0}")]
    CsvParse(String),
    #[error("Invalid price for '{name}': {price}")]
    InvalidPrice { name: String, price: f64 },
    #[error("Invalid weight for '{name}': {weight_kg}")]
    InvalidWeight { name: String, weight_kg: f64 },
    #[error("JSON serialization error: {0}")]
    JsonSerialize(#[from] serde_json::Error),
}

// --- Extract (contract: extract_csv) ---

pub fn extract_csv(input: &str) -> Result<Vec<RawFruit>, EtlError> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input.as_bytes());
    let mut fruits = Vec::new();
    for result in rdr.deserialize() {
        let fruit: RawFruit = result.map_err(|e| EtlError::CsvParse(e.to_string()))?;
        fruits.push(fruit);
    }
    Ok(fruits)
}

// --- Transform (contract: ok.len() + err.len() == raw.len()) ---

const MARGIN_PCT: f64 = 0.30;

pub fn transform(raw: Vec<RawFruit>) -> (Vec<EnrichedFruit>, Vec<EtlError>) {
    let mut ok = Vec::new();
    let mut errs = Vec::new();
    for r in raw {
        if r.price <= 0.0 {
            errs.push(EtlError::InvalidPrice {
                name: r.name,
                price: r.price,
            });
            continue;
        }
        if r.weight_kg <= 0.0 {
            errs.push(EtlError::InvalidWeight {
                name: r.name,
                weight_kg: r.weight_kg,
            });
            continue;
        }
        ok.push(EnrichedFruit {
            name: r.name,
            price_cents: (r.price * 100.0).round() as u64,
            category: r.category,
            weight_grams: (r.weight_kg * 1000.0).round() as u32,
            margin_pct: MARGIN_PCT,
        });
    }
    (ok, errs)
}

// --- Load (contract: newline-delimited JSON, round-trip safe) ---

pub fn load_json(fruits: &[EnrichedFruit]) -> Result<String, EtlError> {
    let mut lines = Vec::with_capacity(fruits.len());
    for f in fruits {
        lines.push(serde_json::to_string(f)?);
    }
    Ok(lines.join("\n"))
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CSV: &str = "\
name,price,category,weight_kg
Apple,1.50,fruit,0.2
Banana,0.75,fruit,0.15
Mango,3.25,tropical,0.4";

    // Contract: extract_csv deserializes CSV with header
    #[test]
    fn extract_parses_valid_csv() {
        let fruits = extract_csv(SAMPLE_CSV).unwrap();
        assert_eq!(fruits.len(), 3);
        assert_eq!(fruits[0].name, "Apple");
    }

    // Contract: ok.len() + err.len() == raw.len() (ETL totality)
    #[test]
    fn transform_totality() {
        let raw = extract_csv(SAMPLE_CSV).unwrap();
        let n = raw.len();
        let (ok, errs) = transform(raw);
        assert_eq!(ok.len() + errs.len(), n);
    }

    // Contract: no floats in output prices — integer cents
    #[test]
    fn transform_integer_cents() {
        let raw = extract_csv(SAMPLE_CSV).unwrap();
        let (ok, _) = transform(raw);
        assert_eq!(ok[0].price_cents, 150); // $1.50 -> 150 cents
        assert_eq!(ok[1].price_cents, 75);
        assert_eq!(ok[2].price_cents, 325);
    }

    // Contract: weight converted to grams
    #[test]
    fn transform_weight_grams() {
        let raw = extract_csv(SAMPLE_CSV).unwrap();
        let (ok, _) = transform(raw);
        assert_eq!(ok[0].weight_grams, 200);
    }

    // Contract: invalid price rejected
    #[test]
    fn transform_rejects_negative_price() {
        let csv = "name,price,category,weight_kg\nBad,-1.0,fruit,0.5";
        let raw = extract_csv(csv).unwrap();
        let (ok, errs) = transform(raw);
        assert!(ok.is_empty());
        assert_eq!(errs.len(), 1);
    }

    // Contract: invalid weight rejected
    #[test]
    fn transform_rejects_zero_weight() {
        let csv = "name,price,category,weight_kg\nBad,2.0,fruit,0.0";
        let raw = extract_csv(csv).unwrap();
        let (ok, errs) = transform(raw);
        assert!(ok.is_empty());
        assert_eq!(errs.len(), 1);
    }

    // Contract: JSON round-trip safe
    #[test]
    fn json_round_trip() {
        let raw = extract_csv(SAMPLE_CSV).unwrap();
        let (ok, _) = transform(raw);
        let json = load_json(&ok).unwrap();
        for line in json.lines() {
            let parsed: EnrichedFruit = serde_json::from_str(line).unwrap();
            assert!(ok.contains(&parsed));
        }
    }

    // Contract: empty input → empty output
    #[test]
    fn empty_csv_returns_empty() {
        let csv = "name,price,category,weight_kg\n";
        let raw = extract_csv(csv).unwrap();
        assert!(raw.is_empty());
        let (ok, errs) = transform(raw);
        assert!(ok.is_empty());
        assert!(errs.is_empty());
        let json = load_json(&ok).unwrap();
        assert!(json.is_empty());
    }

    // Contract: malformed CSV row produces CsvParse error
    #[test]
    fn extract_csv_parse_error() {
        let csv = "name,price,category,weight_kg\nBad,not_a_number,fruit,0.5";
        let err = extract_csv(csv).unwrap_err();
        assert!(matches!(err, EtlError::CsvParse(_)));
    }
}
