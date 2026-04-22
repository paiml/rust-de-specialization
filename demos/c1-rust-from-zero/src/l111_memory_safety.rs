//! Lesson 1.1.1 — What Is Rust: memory safety at compile time.
//!
//! Contract: `contracts/c1-l111-memory-safety-v1.yaml`
//! Transcript anchor: "the compiler refuses to build your program if it cannot
//! prove your memory access is safe"
//!
//! The demo proves two things:
//! 1. RUNTIME: Drop runs exactly once per owned value, observable via a counter.
//! 2. COMPILE-TIME: a double-free is unrepresentable — the second use after
//!    move is rejected by rustc E0382. See the `compile_fail` doc test below.

use std::sync::atomic::{AtomicU64, Ordering};

static DROP_COUNT: AtomicU64 = AtomicU64::new(0);

/// Heap-allocated string that increments a global counter when dropped.
pub struct LoggedString {
    inner: String,
}

impl LoggedString {
    pub fn new(s: &str) -> Self {
        Self {
            inner: s.to_owned(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl Drop for LoggedString {
    fn drop(&mut self) {
        DROP_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

/// CONTRACT: scoped_allocation
/// Invariant: Drop runs exactly once per LoggedString instance.
/// Invariant: Returned count equals DROP_COUNT delta observed across the call.
pub fn scoped_allocation(s: &str) -> u64 {
    let before = DROP_COUNT.load(Ordering::SeqCst);
    {
        let owned = LoggedString::new(s);
        // owned is dropped at the end of this block.
        let _ = owned.len();
    }
    let after = DROP_COUNT.load(Ordering::SeqCst);
    after - before
}

/// Compile-time proof that a double-free is unrepresentable.
///
/// The following snippet is rejected by rustc E0382 "use of moved value".
///
/// ```compile_fail
/// use c1_rust_from_zero::l111_memory_safety::LoggedString;
/// let s = LoggedString::new("x");
/// drop(s);
/// drop(s); // E0382: use of moved value
/// ```
pub fn double_drop_is_unrepresentable() {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Tests here mutate the global DROP_COUNT, so they must not interleave
    // with each other. A single Mutex serializes the whole module.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn drop_runs_exactly_once_per_scope() {
        let _g = TEST_LOCK.lock().unwrap();
        assert_eq!(scoped_allocation("alice"), 1);
    }

    #[test]
    fn drop_runs_once_even_for_empty_string() {
        let _g = TEST_LOCK.lock().unwrap();
        assert_eq!(scoped_allocation(""), 1);
    }

    #[test]
    fn drop_runs_once_for_unicode() {
        let _g = TEST_LOCK.lock().unwrap();
        assert_eq!(scoped_allocation("日本語"), 1);
    }

    #[test]
    fn len_matches_input() {
        let _g = TEST_LOCK.lock().unwrap();
        let s = LoggedString::new("hello");
        assert_eq!(s.len(), 5);
        assert!(!s.is_empty());
    }

    #[test]
    fn empty_logged_string_reports_empty() {
        let _g = TEST_LOCK.lock().unwrap();
        let s = LoggedString::new("");
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn double_drop_marker_is_callable() {
        double_drop_is_unrepresentable();
    }
}
