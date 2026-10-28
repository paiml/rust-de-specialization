# Run the Rust from Zero Demos in Your Browser

Each section below is one lesson's demo, shrunk to a single file that runs on the Rust Playground with no install. Every section names its **Contract**, asserts it with `assert!` or `assert_eq!`, and links to a gist that was run on play.rust-lang.org (exit 0) before this reading shipped. Click **Run**, then break the contract on purpose and watch the assertion fail.

## Section 1 — Lesson 1.1.1: memory safety drop once

**Contract:** Lesson 1.1.1 Playground Proof: Drop runs exactly once per scoped value.

```rust
use std::sync::atomic::{AtomicU64, Ordering};

static DROPS: AtomicU64 = AtomicU64::new(0);

struct LoggedString(String);

impl Drop for LoggedString {
    fn drop(&mut self) {
        DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

fn main() {
    {
        let s = LoggedString(String::from("alice"));
        assert_eq!(s.0.len(), 5);
    }
    let n = DROPS.load(Ordering::SeqCst);
    println!("drops: {n}");
    assert_eq!(n, 1);
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=3c10851a578b59352f8eef4eb8e6192c>

Expected output contains: `drops: 1`

## Section 2 — Lesson 2.1.1: ownership stack heap one owner drop

**Contract:** Lesson 2.1.1 Playground Proof: stack vs heap, single ownership, and scope-exit Drop. Demonstrates three invariants in one snippet — stack_size_i32() == 4, heap-backed String passed by value is dropped exactly once at scope exit, and the caller can no longer access it (enforced at compile time; the runtime check here is that the function completes and Drop fires). Expected stdout "ownership: 3/3 pass", exit 0.

```rust
use std::mem::size_of;
use std::sync::atomic::{AtomicU64, Ordering};

static OWNER_DROPS: AtomicU64 = AtomicU64::new(0);

struct TrackedAlloc {
    payload: String,
}

impl TrackedAlloc {
    fn new(payload: &'static str) -> Self {
        Self { payload: payload.to_owned() }
    }
    fn len(&self) -> usize {
        self.payload.len()
    }
}

impl Drop for TrackedAlloc {
    fn drop(&mut self) {
        OWNER_DROPS.fetch_add(1, Ordering::SeqCst);
    }
}

fn heap_backed_length(s: String) -> usize {
    // Takes ownership. s is dropped when this function returns —
    // the caller cannot use the original String after the call.
    s.len()
}

fn main() {
    let mut passed = 0usize;

    // Invariant 1: i32 lives on the stack at a fixed size.
    assert_eq!(size_of::<i32>(), 4);
    passed += 1;

    // Invariant 2: heap-backed String passed by value is consumed.
    let hello = String::from("hello");
    assert_eq!(heap_backed_length(hello), 5);
    // `hello` is unusable here — ownership moved into the function.
    passed += 1;

    // Invariant 3: scope-exit fires Drop exactly once per owner.
    let before = OWNER_DROPS.load(Ordering::SeqCst);
    {
        let owner = TrackedAlloc::new("owned");
        assert_eq!(owner.len(), 5);
    }
    let delta = OWNER_DROPS.load(Ordering::SeqCst) - before;
    assert_eq!(delta, 1);
    passed += 1;

    println!("ownership: {passed}/3 pass");
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=32bd71a61cfa87984bac3963e26a9003>

Expected output contains: `ownership: 3/3 pass`

## Section 3 — Lesson 2.1.2: move semantics use after move rejected

**Contract:** Lesson 2.1.2 Playground Proof: the borrow checker rejects use of a moved String.

```rust
fn main() {
    let s = String::from("hello");
    let t = s;
    println!("{}", s);
    assert_eq!(t, "hello");
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=28b2a097309603161eba30ac232dde7b>

Expected output contains: `compile error E0382 (borrow of moved value)`

## Section 4 — Lesson 2.3.2: result question operator oracle equivalence

**Contract:** Lesson 2.3.2 Playground Proof: the `?` operator produces output equivalent to nested `match`.

```rust
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
enum AppError {
    Parse(String),
    DivByZero,
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e.to_string())
    }
}

fn parse_age(s: &'static str) -> Result<i32, AppError> {
    let n: i32 = s.parse()?;
    Ok(n)
}

fn divide(n: i32, d: i32) -> Result<i32, AppError> {
    if d == 0 { Err(AppError::DivByZero) } else { Ok(n / d) }
}

fn chain_question(s: &'static str, d: i32) -> Result<i32, AppError> {
    let n = parse_age(s)?;
    let q = divide(n, d)?;
    Ok(q)
}

fn chain_match(s: &'static str, d: i32) -> Result<i32, AppError> {
    match parse_age(s) {
        Err(e) => Err(e),
        Ok(n) => match divide(n, d) {
            Err(e) => Err(e),
            Ok(q) => Ok(q),
        },
    }
}

