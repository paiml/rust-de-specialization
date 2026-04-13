use polars::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalyticsError {
    #[error("Polars error: {0}")]
    Polars(#[from] PolarsError),
}

/// Sample dataset: ~10 fruits with name, category, price_cents (UInt64), quantity (UInt32).
pub fn build_sample_df() -> DataFrame {
    df! {
        "name"        => ["Apple", "Banana", "Cherry", "Date", "Elderberry",
                          "Fig", "Grape", "Honeydew", "Kiwi", "Lemon"],
        "category"    => ["Pome", "Tropical", "Stone", "Tropical", "Berry",
                          "Tropical", "Berry", "Melon", "Tropical", "Citrus"],
        "price_cents" => [150u64, 75, 300, 500, 800, 250, 200, 450, 175, 100],
        "quantity"    => [40u32, 100, 25, 10, 5, 20, 60, 15, 35, 50],
    }
    .expect("sample dataframe construction should not fail")
}

/// Contract: fruit-analytics-v1 / analyze_sales
/// Filter price_cents > 0, group by category, aggregate total_revenue (sum) and avg_price (mean).
/// Output columns: category, total_revenue, avg_price.
pub fn analyze_sales(df: LazyFrame) -> Result<DataFrame, AnalyticsError> {
    let out = df
        .filter(col("price_cents").gt(lit(0u64)))
        .with_column((col("price_cents") * col("quantity").cast(DataType::UInt64)).alias("revenue"))
        .group_by([col("category")])
        .agg([
            col("revenue").sum().alias("total_revenue"),
            col("price_cents").mean().alias("avg_price"),
        ])
        .sort(
            ["total_revenue"],
            SortMultipleOptions::default().with_order_descending(true),
        )
        .collect()?;
    Ok(out)
}

/// Contract: fruit-analytics-v1 / top_fruits
/// Rank fruits by total revenue descending, return top N rows.
/// Output columns: name, total_revenue.
pub fn top_fruits(df: LazyFrame, n: u32) -> Result<DataFrame, AnalyticsError> {
    let out = df
        .filter(col("price_cents").gt(lit(0u64)))
        .with_column(
            (col("price_cents") * col("quantity").cast(DataType::UInt64)).alias("total_revenue"),
        )
        .select([col("name"), col("total_revenue")])
        .sort(
            ["total_revenue"],
            SortMultipleOptions::default().with_order_descending(true),
        )
        .limit(n)
        .collect()?;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Invariant: analyze_sales output has exactly {category, total_revenue, avg_price}.
    #[test]
    fn analyze_sales_has_expected_columns() {
        let df = build_sample_df();
        let result = analyze_sales(df.lazy()).unwrap();
        let cols: Vec<&PlSmallStr> = result.get_column_names().into_iter().collect();
        assert!(cols.iter().any(|c| c.as_str() == "category"));
        assert!(cols.iter().any(|c| c.as_str() == "total_revenue"));
        assert!(cols.iter().any(|c| c.as_str() == "avg_price"));
        assert_eq!(cols.len(), 3);
    }

    // Invariant: top_fruits(n) returns exactly n rows (when n <= total rows).
    #[test]
    fn top_fruits_returns_correct_count() {
        let df = build_sample_df();
        let result = top_fruits(df.lazy(), 3).unwrap();
        assert_eq!(result.height(), 3);
    }

    // Invariant: total_revenue for a category equals sum(price_cents * quantity) per row.
    // Manual: Tropical = Banana(75*100) + Date(500*10) + Fig(250*20) + Kiwi(175*35)
    //       = 7500 + 5000 + 5000 + 6125 = 23625
    #[test]
    fn total_revenue_matches_manual_calculation() {
        let df = build_sample_df();
        let result = analyze_sales(df.lazy()).unwrap();
        let cat_col = result.column("category").unwrap().str().unwrap();
        let rev_col = result.column("total_revenue").unwrap();

        let idx = cat_col
            .into_iter()
            .position(|v| v == Some("Tropical"))
            .expect("Tropical category must exist");

        let tropical_rev = rev_col.get(idx).unwrap();
        // AnyValue comparison: extract u64
        assert_eq!(tropical_rev, AnyValue::UInt64(23_625));
    }
}
