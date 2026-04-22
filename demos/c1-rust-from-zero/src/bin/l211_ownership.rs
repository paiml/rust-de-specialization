//! Runnable demo for Lesson 2.1.1 — Ownership.
//! `cargo run -p c1-rust-from-zero --bin l211_ownership`

use c1_rust_from_zero::l211_ownership::{heap_backed_length, one_owner_one_drop, stack_size_i32};

fn main() {
    println!("=== Lesson 2.1.1 — Ownership ===");
    println!();
    println!("CONTRACT: i32 lives on the stack, size = 4 bytes.");
    println!("  stack_size_i32() = {}", stack_size_i32());
    assert_eq!(stack_size_i32(), 4);
    println!("  -> PASS");

    println!();
    println!("CONTRACT: heap-backed String drops its buffer on scope exit.");
    let s = String::from("alice");
    let len = heap_backed_length(s);
    println!("  heap_backed_length(\"alice\") = {len}");
    assert_eq!(len, 5);
    println!("  -> PASS");

    println!();
    println!("CONTRACT: exactly one Drop call per owner.");
    let drops = one_owner_one_drop();
    println!("  one_owner_one_drop() = {drops}");
    assert_eq!(drops, 1);
    println!("  -> PASS");
}
