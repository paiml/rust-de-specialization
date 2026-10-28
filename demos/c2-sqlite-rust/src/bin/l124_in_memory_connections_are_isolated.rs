use std::collections::HashMap;

struct Connection {
    tables: HashMap<String, Vec<String>>,
}

impl Connection {
    fn open_in_memory() -> Connection {
        Connection {
            tables: HashMap::new(),
        }
    }

    fn insert(&mut self, table: &str, row: &str) {
        self.tables
            .entry(table.to_string())
            .or_default()
            .push(row.to_string());
    }

    fn count(&self, table: &str) -> usize {
        self.tables.get(table).map_or(0, |rows| rows.len())
    }
}

fn main() {
    let mut a = Connection::open_in_memory();
    let b = Connection::open_in_memory();
    a.insert("fruits", "apple");
    a.insert("fruits", "banana");
    assert_eq!(a.count("fruits"), 2);
    assert_eq!(
        b.count("fruits"),
        0,
        "a second :memory: connection must start empty"
    );
    println!("in-memory: a={} b={}", a.count("fruits"), b.count("fruits"));
}
