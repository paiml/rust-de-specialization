//! Lesson 3.3.1 — Closures: Fn, FnMut, FnOnce capture modes.
//!
//! Contract: `contracts/c1-l331-closures-v1.yaml`
//! Transcript anchor: "Fn, FnMut, FnOnce — the three traits that describe how
//! a closure captures"

/// CONTRACT: make_adder (Fn — immutable capture)
/// Invariant: for all (n, x): make_adder(n)(x) == n.saturating_add(x).
/// The returned closure is pure and callable repeatedly.
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| n.saturating_add(x)
}

/// CONTRACT: make_counter (FnMut — mutable capture)
/// Invariant: the k-th call returns exactly k (1-indexed).
pub fn make_counter() -> impl FnMut() -> u32 {
    let mut count: u32 = 0;
    move || {
        count += 1;
        count
    }
}

/// CONTRACT: make_once (FnOnce — consuming capture)
/// Invariant: returns the captured String byte-identical to input; can only
/// be invoked once (compile-time enforced by FnOnce).
pub fn make_once(s: String) -> impl FnOnce() -> String {
    move || s
}

/// Compile-time proof that FnOnce closures cannot be called twice.
///
/// ```compile_fail
/// use c1_rust_from_zero::l331_closures::make_once;
/// let f = make_once(String::from("alice"));
/// let _ = f();
/// let _ = f(); // E0382: use of moved value (FnOnce consumed its captured state)
/// ```
pub fn fnonce_is_single_use() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_adder_is_pure() {
        let add5 = make_adder(5);
        assert_eq!(add5(1), 6);
        assert_eq!(add5(10), 15);
        // Multiple invocations yield consistent results.
        assert_eq!(add5(1), 6);
    }

    #[test]
    fn fn_adder_saturates() {
        let add_max = make_adder(i32::MAX);
        assert_eq!(add_max(1), i32::MAX);
        assert_eq!(add_max(-1), i32::MAX - 1);
    }

    #[test]
    fn fnmut_counter_is_monotonic() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
        assert_eq!(counter(), 4);
    }

    #[test]
    fn fnmut_counter_starts_at_one() {
        let mut counter = make_counter();
        assert_eq!(counter(), 1);
    }

    #[test]
    fn fnonce_returns_captured_value() {
        let f = make_once(String::from("alice"));
        assert_eq!(f(), "alice");
    }

    #[test]
    fn fnonce_handles_empty_string() {
        let f = make_once(String::new());
        assert!(f().is_empty());
    }

    #[test]
    fn fnonce_marker_is_callable() {
        fnonce_is_single_use();
    }
}
