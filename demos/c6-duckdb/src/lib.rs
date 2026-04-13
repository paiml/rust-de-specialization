//! Fruit warehouse analytics: window functions and ROLLUP with DuckDB.

use duckdb::{params, Connection};
use thiserror::Error;

// ── Types ──────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct RankedFruit {
    pub name: String,
    pub category: String,
    pub revenue_cents: i64,
    pub rank: i64,
}

#[derive(Debug)]
pub struct RollupRow {
    pub category: Option<String>,
    pub total_revenue: i64,
}

#[derive(Debug, Error)]
pub enum WarehouseError {
    #[error("duckdb error: {0}")]
    Db(#[from] duckdb::Error),
}

// ── Contract functions ─────────────────────────────────────────────────

/// Create and populate a warehouse table with ~10 fruit rows.
pub fn seed_warehouse(conn: &Connection) -> Result<(), WarehouseError> {
    conn.execute(
        "CREATE TABLE warehouse (
            name       VARCHAR NOT NULL,
            category   VARCHAR NOT NULL,
            revenue_cents BIGINT NOT NULL,
            quantity   INTEGER NOT NULL
        )",
        params![],
    )?;

    conn.execute(
        "INSERT INTO warehouse (name, category, revenue_cents, quantity) VALUES
            ('Honeycrisp Apple', 'Pome',     98_50, 120),
            ('Bartlett Pear',    'Pome',     62_30,  85),
            ('Fuji Apple',       'Pome',     74_00, 200),
            ('Satsuma Mandarin', 'Citrus',   53_20, 310),
            ('Valencia Orange',  'Citrus',  112_40, 450),
            ('Meyer Lemon',      'Citrus',   41_80, 175),
            ('Bing Cherry',      'Stone',    87_60,  90),
            ('Elberta Peach',    'Stone',    69_10, 140),
            ('Italian Plum',     'Stone',    38_90, 110),
            ('Rainier Cherry',   'Stone',    95_20,  65)",
        params![],
    )?;

    Ok(())
}

/// RANK() OVER (PARTITION BY category ORDER BY revenue_cents DESC).
/// Output row count == input row count.
pub fn rank_by_revenue(conn: &Connection) -> Result<Vec<RankedFruit>, WarehouseError> {
    let mut stmt = conn.prepare(
        "SELECT name, category, revenue_cents,
                RANK() OVER (PARTITION BY category ORDER BY revenue_cents DESC) AS rnk
         FROM warehouse
         ORDER BY category, rnk",
    )?;

    let rows = stmt
        .query_map(params![], |row| {
            Ok(RankedFruit {
                name: row.get::<_, String>(0)?,
                category: row.get::<_, String>(1)?,
                revenue_cents: row.get::<_, i64>(2)?,
                rank: row.get::<_, i64>(3)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}

/// GROUP BY ROLLUP(category) with SUM(revenue_cents).
/// Grand total row has NULL category.
pub fn category_rollup(conn: &Connection) -> Result<Vec<RollupRow>, WarehouseError> {
    let mut stmt = conn.prepare(
        "SELECT category, SUM(revenue_cents) AS total_revenue
         FROM warehouse
         GROUP BY ROLLUP(category)
         ORDER BY category NULLS LAST",
    )?;

    let rows = stmt
        .query_map(params![], |row| {
            Ok(RollupRow {
                category: row.get::<_, Option<String>>(0)?,
                total_revenue: row.get::<_, i64>(1)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

    Ok(rows)
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        seed_warehouse(&conn).unwrap();
        conn
    }

    /// Invariant: window preserves row count (output rows == input rows).
    #[test]
    fn window_preserves_row_count() {
        let conn = setup();
        let ranked = rank_by_revenue(&conn).unwrap();
        // seed_warehouse inserts exactly 10 rows
        assert_eq!(ranked.len(), 10, "RANK window must preserve row count");
    }

    /// Invariant: rank=1 in each category has the highest revenue.
    #[test]
    fn rank_one_is_highest_per_category() {
        let conn = setup();
        let ranked = rank_by_revenue(&conn).unwrap();

        for cat in ["Citrus", "Pome", "Stone"] {
            let in_cat: Vec<&RankedFruit> = ranked.iter().filter(|r| r.category == cat).collect();
            assert!(!in_cat.is_empty(), "category {cat} must have rows");

            let top = in_cat
                .iter()
                .find(|r| r.rank == 1)
                .expect("rank 1 must exist");
            for other in &in_cat {
                assert!(
                    top.revenue_cents >= other.revenue_cents,
                    "rank=1 ({}) must have highest revenue in {cat}",
                    top.name
                );
            }
        }
    }

    /// Invariant: rollup grand total == sum of category subtotals.
    #[test]
    fn rollup_grand_total_equals_subtotals() {
        let conn = setup();
        let rollup = category_rollup(&conn).unwrap();

        let grand = rollup
            .iter()
            .find(|r| r.category.is_none())
            .expect("ROLLUP must produce a grand-total row with NULL category");

        let subtotal_sum: i64 = rollup
            .iter()
            .filter(|r| r.category.is_some())
            .map(|r| r.total_revenue)
            .sum();

        assert_eq!(
            grand.total_revenue, subtotal_sum,
            "grand total must equal sum of category subtotals"
        );
    }

    /// Rollup has exactly 4 rows: 3 categories + 1 grand total.
    #[test]
    fn rollup_row_count() {
        let conn = setup();
        let rollup = category_rollup(&conn).unwrap();
        assert_eq!(rollup.len(), 4);
        assert_eq!(rollup.iter().filter(|r| r.category.is_none()).count(), 1);
    }

    /// Error display contains context.
    #[test]
    fn error_display() {
        let e = WarehouseError::Db(duckdb::Error::InvalidParameterCount(0, 1));
        assert!(e.to_string().contains("duckdb error"));
    }

    /// Debug formatting works for domain types.
    #[test]
    fn debug_types() {
        let r = RankedFruit {
            name: "Apple".into(),
            category: "Pome".into(),
            revenue_cents: 100,
            rank: 1,
        };
        assert!(format!("{r:?}").contains("Apple"));
        let rr = RollupRow {
            category: Some("Pome".into()),
            total_revenue: 200,
        };
        assert!(format!("{rr:?}").contains("Pome"));
        let rr_null = RollupRow {
            category: None,
            total_revenue: 500,
        };
        assert!(format!("{rr_null:?}").contains("None"));
    }
}
