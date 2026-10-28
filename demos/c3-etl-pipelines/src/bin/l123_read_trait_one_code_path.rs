use std::io::{BufRead, BufReader, Read};

fn count_rows(source: impl Read) -> usize {
    BufReader::new(source)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .count()
}

fn main() {
    let bytes: &[u8] = b"apple,1.50\nbanana,0.30\ncherry,6.00\n";
    // One generic function, two sources: a byte slice and an owned cursor.
    let from_slice = count_rows(bytes);
    let from_cursor = count_rows(std::io::Cursor::new(bytes.to_vec()));
    assert_eq!(from_slice, 3);
    assert_eq!(
        from_slice, from_cursor,
        "every Read source must give the same answer"
    );
    println!("read trait: {from_slice} rows from both sources");
}
