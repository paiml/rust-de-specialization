//! Lesson 2.1.2 — Move, Copy, and Clone: three ways values transfer.
//!
//! Contract: `contracts/c1-l212-move-copy-clone-v1.yaml`
//! Transcript anchor: "assignment transfers ownership for non-Copy types, Copy
//! duplicates stack-only types, Clone performs explicit deep copy"

/// CONTRACT: copy_i32_both_usable
/// Invariant: i32 implements Copy; both values remain usable and are equal.
pub fn copy_i32_both_usable() -> (i32, i32) {
    let x: i32 = 42;
    let y = x; // bit-wise copy, x still valid
    (x, y)
}

/// CONTRACT: move_string_transfers
/// Invariant: ownership transfers to the returned String; content preserved.
/// Compile-time enforced: the original binding in the caller is unusable after.
pub fn move_string_transfers(s: String) -> (usize, String) {
    let len = s.len();
    let moved = s; // ownership moves to `moved`
    (len, moved)
}

/// CONTRACT: clone_string_independent
/// Invariant: the two returned Strings have equal content but distinct heap
/// buffers — different pointers (when content is non-empty).
pub fn clone_string_independent(s: &str) -> (String, String) {
    let a = s.to_owned();
    let b = a.clone();
    (a, b)
}

/// Compile-time proof that use-after-move is rejected.
///
/// ```compile_fail
/// use c1_rust_from_zero::l212_move_copy_clone::move_string_transfers;
/// let s = String::from("alice");
/// let (_len, _moved) = move_string_transfers(s);
/// let _ = s.len(); // E0382: borrow of moved value
/// ```
pub fn use_after_move_is_rejected() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_preserves_both() {
        let (x, y) = copy_i32_both_usable();
        assert_eq!(x, y);
        assert_eq!(x, 42);
    }

    #[test]
    fn move_preserves_length() {
        let (len, moved) = move_string_transfers(String::from("alice"));
        assert_eq!(len, moved.len());
        assert_eq!(moved, "alice");
    }

    #[test]
    fn move_handles_empty_string() {
        let (len, moved) = move_string_transfers(String::new());
        assert_eq!(len, 0);
        assert!(moved.is_empty());
    }

    #[test]
    fn clone_content_equal() {
        let (a, b) = clone_string_independent("hello");
        assert_eq!(a, b);
        assert_eq!(a, "hello");
    }

    #[test]
    fn clone_heap_buffers_distinct() {
        let (a, b) = clone_string_independent("hello");
        // Heap buffers must be at distinct addresses for non-empty input.
        assert_ne!(a.as_ptr(), b.as_ptr());
    }

    #[test]
    fn clone_empty_string_still_equal() {
        let (a, b) = clone_string_independent("");
        assert_eq!(a, b);
        assert!(a.is_empty());
    }

    #[test]
    fn use_after_move_marker_is_callable() {
        use_after_move_is_rejected();
    }
}
