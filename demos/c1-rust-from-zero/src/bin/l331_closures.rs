//! Runnable demo for Lesson 3.3.1 — Closures (Fn, FnMut, FnOnce).
//! `cargo run -p c1-rust-from-zero --bin l331_closures`

use c1_rust_from_zero::l331_closures::{make_adder, make_counter, make_once};

fn main() {
    println!("=== Lesson 3.3.1 — Closures ===");
    println!();
    println!("CONTRACT: Fn — pure, callable multiple times.");
    let add5 = make_adder(5);
    println!(
        "  add5(1) = {}, add5(10) = {}, add5(1) again = {}",
        add5(1),
        add5(10),
        add5(1)
    );
    assert_eq!(add5(1), 6);
    println!("  -> PASS");

    println!();
    println!("CONTRACT: FnMut — monotonic counter starting at 1.");
    let mut counter = make_counter();
    for expected in 1..=4 {
        let got = counter();
        println!("  counter() = {got} (expected {expected})");
        assert_eq!(got, expected);
    }
    println!("  -> PASS");

    println!();
    println!("CONTRACT: FnOnce — consumes captured value, single use.");
    let f = make_once(String::from("alice"));
    let result = f();
    println!("  f() = {result:?}");
    assert_eq!(result, "alice");
    // `f()` again here would be rejected by rustc E0382.
    println!("  -> PASS (compile-time single-use proof)");
}
