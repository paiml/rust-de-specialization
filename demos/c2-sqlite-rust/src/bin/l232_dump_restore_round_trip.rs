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
