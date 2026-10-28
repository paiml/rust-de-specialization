#[derive(Debug, PartialEq)]
enum InventoryError {
    InvalidPrice(u64),
}

struct Table {
    committed: Vec<(String, u64)>,
}

impl Table {
    // Stage every row; only a fully valid batch is committed. Dropping the
    // staging vector without committing is the rollback.
    fn bulk_import(&mut self, batch: &[(&str, u64)]) -> Result<usize, InventoryError> {
        let mut staged = Vec::new();
        for &(name, price) in batch {
            if price == 0 {
                return Err(InventoryError::InvalidPrice(0));
            }
            staged.push((name.to_string(), price));
        }
        let n = staged.len();
        self.committed.extend(staged);
        Ok(n)
    }
}

fn main() {
    let mut t = Table {
        committed: Vec::new(),
    };
    assert_eq!(t.bulk_import(&[("apple", 150), ("pear", 90)]), Ok(2));
    assert_eq!(t.committed.len(), 2);

    let bad = t.bulk_import(&[("plum", 70), ("ghost", 0), ("fig", 200)]);
    assert_eq!(bad, Err(InventoryError::InvalidPrice(0)));
    assert_eq!(
        t.committed.len(),
        2,
        "a failed batch must leave zero rows behind"
    );
    println!(
        "transaction: rolled back, {} rows committed",
        t.committed.len()
    );
}
