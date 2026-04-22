//! Runnable demo for Lesson 1.1.1 — Memory safety at compile time.
//! `cargo run -p c1-rust-from-zero --bin l111_memory_safety`

use c1_rust_from_zero::l111_memory_safety::{scoped_allocation, LoggedString};

fn main() {
    println!("=== Lesson 1.1.1 — What Is Rust ===");
    println!();
    println!("CONTRACT: Drop runs exactly once per owned value.");

    let drops = scoped_allocation("alice");
    println!("  scoped_allocation(\"alice\") observed drops = {drops}");
    assert_eq!(drops, 1, "CONTRACT VIOLATED");
    println!("  -> PASS");

    println!();
    println!("CONTRACT: double-free is unrepresentable.");
    println!("  See the `compile_fail` doc test on `double_drop_is_unrepresentable()`.");

    let s = LoggedString::new("bob");
    println!("  new LoggedString has len = {}", s.len());
    drop(s);
    // Attempting `drop(s)` again here would be rejected by rustc E0382.
    println!("  -> PASS (compile-time proof)");
}
