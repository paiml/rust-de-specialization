//! Runnable demo for Lesson 2.3.2 — Result and the ? operator.
//! `cargo run -p c1-rust-from-zero --bin l232_result_question`

use c1_rust_from_zero::l232_result_question::{chain_match, chain_question};

fn main() {
    println!("=== Lesson 2.3.2 — Result and the ? Operator ===");
    println!();
    println!("CONTRACT: chain_question (?) produces identical output to chain_match (nested).");

    let cases = [("42", 2), ("10", 0), ("abc", 2), ("", 1), ("-1", 5)];
    for (s, d) in cases {
        let q = chain_question(s, d);
        let m = chain_match(s, d);
        println!("  input=({s:?}, {d}) -> question={q:?} match={m:?}");
        assert_eq!(q, m, "CONTRACT VIOLATED at ({s:?}, {d})");
    }
    println!("  -> PASS (oracle equivalence across sample space)");
}
