//! Runnable demo for Lesson 2.1.2 — Move, Copy, Clone.
//! `cargo run -p c1-rust-from-zero --bin l212_move_copy_clone`

use c1_rust_from_zero::l212_move_copy_clone::{
    clone_string_independent, copy_i32_both_usable, move_string_transfers,
};

fn main() {
    println!("=== Lesson 2.1.2 — Move, Copy, Clone ===");
    println!();
    println!("CONTRACT: i32 Copy — both values usable.");
    let (x, y) = copy_i32_both_usable();
    println!("  x = {x}, y = {y}");
    assert_eq!(x, y);
    println!("  -> PASS");

    println!();
    println!("CONTRACT: String move — ownership transferred, content preserved.");
    let (len, moved) = move_string_transfers(String::from("alice"));
    println!("  moved.len() = {len}, moved = {moved:?}");
    assert_eq!(len, moved.len());
    println!("  -> PASS");

    println!();
    println!("CONTRACT: Clone — independent heap buffers with equal content.");
    let (a, b) = clone_string_independent("hello");
    println!("  a = {a:?} @ {:p}", a.as_ptr());
    println!("  b = {b:?} @ {:p}", b.as_ptr());
    assert_eq!(a, b);
    assert_ne!(a.as_ptr(), b.as_ptr());
    println!("  -> PASS (distinct buffers, equal content)");
}
