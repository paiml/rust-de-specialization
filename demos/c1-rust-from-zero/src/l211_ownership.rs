//! Lesson 2.1.1 — Ownership: stack vs heap, one owner, Drop on scope exit.
//!
//! Contract: `contracts/c1-l211-ownership-v1.yaml`
//! Transcript anchor: "every Rust program has two places to put things, the
//! stack and heap"

use std::mem::size_of;
use std::sync::atomic::{AtomicU64, Ordering};

static OWNER_DROPS: AtomicU64 = AtomicU64::new(0);

/// Heap-backed allocation whose Drop fires exactly once.
pub struct TrackedAlloc {
    payload: String,
}

impl TrackedAlloc {
    pub fn new(payload: &str) -> Self {
        Self {
            payload: payload.to_owned(),
        }
    }

    pub fn len(&self) -> usize {
        self.payload.len()
    }

    pub fn is_empty(&self) -> bool {
        self.payload.is_empty()
    }
}

impl Drop for TrackedAlloc {
    fn drop(&mut self) {
        OWNER_DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

/// CONTRACT: stack_size_i32
/// Invariant: returns std::mem::size_of::<i32>() = 4 bytes.
pub fn stack_size_i32() -> usize {
    size_of::<i32>()
}

/// CONTRACT: heap_backed_length
/// Invariant: takes ownership of s and returns its byte length.
/// Postcondition: heap buffer is freed before the function returns.
pub fn heap_backed_length(s: String) -> usize {
    s.len()
    // s dropped here — String's Drop frees the heap buffer.
}

/// CONTRACT: one_owner_one_drop
/// Invariant: exactly one Drop call per TrackedAlloc instance.
/// Postcondition: return value equals the number of drops observed.
pub fn one_owner_one_drop() -> u64 {
    let before = OWNER_DROPS.load(Ordering::SeqCst);
    {
        let owner = TrackedAlloc::new("owned");
        let _ = owner.len();
    }
    let after = OWNER_DROPS.load(Ordering::SeqCst);
    after - before
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Tests here mutate OWNER_DROPS; serialize so parallel drops don't
    // contaminate the before/after delta read by one_owner_one_drop.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn i32_is_four_bytes() {
        assert_eq!(stack_size_i32(), 4);
    }

    #[test]
    fn heap_length_matches_input() {
        assert_eq!(heap_backed_length(String::from("hello")), 5);
    }

    #[test]
    fn heap_length_empty_is_zero() {
        assert_eq!(heap_backed_length(String::new()), 0);
    }

    #[test]
    fn scope_exit_drops_exactly_once() {
        let _g = TEST_LOCK.lock().unwrap();
        assert_eq!(one_owner_one_drop(), 1);
    }

    #[test]
    fn tracked_alloc_reports_length() {
        let _g = TEST_LOCK.lock().unwrap();
        let a = TrackedAlloc::new("abc");
        assert_eq!(a.len(), 3);
        assert!(!a.is_empty());
    }

    #[test]
    fn empty_tracked_alloc_reports_empty() {
        let _g = TEST_LOCK.lock().unwrap();
        let a = TrackedAlloc::new("");
        assert_eq!(a.len(), 0);
        assert!(a.is_empty());
    }
}
