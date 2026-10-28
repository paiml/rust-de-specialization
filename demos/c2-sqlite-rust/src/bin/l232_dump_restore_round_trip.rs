fn quote(s: &str) -> String {
    // SQLite doubles a single quote inside a string literal.
    format!("'{}'", s.replace('\'', "''"))
}

fn dump(rows: &[(i64, &str, u64)]) -> String {
    let mut out = String::new();
    for (id, name, price) in rows {
        out.push_str(&format!(
            "INSERT INTO fruits VALUES({id},{},{price});\n",
            quote(name)
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
            let (id, rest) = inner.split_once(',').unwrap();
            // The price follows the last comma; the quoted name sits between.
            let (quoted, price) = rest.rsplit_once(',').unwrap();
            let name = quoted[1..quoted.len() - 1].replace("''", "'");
            (id.parse().unwrap(), name, price.parse().unwrap())
        })
        .collect()
}

fn main() {
    let rows = [
        (1, "apple", 150),
        (2, "banana", 30),
        (3, "cherry, red", 600),
        (4, "o'brien plum", 90),
    ];
    let script = dump(&rows);
    let back = restore(&script);
    let expected: Vec<(i64, String, u64)> = rows
        .iter()
        .map(|&(i, n, p)| (i, n.to_string(), p))
        .collect();
    assert_eq!(back, expected, "dump then restore must reproduce every row");
    assert_eq!(script.lines().count(), rows.len());
    // The quote inside o'brien must be doubled, as SQLite requires.
    assert!(script.contains("'o''brien plum'"));
    println!("dump: {} rows round-tripped", back.len());
}
