use c6_duckdb::{category_rollup, rank_by_revenue, seed_warehouse};
use duckdb::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open_in_memory()?;
    seed_warehouse(&conn)?;

    println!("=== Fruit Revenue Rankings (RANK window) ===");
    for f in rank_by_revenue(&conn)? {
        println!(
            "  #{} {:<20} {:<10} ${:.2}",
            f.rank,
            f.name,
            f.category,
            f.revenue_cents as f64 / 100.0
        );
    }

    println!("\n=== Category Rollup (GROUP BY ROLLUP) ===");
    for r in category_rollup(&conn)? {
        let label = r.category.as_deref().unwrap_or("GRAND TOTAL");
        println!("  {:<14} ${:.2}", label, r.total_revenue as f64 / 100.0);
    }

    Ok(())
}
