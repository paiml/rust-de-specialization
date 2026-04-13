use rusqlite::{params, Connection, Result as SqlResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InventoryError {
    #[error("duplicate fruit name: {0}")]
    DuplicateName(String),
    #[error("invalid price: must be > 0, got {0}")]
    InvalidPrice(u64),
    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fruit {
    pub id: i64,
    pub name: String,
    pub price_cents: u64,
    pub quantity: u64,
}

#[derive(Debug, Clone)]
pub struct NewFruit {
    pub name: String,
    pub price_cents: u64,
    pub quantity: u64,
}

pub fn migrate(conn: &Connection) -> Result<(), InventoryError> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS fruits (
            id          INTEGER PRIMARY KEY,
            name        TEXT NOT NULL UNIQUE,
            price_cents INTEGER NOT NULL CHECK(price_cents > 0),
            quantity    INTEGER NOT NULL DEFAULT 0
        );",
    )?;
    Ok(())
}

pub fn insert_fruit(conn: &Connection, fruit: &NewFruit) -> Result<i64, InventoryError> {
    if fruit.price_cents == 0 {
        return Err(InventoryError::InvalidPrice(0));
    }
    conn.execute(
        "INSERT INTO fruits (name, price_cents, quantity) VALUES (?1, ?2, ?3)",
        params![fruit.name, fruit.price_cents, fruit.quantity],
    )
    .map_err(|e| match e {
        rusqlite::Error::SqliteFailure(ref sf, _)
            if sf.code == rusqlite::ffi::ErrorCode::ConstraintViolation =>
        {
            InventoryError::DuplicateName(fruit.name.clone())
        }
        other => InventoryError::Db(other),
    })?;
    Ok(conn.last_insert_rowid())
}

pub fn get_fruit(conn: &Connection, id: i64) -> Result<Option<Fruit>, InventoryError> {
    let mut stmt =
        conn.prepare("SELECT id, name, price_cents, quantity FROM fruits WHERE id = ?1")?;
    let mut rows = stmt.query_map(params![id], |row| {
        Ok(Fruit {
            id: row.get(0)?,
            name: row.get(1)?,
            price_cents: row.get::<_, u64>(2)?,
            quantity: row.get::<_, u64>(3)?,
        })
    })?;
    match rows.next() {
        Some(r) => Ok(Some(r?)),
        None => Ok(None),
    }
}

pub fn update_price(conn: &Connection, id: i64, new_price: u64) -> Result<bool, InventoryError> {
    if new_price == 0 {
        return Err(InventoryError::InvalidPrice(0));
    }
    let changed = conn.execute(
        "UPDATE fruits SET price_cents = ?1 WHERE id = ?2",
        params![new_price, id],
    )?;
    Ok(changed > 0)
}

pub fn delete_fruit(conn: &Connection, id: i64) -> Result<bool, InventoryError> {
    let changed = conn.execute("DELETE FROM fruits WHERE id = ?1", params![id])?;
    Ok(changed > 0)
}

pub fn list_all(conn: &Connection) -> Result<Vec<Fruit>, InventoryError> {
    let mut stmt =
        conn.prepare("SELECT id, name, price_cents, quantity FROM fruits ORDER BY id")?;
    let fruits = stmt
        .query_map([], |row| {
            Ok(Fruit {
                id: row.get(0)?,
                name: row.get(1)?,
                price_cents: row.get::<_, u64>(2)?,
                quantity: row.get::<_, u64>(3)?,
            })
        })?
        .collect::<SqlResult<Vec<_>>>()?;
    Ok(fruits)
}

