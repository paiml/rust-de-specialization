use sqlx::SqlitePool;
use thiserror::Error;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct OrderItem {
    pub fruit_name: String,
    pub price_cents: u64,
    pub quantity: u32,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: i64,
    pub total_cents: u64,
    pub items: Vec<OrderItem>,
}

#[derive(Debug, Error)]
pub enum OrderError {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("order must contain at least one item")]
    EmptyOrder,
    #[error("order not found")]
    NotFound,
}

// ---------------------------------------------------------------------------
// Migrations
// ---------------------------------------------------------------------------

/// Create tables idempotently. Calling twice is a no-op.
pub async fn run_migrations(pool: &SqlitePool) -> Result<(), OrderError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS orders (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            total_cents INTEGER NOT NULL DEFAULT 0
        )",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS order_items (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            order_id    INTEGER NOT NULL REFERENCES orders(id),
            fruit_name  TEXT    NOT NULL,
            price_cents INTEGER NOT NULL,
            quantity    INTEGER NOT NULL
        )",
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

/// Atomically insert an order with its line items.
/// `total_cents` is the computed sum of `price_cents * quantity` across items.
/// Returns the new order id. Errors on an empty item list.
pub async fn create_order(pool: &SqlitePool, items: &[OrderItem]) -> Result<i64, OrderError> {
    if items.is_empty() {
        return Err(OrderError::EmptyOrder);
    }

    let total_cents: u64 = items
        .iter()
        .map(|i| i.price_cents * i.quantity as u64)
        .sum();

    let mut tx = pool.begin().await?;

    let order_id = sqlx::query("INSERT INTO orders (total_cents) VALUES (?)")
        .bind(total_cents as i64)
        .execute(&mut *tx)
        .await?
        .last_insert_rowid();

    for item in items {
        sqlx::query(
            "INSERT INTO order_items (order_id, fruit_name, price_cents, quantity)
             VALUES (?, ?, ?, ?)",
        )
        .bind(order_id)
        .bind(&item.fruit_name)
        .bind(item.price_cents as i64)
        .bind(item.quantity as i64)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(order_id)
}

/// Fetch an order with all its line items, or `None` if the id does not exist.
pub async fn get_order(pool: &SqlitePool, id: i64) -> Result<Option<Order>, OrderError> {
    let row = sqlx::query("SELECT id, total_cents FROM orders WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;

    let row = match row {
        Some(r) => r,
        None => return Ok(None),
    };

    use sqlx::Row;
    let order_id: i64 = row.get("id");
    let total_cents: i64 = row.get("total_cents");

    let item_rows =
        sqlx::query("SELECT fruit_name, price_cents, quantity FROM order_items WHERE order_id = ?")
            .bind(order_id)
            .fetch_all(pool)
            .await?;

    let items = item_rows
        .iter()
        .map(|r| {
            let price: i64 = r.get("price_cents");
            let qty: i64 = r.get("quantity");
            OrderItem {
                fruit_name: r.get("fruit_name"),
                price_cents: price as u64,
                quantity: qty as u32,
            }
        })
        .collect();

    Ok(Some(Order {
        id: order_id,
        total_cents: total_cents as u64,
        items,
    }))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn migrations_are_idempotent() {
        let pool = setup().await;
        // Second call must succeed without error.
        run_migrations(&pool).await.unwrap();
    }

    #[tokio::test]
    async fn empty_order_is_rejected() {
        let pool = setup().await;
        let err = create_order(&pool, &[]).await.unwrap_err();
        assert!(matches!(err, OrderError::EmptyOrder));
    }

    #[tokio::test]
    async fn create_and_get_order() {
        let pool = setup().await;
        let items = vec![
            OrderItem {
                fruit_name: "Apple".into(),
                price_cents: 150,
                quantity: 4,
            },
            OrderItem {
                fruit_name: "Banana".into(),
                price_cents: 75,
                quantity: 6,
            },
        ];
        // expected total: 150*4 + 75*6 = 600 + 450 = 1050
        let id = create_order(&pool, &items).await.unwrap();
        let order = get_order(&pool, id).await.unwrap().expect("order exists");

        assert_eq!(order.id, id);
        assert_eq!(order.total_cents, 1050);
        assert_eq!(order.items.len(), 2);
    }

    #[tokio::test]
    async fn get_missing_order_returns_none() {
        let pool = setup().await;
        let result = get_order(&pool, 999).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn total_cents_consistency() {
        let pool = setup().await;
        let items = vec![
            OrderItem {
                fruit_name: "Mango".into(),
                price_cents: 200,
                quantity: 3,
            },
            OrderItem {
                fruit_name: "Strawberry".into(),
                price_cents: 50,
                quantity: 10,
            },
            OrderItem {
                fruit_name: "Blueberry".into(),
                price_cents: 300,
                quantity: 1,
            },
        ];
        let expected: u64 = items
            .iter()
            .map(|i| i.price_cents * i.quantity as u64)
            .sum();
        let id = create_order(&pool, &items).await.unwrap();
        let order = get_order(&pool, id).await.unwrap().unwrap();
        assert_eq!(order.total_cents, expected);
    }
}
