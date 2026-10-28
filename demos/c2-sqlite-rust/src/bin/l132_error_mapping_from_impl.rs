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
