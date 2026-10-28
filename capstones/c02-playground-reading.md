# Run the SQLite for Rust Patterns in Your Browser

Each section below is one lesson's contract, shrunk to a single file that runs on the Rust Playground with no install. Every section names its **Contract**, asserts it with `assert!` or `assert_eq!`, and links to a gist that was run on play.rust-lang.org (exit 0) before this reading shipped. Click **Run**, then break the contract on purpose and watch the assertion fail.

The Playground cannot link the `rusqlite` crate, so each snippet models the SQLite pattern from the lesson with the standard library alone: the same contract, the same assertion, a few dozen lines. The real `rusqlite` versions live in this course's demo crate.

## Section 1 — Lesson 1.2.4: in memory connections are isolated

**Contract:** Each in-memory connection owns a private database, so rows inserted through one connection are invisible to a second.

```rust
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
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=43ccc18e9889bca619fd8823b5029f37>

Expected output contains: `in-memory: a=2 b=0`

## Section 2 — Lesson 1.2.5: crud round trip

**Contract:** Insert, get, update and delete round-trip on one row, and deleting a missing row reports false rather than an error.

```rust
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
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=ad53a8bcfafd1258070078ba3e88fa70>

Expected output contains: `crud: insert, read, update, delete ok`

## Section 3 — Lesson 1.3.2: error mapping from impl

**Contract:** A low-level unique-violation maps to a typed DuplicateName error, other failures wrap via From and the question-mark operator.

```rust
use std::fmt;

#[derive(Debug, PartialEq)]
enum DbError {
    UniqueViolation,
    Busy,
}

#[derive(Debug, PartialEq)]
enum InventoryError {
    DuplicateName(String),
    Db(DbError),
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InventoryError::DuplicateName(n) => write!(f, "duplicate fruit name: {n}"),
            InventoryError::Db(e) => write!(f, "database error: {e:?}"),
        }
    }
}

impl From<DbError> for InventoryError {
    fn from(e: DbError) -> Self {
        InventoryError::Db(e)
    }
}

fn raw_insert(name: &str, taken: &[&str]) -> Result<(), DbError> {
    if name == "locked" {
        return Err(DbError::Busy);
    }
    if taken.contains(&name) {
        return Err(DbError::UniqueViolation);
    }
    Ok(())
}

fn insert(name: &str, taken: &[&str]) -> Result<(), InventoryError> {
    match raw_insert(name, taken) {
        Err(DbError::UniqueViolation) => Err(InventoryError::DuplicateName(name.to_string())),
        other => Ok(other?),
    }
}

fn main() {
    let taken = ["apple"];
    assert_eq!(insert("pear", &taken), Ok(()));
    let dup = insert("apple", &taken).unwrap_err();
    assert_eq!(dup, InventoryError::DuplicateName("apple".into()));
    assert_eq!(dup.to_string(), "duplicate fruit name: apple");
    assert_eq!(
        insert("locked", &taken),
        Err(InventoryError::Db(DbError::Busy))
    );
    println!("errors: {dup}");
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=3e076846ff59875b10df4e23d5abc178>

Expected output contains: `errors: duplicate fruit name: apple`

## Section 4 — Lesson 1.3.4: transaction rollback all or nothing

**Contract:** A bulk import is all-or-nothing: one invalid row rolls back the whole batch and leaves previously committed rows untouched.

```rust
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
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=044fbdda3e7d8630dd64ebce142913ad>

Expected output contains: `transaction: rolled back, 2 rows committed`

## Section 5 — Lesson 2.1.2: csv rows parse or reject

**Contract:** CSV rows parse into typed records or yield a named error, and a malformed row never aborts the batch.