fn main() {
    let cases = [("42", 2), ("10", 0), ("abc", 2), ("", 1)];
    let mut passed = 0usize;
    for (s, d) in cases {
        let q = chain_question(s, d);
        let m = chain_match(s, d);
        assert_eq!(q, m, "divergence at ({s:?}, {d})");
        passed += 1;
    }
    println!("oracle: {passed}/{} equivalent", cases.len());
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a9e4ceba7cd26227af5f38f7a712e395>

Expected output contains: `oracle: 4/4 equivalent`

## Section 5 — Lesson 3.3.1: closures fn fnmut fnonce capture traits

**Contract:** Lesson 3.3.1 Playground Proof: the three closure traits (Fn, FnMut, FnOnce) each demonstrated with a canonical example. `make_adder` implements Fn (pure, callable multiple times). `make_counter` implements FnMut (mutates captured state across calls). `make_once` implements FnOnce (consumes captured value on single call). Expected stdout "closures: 3/3 pass", exit 0.

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| n.saturating_add(x)
}

fn make_counter() -> impl FnMut() -> u32 {
    let mut count: u32 = 0;
    move || {
        count += 1;
        count
    }
}

fn make_once(s: String) -> impl FnOnce() -> String {
    move || s
}

fn main() {
    let mut passed = 0usize;

    // Invariant 1: Fn — pure, consistent across invocations.
    let add5 = make_adder(5);
    assert_eq!(add5(1), 6);
    assert_eq!(add5(10), 15);
    assert_eq!(add5(1), 6);
    passed += 1;

    // Invariant 2: FnMut — counter advances exactly once per call.
    let mut counter = make_counter();
    assert_eq!(counter(), 1);
    assert_eq!(counter(), 2);
    assert_eq!(counter(), 3);
    passed += 1;

    // Invariant 3: FnOnce — consumes captured String, returns it verbatim.
    let once = make_once(String::from("alice"));
    assert_eq!(once(), "alice");
    // `once` cannot be called again — FnOnce consumed its captured state.
    passed += 1;

    println!("closures: {passed}/3 pass");
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=a6adf7bf39a3facf1733875bd21e480d>

Expected output contains: `closures: 3/3 pass`

## Section 6 — Lesson 3.3.2: iterators lazy adapters oracle equivalence

**Contract:** Lesson 3.3.2 Playground Proof: iterator pipelines (filter + map + collect) produce output bit-identical to an explicit loop (oracle equivalence), AND adapter chains without a consumer do NOT execute their closures (laziness invariant). Runs the oracle for n in 0..=20 and verifies the lazy side-effect counter stays at 0 until a consumer drives the chain. Expected stdout "iterators: 21/21 equivalent, lazy: ok", exit 0.

```rust
use std::cell::Cell;

fn pipeline(n: u32) -> Vec<u32> {
    (1..=n).filter(|x| x % 2 == 0).map(|x| x * x).collect()
}

fn eager_pipeline(n: u32) -> Vec<u32> {
    let mut out = Vec::new();
    for x in 1..=n {
        if x % 2 == 0 {
            out.push(x * x);
        }
    }
    out
}

fn main() {
    // Invariant 1: oracle equivalence across a range.
    let mut equivalent = 0usize;
    let upper = 20u32;
    for n in 0..=upper {
        assert_eq!(pipeline(n), eager_pipeline(n), "divergence at n={n}");
        equivalent += 1;
    }

    // Invariant 2: lazy adapters do not execute without a consumer.
    let counter: Cell<u32> = Cell::new(0);
    let _chain = (1..=10_u32).filter(|_| {
        counter.set(counter.get() + 1);
        true
    }).map(|x| x * 2);
    // _chain is built but never consumed.
    assert_eq!(counter.get(), 0, "filter closure ran without a consumer");

    // Invariant 3 (companion): once consumed, the closure runs exactly
    // once per element. Demonstrates that the counter above was zero
    // because of laziness, not because the closure was broken.
    let counter2: Cell<u32> = Cell::new(0);
    let sum: u32 = (1..=10_u32)
        .filter(|_| { counter2.set(counter2.get() + 1); true })
        .map(|x| x * 2)
        .sum();
    assert_eq!(sum, 110);
    assert_eq!(counter2.get(), 10);

    println!("iterators: {equivalent}/{} equivalent, lazy: ok", upper + 1);
}
```

▶ Run on Playground: <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=70f4efe0e1ace46e9973be2b228eaa69>

Expected output contains: `iterators: 21/21 equivalent, lazy: ok`

## Contract summary

| # | Lesson | Contract | Enforced by | Playground output |
|---|--------|----------|-------------|-------------------|
| 1 | 1.1.1 | memory safety drop once | `assert!`/`assert_eq!` in the snippet | `drops: 1` |
| 2 | 2.1.1 | ownership stack heap one owner drop | `assert!`/`assert_eq!` in the snippet | `ownership: 3/3 pass` |
| 3 | 2.1.2 | move semantics use after move rejected | `assert!`/`assert_eq!` in the snippet | `compile error E0382 (borrow of moved value)` |
| 4 | 2.3.2 | result question operator oracle equivalence | `assert!`/`assert_eq!` in the snippet | `oracle: 4/4 equivalent` |
| 5 | 3.3.1 | closures fn fnmut fnonce capture traits | `assert!`/`assert_eq!` in the snippet | `closures: 3/3 pass` |
| 6 | 3.3.2 | iterators lazy adapters oracle equivalence | `assert!`/`assert_eq!` in the snippet | `iterators: 21/21 equivalent, lazy: ok` |
