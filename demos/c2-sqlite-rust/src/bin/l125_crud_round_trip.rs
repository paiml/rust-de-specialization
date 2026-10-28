use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
struct Fruit {
    name: String,
    price_cents: u64,
}

#[derive(Default)]
struct Inventory {
    rows: BTreeMap<i64, Fruit>,
    next_id: i64,
}

impl Inventory {
    fn insert(&mut self, name: &str, price_cents: u64) -> i64 {
        self.next_id += 1;
        self.rows.insert(
            self.next_id,
            Fruit {
                name: name.to_string(),
                price_cents,
            },
        );
        self.next_id
    }

    fn get(&self, id: i64) -> Option<&Fruit> {
        self.rows.get(&id)
    }

    fn update_price(&mut self, id: i64, price_cents: u64) -> bool {
        match self.rows.get_mut(&id) {
            Some(f) => {
                f.price_cents = price_cents;
                true
            }
            None => false,
        }
    }

    fn delete(&mut self, id: i64) -> bool {
        self.rows.remove(&id).is_some()
    }
}

fn main() {
    let mut inv = Inventory::default();
    let id = inv.insert("apple", 150);
    assert_eq!(
        inv.get(id),
        Some(&Fruit {
            name: "apple".into(),
            price_cents: 150
        })
    );
    assert!(inv.update_price(id, 175));
    assert_eq!(inv.get(id).unwrap().price_cents, 175);
    assert!(inv.delete(id));
    assert_eq!(inv.get(id), None);
    assert!(
        !inv.delete(id),
        "deleting a missing row reports false, not an error"
    );
    println!("crud: insert, read, update, delete ok");
}