```rust
#[derive(Debug, PartialEq)]
struct NewFruit {
    name: String,
    price_cents: u64,
    quantity: u64,
}

#[derive(Debug, PartialEq)]
enum ParseError {
    WrongFieldCount(usize),
    BadNumber(String),
}

fn parse_row(line: &str) -> Result<NewFruit, ParseError> {
    let fields: Vec<&str> = line.split(',').map(str::trim).collect();
    if fields.len() != 3 {
        return Err(ParseError::WrongFieldCount(fields.len()));
    }
    let num = |s: &str| {
        s.parse::<u64>()
            .map_err(|_| ParseError::BadNumber(s.to_string()))
    };
    Ok(NewFruit {
        name: fields[0].to_string(),
        price_cents: num(fields[1])?,
        quantity: num(fields[2])?,
    })
}

fn main() {
    let ok = parse_row("apple, 150, 12").unwrap();
    assert_eq!(
        ok,
        NewFruit {
            name: "apple".into(),
            price_cents: 150,
            quantity: 12
        }
    );
    assert_eq!(parse_row("pear,90"), Err(ParseError::WrongFieldCount(2)));
    assert_eq!(
        parse_row("fig,abc,3"),
        Err(ParseError::BadNumber("abc".into()))
    );

    let input = "apple,150,12\nbroken\nbanana,30,40\n";
    let (good, bad): (Vec<_>, Vec<_>) = input.lines().map(parse_row).partition(|r| r.is_ok());
    assert_eq!((good.len(), bad.len()), (2, 1));
    println!("csv: {} parsed, {} rejected", good.len(), bad.len());
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=e10afabea941ac1a16950cd9b509cddd>

Expected output contains: `csv: 2 parsed, 1 rejected`

## Section 6 — Lesson 2.3.2: dump restore round trip

**Contract:** Dumping rows to INSERT statements and restoring them reproduces every row exactly.

```rust
fn dump(rows: &[(i64, &str, u64)]) -> String {
    let mut out = String::new();
    for (id, name, price) in rows {
        out.push_str(&format!(
            "INSERT INTO fruits VALUES({id},'{name}',{price});\n"
        ));
    }
    out
}

fn restore(script: &str) -> Vec<(i64, String, u64)> {
    script
        .lines()
        .map(|l| {
            let inner = l
                .strip_prefix("INSERT INTO fruits VALUES(")
                .and_then(|r| r.strip_suffix(");"))
                .expect("dump line has the INSERT shape");
            let mut p = inner.split(',');
            let id = p.next().unwrap().parse().unwrap();
            let name = p.next().unwrap().trim_matches('\'').to_string();
            let price = p.next().unwrap().parse().unwrap();
            (id, name, price)
        })
        .collect()
}

fn main() {
    let rows = [(1, "apple", 150), (2, "banana", 30), (3, "cherry", 600)];
    let script = dump(&rows);
    let back = restore(&script);
    let expected: Vec<(i64, String, u64)> = rows
        .iter()
        .map(|&(i, n, p)| (i, n.to_string(), p))
        .collect();
    assert_eq!(back, expected, "dump then restore must reproduce every row");
    assert_eq!(script.lines().count(), rows.len());
    println!("dump: {} rows round-tripped", back.len());
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=de318fee3e8226a843d7b3875d20dc48>

Expected output contains: `dump: 3 rows round-tripped`

## Contract summary

| # | Lesson | Contract | Enforced by | Playground output |
|---|--------|----------|-------------|-------------------|
| 1 | 1.2.4 | in memory connections are isolated | `assert!`/`assert_eq!` in the snippet | `in-memory: a=2 b=0` |
| 2 | 1.2.5 | crud round trip | `assert!`/`assert_eq!` in the snippet | `crud: insert, read, update, delete ok` |
| 3 | 1.3.2 | error mapping from impl | `assert!`/`assert_eq!` in the snippet | `errors: duplicate fruit name: apple` |
| 4 | 1.3.4 | transaction rollback all or nothing | `assert!`/`assert_eq!` in the snippet | `transaction: rolled back, 2 rows committed` |
| 5 | 2.1.2 | csv rows parse or reject | `assert!`/`assert_eq!` in the snippet | `csv: 2 parsed, 1 rejected` |
| 6 | 2.3.2 | dump restore round trip | `assert!`/`assert_eq!` in the snippet | `dump: 3 rows round-tripped` |
