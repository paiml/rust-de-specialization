//! Lesson 3.3.2 — Iterators: lazy pipelines with map, filter, and collect.
//!
//! Contract: `contracts/c1-l332-iterators-v1.yaml`
//! Transcript anchor: "adapters build a pipeline, consumers run it"

use std::cell::Cell;

/// CONTRACT: pipeline
/// Lazy adapter chain: filter evens, square each, collect into Vec.
pub fn pipeline(n: u32) -> Vec<u32> {
    (1..=n).filter(|x| x % 2 == 0).map(|x| x * x).collect()
}

/// CONTRACT: eager_pipeline (oracle)
/// Same semantics as `pipeline` but written as an explicit loop. Used to
/// prove equivalence with the iterator version.
pub fn eager_pipeline(n: u32) -> Vec<u32> {
    let mut out = Vec::new();
    for x in 1..=n {
        if x % 2 == 0 {
            out.push(x * x);
        }
    }
    out
}

/// CONTRACT: iter_borrow_sum
/// Borrow iteration — caller retains ownership of v.
pub fn iter_borrow_sum(v: &[i32]) -> i32 {
    v.iter().sum()
}

/// CONTRACT: into_iter_sum
/// Consuming iteration — takes ownership of v.
pub fn into_iter_sum(v: Vec<i32>) -> i32 {
    v.into_iter().sum()
}

/// Builds a filter + map chain over 1..=10 that increments `counter` once
/// per element the filter sees. Shared by both the lazy and eager demos
/// so that the *same* closures witness both contracts: they run when the
/// chain is consumed, they don't when it isn't.
fn build_counted_chain(counter: &Cell<u32>) -> impl Iterator<Item = u32> + '_ {
    (1..=10_u32)
        .filter(|_| {
            counter.set(counter.get() + 1);
            true
        })
        .map(|x| x * 2)
}

/// CONTRACT: lazy_adapters_no_execution
/// Invariant: building an adapter chain (filter + map) without a consumer
/// does NOT execute the closures. Returns the side-effect counter value
/// AFTER the chain is built — must be 0.
pub fn lazy_adapters_no_execution() -> u32 {
    let counter = Cell::new(0u32);
    let _chain = build_counted_chain(&counter);
    // _chain is never consumed — closures never run.
    counter.get()
}

/// CONTRACT: eager_adapters_execute
/// Companion to `lazy_adapters_no_execution`. Same closures, but THIS time
/// we call `.sum()` to consume the chain. The filter closure must run
/// exactly 10 times (once per element in 1..=10) and the map must produce
/// 2+4+...+20 = 110. Returns the side-effect counter value — must be 10.
pub fn eager_adapters_execute() -> u32 {
    let counter = Cell::new(0u32);
    let total: u32 = build_counted_chain(&counter).sum();
    assert_eq!(total, 110);
    counter.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipeline_small_cases() {
        assert_eq!(pipeline(0), Vec::<u32>::new());
        assert_eq!(pipeline(1), Vec::<u32>::new());
        assert_eq!(pipeline(2), vec![4]);
        assert_eq!(pipeline(6), vec![4, 16, 36]);
    }

    /// CONTRACT: pipeline_oracle_equivalence
    /// For every n in 0..=50, pipeline(n) must equal eager_pipeline(n).
    #[test]
    fn pipeline_equivalence_oracle() {
        for n in 0..=50 {
            assert_eq!(pipeline(n), eager_pipeline(n), "divergence at n={n}");
        }
    }

    #[test]
    fn iter_borrow_preserves_vec() {
        let v = vec![1, 2, 3, 4, 5];
        let sum = iter_borrow_sum(&v);
        assert_eq!(sum, 15);
        // v still usable — borrow did not consume it.
        assert_eq!(v.len(), 5);
    }

    #[test]
    fn iter_borrow_empty_is_zero() {
        let v: Vec<i32> = vec![];
        assert_eq!(iter_borrow_sum(&v), 0);
    }

    #[test]
    fn into_iter_consumes_and_sums() {
        let v = vec![10, 20, 30];
        assert_eq!(into_iter_sum(v), 60);
    }

    #[test]
    fn into_iter_empty_is_zero() {
        let v: Vec<i32> = vec![];
        assert_eq!(into_iter_sum(v), 0);
    }

    #[test]
    fn adapters_are_lazy() {
        assert_eq!(lazy_adapters_no_execution(), 0);
    }

    #[test]
    fn eager_adapters_run_ten_times() {
        assert_eq!(eager_adapters_execute(), 10);
    }
}
