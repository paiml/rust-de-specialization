use c2_sqlite_rust::*;
use rusqlite::Connection;

fn main() -> Result<(), InventoryError> {
    let conn = Connection::open_in_memory()?;
    migrate(&conn)?;

    // Single inserts
    let apple_id = insert_fruit(
        &conn,
        &NewFruit {
            name: "Apple".into(),
            price_cents: 150,
            quantity: 10,
        },
    )?;
    println!("Inserted Apple -> id {apple_id}");

    // Bulk import
    let batch = vec![
        NewFruit {
            name: "Banana".into(),
            price_cents: 75,
            quantity: 25,
        },
        NewFruit {
            name: "Mango".into(),
            price_cents: 320,
            quantity: 8,
        },
        NewFruit {
            name: "Kiwi".into(),
            price_cents: 200,
            quantity: 15,
        },
    ];
    let n = bulk_import(&conn, &batch)?;
    println!("Bulk-imported {n} fruits");

    // List all
    println!("\n--- Inventory ---");
    for f in list_all(&conn)? {
        println!(
            "  [{}] {} — {}c x{}",
            f.id, f.name, f.price_cents, f.quantity
        );
    }

    // Update + delete
    update_price(&conn, apple_id, 175)?;
    delete_fruit(&conn, 2)?; // remove Banana
    println!("\n--- After update & delete ---");
    for f in list_all(&conn)? {
        println!(
            "  [{}] {} — {}c x{}",
            f.id, f.name, f.price_cents, f.quantity
        );
    }
    Ok(())
}
