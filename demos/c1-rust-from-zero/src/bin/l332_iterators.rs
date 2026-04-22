//! Runnable demo for Lesson 3.3.2 — Iterators.
//! `cargo run -p c1-rust-from-zero --bin l332_iterators`

use c1_rust_from_zero::l332_iterators::{
    eager_pipeline, into_iter_sum, iter_borrow_sum, lazy_adapters_no_execution, pipeline,
};

fn main() {
    println!("=== Lesson 3.3.2 — Iterators ===");
    println!();
    println!("CONTRACT: pipeline(n) == eager_pipeline(n) for all n in 0..=50.");
    for n in [0u32, 1, 2, 6, 10, 50] {
        let p = pipeline(n);
        let e = eager_pipeline(n);
        println!("  n={n:>3} pipeline={p:?} eager={e:?}");
        assert_eq!(p, e);
    }
    println!("  -> PASS");

    println!();
    println!("CONTRACT: iter_borrow does not consume the Vec.");
    let v = vec![1, 2, 3, 4, 5];
    let sum = iter_borrow_sum(&v);
    println!("  sum = {sum}, v still usable (len = {})", v.len());
    assert_eq!(sum, 15);
    assert_eq!(v.len(), 5);
    println!("  -> PASS");

    println!();
    println!("CONTRACT: into_iter consumes the Vec.");
    let v2 = vec![10, 20, 30];
    let total = into_iter_sum(v2);
    println!("  total = {total}");
    assert_eq!(total, 60);
    println!("  -> PASS (ownership moved)");

    println!();
    println!("CONTRACT: adapter chains are lazy — no consumer means no execution.");
    let counter = lazy_adapters_no_execution();
    println!("  side-effect counter after building chain = {counter}");
    assert_eq!(counter, 0);
    println!("  -> PASS");
}
