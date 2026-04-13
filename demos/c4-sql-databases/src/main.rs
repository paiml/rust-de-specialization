use c4_sql_databases::{create_order, get_order, run_migrations, OrderItem};
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect(":memory:").await?;
    run_migrations(&pool).await?;

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
        OrderItem {
            fruit_name: "Blueberry".into(),
            price_cents: 300,
            quantity: 2,
        },
    ];

    let order_id = create_order(&pool, &items).await?;
    println!("Created order #{order_id}");

    if let Some(order) = get_order(&pool, order_id).await? {
        println!("Order #{} - total: {} cents", order.id, order.total_cents);
        for item in &order.items {
            println!(
                "  {} x {} @ {} cents each",
                item.quantity, item.fruit_name, item.price_cents
            );
        }
    }

    Ok(())
}