pub fn bulk_import(conn: &Connection, fruits: &[NewFruit]) -> Result<usize, InventoryError> {
    let tx = conn.unchecked_transaction()?;
    for f in fruits {
        if f.price_cents == 0 {
            return Err(InventoryError::InvalidPrice(0));
            // tx drops here without commit -> automatic rollback
        }
        tx.execute(
            "INSERT INTO fruits (name, price_cents, quantity) VALUES (?1, ?2, ?3)",
            params![f.name, f.price_cents, f.quantity],
        )
        .map_err(|e| match e {
            rusqlite::Error::SqliteFailure(ref sf, _)
                if sf.code == rusqlite::ffi::ErrorCode::ConstraintViolation =>
            {
                InventoryError::DuplicateName(f.name.clone())
            }
            other => InventoryError::Db(other),
        })?;
    }
    tx.commit()?;
    Ok(fruits.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        conn
    }

    fn apple() -> NewFruit {
        NewFruit {
            name: "Apple".into(),
            price_cents: 150,
            quantity: 10,
        }
    }

    #[test]
    fn insert_returns_positive_rowid() {
        let conn = setup();
        let id = insert_fruit(&conn, &apple()).unwrap();
        assert!(id > 0);
    }

    #[test]
    fn duplicate_name_is_error() {
        let conn = setup();
        insert_fruit(&conn, &apple()).unwrap();
        let err = insert_fruit(&conn, &apple()).unwrap_err();
        assert!(matches!(err, InventoryError::DuplicateName(_)));
    }

    #[test]
    fn zero_price_is_error() {
        let conn = setup();
        let bad = NewFruit {
            name: "Free".into(),
            price_cents: 0,
            quantity: 1,
        };
        let err = insert_fruit(&conn, &bad).unwrap_err();
        assert!(matches!(err, InventoryError::InvalidPrice(0)));
    }

    #[test]
    fn get_fruit_returns_none_for_missing() {
        let conn = setup();
        assert!(get_fruit(&conn, 999).unwrap().is_none());
    }

    #[test]
    fn get_fruit_roundtrip() {
        let conn = setup();
        let id = insert_fruit(&conn, &apple()).unwrap();
        let fruit = get_fruit(&conn, id).unwrap().unwrap();
        assert_eq!(fruit.name, "Apple");
        assert_eq!(fruit.price_cents, 150);
    }

    #[test]
    fn bulk_import_all_or_nothing() {
        let conn = setup();
        insert_fruit(&conn, &apple()).unwrap(); // pre-existing
        let batch = vec![
            NewFruit {
                name: "Banana".into(),
                price_cents: 75,
                quantity: 20,
            },
            NewFruit {
                name: "Apple".into(),
                price_cents: 200,
                quantity: 5,
            }, // dup -> rollback
        ];
        assert!(bulk_import(&conn, &batch).is_err());
        // Banana must NOT exist (rolled back)
        let all = list_all(&conn).unwrap();
        assert_eq!(all.len(), 1);
    }

    #[test]
    fn bulk_import_rejects_zero_price() {
        let conn = setup();
        let batch = vec![
            NewFruit {
                name: "Mango".into(),
                price_cents: 300,
                quantity: 5,
            },
            NewFruit {
                name: "Bad".into(),
                price_cents: 0,
                quantity: 1,
            },
        ];
        assert!(bulk_import(&conn, &batch).is_err());
        assert!(list_all(&conn).unwrap().is_empty());
    }

    #[test]
    fn bulk_import_count_matches_input() {
        let conn = setup();
        let batch = vec![
            NewFruit {
                name: "Kiwi".into(),
                price_cents: 200,
                quantity: 12,
            },
            NewFruit {
                name: "Pear".into(),
                price_cents: 180,
                quantity: 8,
            },
        ];
        let n = bulk_import(&conn, &batch).unwrap();
        assert_eq!(n, 2);
        assert_eq!(list_all(&conn).unwrap().len(), 2);
    }

    #[test]
    fn update_price_and_delete() {
        let conn = setup();
        let id = insert_fruit(&conn, &apple()).unwrap();
        assert!(update_price(&conn, id, 200).unwrap());
        assert_eq!(get_fruit(&conn, id).unwrap().unwrap().price_cents, 200);
        assert!(delete_fruit(&conn, id).unwrap());
        assert!(get_fruit(&conn, id).unwrap().is_none());
    }

    #[test]
    fn delete_missing_returns_false() {
        let conn = setup();
        assert!(!delete_fruit(&conn, 999).unwrap());
    }

    #[test]
    fn update_zero_price_is_error() {
        let conn = setup();
        let id = insert_fruit(&conn, &apple()).unwrap();
        let err = update_price(&conn, id, 0).unwrap_err();
        assert!(matches!(err, InventoryError::InvalidPrice(0)));
    }

    #[test]
    fn update_missing_returns_false() {
        let conn = setup();
        assert!(!update_price(&conn, 999, 100).unwrap());
    }

    #[test]
    fn error_display() {
        let e = InventoryError::DuplicateName("Apple".into());
        assert!(e.to_string().contains("Apple"));
        let e = InventoryError::InvalidPrice(0);
        assert!(e.to_string().contains("0"));
    }

    #[test]
    fn list_all_empty() {
        let conn = setup();
        assert!(list_all(&conn).unwrap().is_empty());
    }
}
