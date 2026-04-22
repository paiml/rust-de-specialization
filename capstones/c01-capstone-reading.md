# Capstone Reading: Master Rust in the Playground

> **Goal:** Turn everything you learned in *Course 1 — Rust from Zero* into
> muscle memory by running, mutating, and breaking short programs in the
> **Rust Playground** — <https://play.rust-lang.org/>.

You will not install anything. You will not touch `cargo`. Every exercise
below is a self-contained program you paste into the Playground, click
**Run**, observe, then *modify* until you can predict the compiler's
verdict before pressing Run.

Pair this reading with the runnable demos in
[`demos/c1-rust-from-zero/src/`](https://github.com/paiml/rust-de-specialization/tree/main/demos/c1-rust-from-zero/src)
— each exercise links to its source demo so you can see the same contract
exercised inside a real Cargo crate.

This is an alternative to the local-project Capstone (`c01-capstone.md`),
intended for learners on locked-down machines or anyone who prefers to start
before setting up a local toolchain. Completing both tracks is not required.

---

## 1. Anatomy of `play.rust-lang.org`

Open <https://play.rust-lang.org/> in a new tab. Before you write any
code, locate these controls — the rest of this reading assumes you know
where each lives.

| Control | What it does | When to use it |
|---------|--------------|----------------|
| **Run** | Compiles and executes `main` | Every exercise starts here |
| **Build** | Compiles only; emits assembly/LLVM/MIR | Inspect what the compiler produced |
| **Test** | Runs `#[test]` functions without a `main` | Exercises 4 and 7 below |
| **Tools → Rustfmt** | Formats your code canonically | Before sharing |
| **Tools → Clippy** | Lints for idioms and bugs | Every exercise — clippy teaches |
| **Tools → Miri** | Detects undefined behavior at runtime | When you want to *prove* safety |
| **Channel** (Stable / Beta / Nightly) | Picks the toolchain | Leave on **Stable** |
| **Mode** (Debug / Release) | Optimization level | Use **Release** to see closures and iterators inline away |
| **Edition** (2015 / 2018 / 2021 / 2024) | Language edition | **2024** — stable since Rust 1.85 (Feb 2025); the default this course targets |
| **Share** | Generates a GitHub Gist permalink | Save your solutions |

> ⚠️ The Playground has **no network access** and **no filesystem**. You
> cannot `reqwest::get(...)` or read files. That limitation is a feature —
> it forces you to reason about the *language*, not I/O. It also means the
> multi-file module exercises from the local Capstone track cannot be
> reproduced here; keep those for the local-project path.

Click **Tools → Clippy** on the default hello-world template. Read the
output even when clippy is silent — silence is a grade.

The Playground provides the top 100 crates from crates.io plus the Rust
Cookbook crates and their dependencies, so `serde`, `thiserror`, `anyhow`,
and `regex` are available. See the full list at
<https://play.rust-lang.org/help> under "Features → Crates."

---

## 2. Six Exercises — One Per Key Concept

Each exercise maps to one of the core concepts from Course 1. Work in
order — later exercises assume earlier contracts.

The six exercises cover:

1. **Drop determinism and compile-time memory safety** (Lessons 1.1.1, 2.1.1)
2. **Ownership and stack-vs-heap** (Lesson 2.1.1)
3. **Move, Copy, and Clone** (Lesson 2.1.2)
4. **`Result` and the `?` operator** (Lesson 2.3.2)
5. **Closures and the `Fn` / `FnMut` / `FnOnce` traits** (Lesson 3.3.1)
6. **Lazy iterator pipelines** (Lesson 3.3.2)

A final mastery challenge then integrates all six.

### Exercise 1 — Memory Safety Without a Garbage Collector

**Lessons 1.1.1 and 2.1.1 · Demo:** [`l111_memory_safety.rs`](https://github.com/paiml/rust-de-specialization/blob/main/demos/c1-rust-from-zero/src/l111_memory_safety.rs)

**Contract to feel:** `Drop` runs at a known, deterministic point for
each owned value that's actually dropped — and *double-free* is not just
unlikely, it is **unrepresentable** in source. (Leaks are still possible
via `Rc` cycles, `Box::leak`, or `mem::forget` — Rust guarantees no
double-free and no use-after-free, not universal reclamation.)

Paste this into the Playground and press **Run**:

```rust
struct Loud(&'static str);

impl Drop for Loud {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn main() {
    let a = Loud("alpha");
    {
        let b = Loud("beta");
        println!("b is alive");
    } // b dropped here
    println!("a is still alive");
} // a dropped here
```

**Predict before running:** Which order do the two drops print in?

**Your challenge:** Add a third value `Loud("gamma")` at the top of
`main` and predict the drop order. LIFO? FIFO? Run it to check.

**Break it on purpose:** Add `drop(a); drop(a);` as the last two lines
of `main`. Press Run. Read the error: *use of moved value*. That is
the compile-time proof that a double-free cannot ship in Rust.

### Exercise 2 — Stack vs Heap, One Owner, One Drop

**Lesson 2.1.1 · Demo:** [`l211_ownership.rs`](https://github.com/paiml/rust-de-specialization/blob/main/demos/c1-rust-from-zero/src/l211_ownership.rs)

**Contract to feel:** Every heap allocation has exactly one owner, and
when that owner goes out of scope its heap buffer is freed — once.

```rust
fn take(s: String) -> usize {
    s.len()
} // s is dropped here — the String's heap buffer is freed

fn main() {
    let hello = String::from("hello");
    let n = take(hello);
    println!("{n}");
    // Try uncommenting this:
    // println!("{hello}");
}
```

**Your challenge:** Uncomment the last line. The error mentions
*borrow of moved value*. Read the suggestion — the compiler tells you
*how* to fix it. Apply the fix and verify the program still prints
both values.

**Break it on purpose:** Replace `take(hello)` with `take(&hello)` and
fix `take`'s signature accordingly. You just converted an owning API
into a borrowing one. Notice what the last line needs now.

### Exercise 3 — Move vs Copy vs Clone

**Lesson 2.1.2 · Demo:** [`l212_move_copy_clone.rs`](https://github.com/paiml/rust-de-specialization/blob/main/demos/c1-rust-from-zero/src/l212_move_copy_clone.rs)

**Contract to feel:** Primitive types implement `Copy` and are duplicated
implicitly on assignment. Heap-owning types (`String`, `Vec<T>`) are
*moved* by default, and require an explicit `.clone()` to duplicate.

```rust
fn main() {
    // Copy — both usable
    let x: i32 = 42;
    let y = x;
    println!("x={x}, y={y}");

    // Move — original is gone
    let a = String::from("hello");
    let b = a;
    println!("b={b}");
    // println!("a={a}"); // rejected: borrow of moved value

    // Clone — explicit deep copy
    let c = String::from("world");
    let d = c.clone();
    println!("c={c}, d={d}");

    // Prove the clone has its own heap buffer
    println!("c addr = {:p}", c.as_ptr());
    println!("d addr = {:p}", d.as_ptr());
}
```

**Predict before running:** Will `c.as_ptr() == d.as_ptr()`?

**Your challenge:** Add `#[derive(Clone)]` to a custom struct and
demonstrate that moving it works but cloning preserves both copies:

```rust
#[derive(Clone, Debug)]
struct Record { name: String, age: u32 }
```

### Exercise 4 — `Result` and the `?` Operator

**Lesson 2.3.2 · Demo:** [`l232_result_question.rs`](https://github.com/paiml/rust-de-specialization/blob/main/demos/c1-rust-from-zero/src/l232_result_question.rs)

**Contract to feel:** `?` is *literally* the nested-match form you'd
write by hand — same behavior, shorter code. The two versions are
byte-for-byte equivalent across every input.

Switch the Playground to **Test** mode for this one.

```rust
#[derive(Debug, PartialEq, Eq)]
enum AppError {
    Parse,
    DivByZero,
}

fn parse_age(s: &str) -> Result<u32, AppError> {
    s.parse::<u32>().map_err(|_| AppError::Parse)
}

fn divide(n: u32, d: u32) -> Result<u32, AppError> {
    n.checked_div(d).ok_or(AppError::DivByZero)
}

// Using ?
fn chain_question(s: &str, d: u32) -> Result<u32, AppError> {
    let n = parse_age(s)?;
    let q = divide(n, d)?;
    Ok(q)
}

// Using explicit match — the oracle
fn chain_match(s: &str, d: u32) -> Result<u32, AppError> {
    match parse_age(s) {
        Ok(n) => match divide(n, d) {
            Ok(q) => Ok(q),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

#[test]
fn equivalence() {
    for s in ["42", "abc", "0", "-1"] {
        for d in [0u32, 1, 2, 7] {
            assert_eq!(chain_question(s, d), chain_match(s, d));
        }
    }
}
```

**Your challenge:** Click **Test**. Add a new error variant (say
`Overflow`) and a new primitive that can produce it. Extend both
`chain_question` and `chain_match` to propagate it. The `equivalence`
test must still pass — that is your proof you desugared `?` correctly.

### Exercise 5 — Closures: `Fn`, `FnMut`, `FnOnce`

**Lesson 3.3.1 · Demo:** [`l331_closures.rs`](https://github.com/paiml/rust-de-specialization/blob/main/demos/c1-rust-from-zero/src/l331_closures.rs)

**Contract to feel:** How a closure *captures* its environment decides
which trait it implements. `Fn` is pure, `FnMut` mutates captured state,
`FnOnce` consumes something and can therefore be called only once.

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| n + x
}

fn make_counter() -> impl FnMut() -> u32 {
    let mut count = 0u32;
    move || { count += 1; count }
}

fn make_once(s: String) -> impl FnOnce() -> String {
    move || s  // captures s by value, returns it — consumes the closure
}

fn main() {
    let add5 = make_adder(5);
    println!("{} {} {}", add5(1), add5(10), add5(1));

    let mut c = make_counter();
    println!("{} {} {} {}", c(), c(), c(), c());

    let hello = make_once(String::from("hello"));
    println!("{}", hello());
    // println!("{}", hello()); // rejected: FnOnce consumed
}
```

**Predict before running:** What does the counter print?

**Your challenge:** Uncomment the second `hello()` call. Read the
error — `FnOnce` closures cannot be called twice. Now refactor
`make_once` to return `impl FnMut() -> String` using `.clone()`
internally. Verify the second call works.

**Break it on purpose:** Change `make_adder` to mutate `n` inside the
closure. Read the error. Which trait did the compiler infer instead?

### Exercise 6 — Iterators: Lazy Pipelines

**Lesson 3.3.2 · Demo:** [`l332_iterators.rs`](https://github.com/paiml/rust-de-specialization/blob/main/demos/c1-rust-from-zero/src/l332_iterators.rs)

**Contract to feel:** Iterator adapters like `.map` and `.filter` are
**lazy**. They do nothing until a *consumer* (`.collect()`, `.sum()`,
`.count()`, a `for` loop) pulls on them.

```rust
use std::cell::Cell;

fn main() {
    let counter = Cell::new(0u32);

    // Build the chain — but never consume it.
    let _chain = (1..=10_u32)
        .filter(|_| { counter.set(counter.get() + 1); true })
        .map(|x| x * 2);

    println!("lazy: filter ran {} times", counter.get());
    // ^ prints 0 — closures never execute without a consumer

    // Same chain, now consumed with .sum()
    let counter = Cell::new(0u32);
    let total: u32 = (1..=10_u32)
        .filter(|_| { counter.set(counter.get() + 1); true })
        .map(|x| x * 2)
        .sum();
    println!("eager: filter ran {} times, total = {}", counter.get(), total);
}
```

**Predict before running:** What do the two `counter.get()` values print?

**Your challenge:** Replace `.sum()` with `.count()`. The total goes
away, but what does the filter counter read now? Why?

**Then:** Write a pipeline that takes `1..=n`, keeps even numbers,
squares them, and collects into a `Vec<u32>`. Compare your iterator
pipeline to an explicit `for` loop. Which one is easier to read? Which
is easier to *refactor* (say, to add a `take(10)` in the middle)?

---

## 3. Final Mastery Challenge

Paste this into the Playground. It will **not compile as written**.
Your job: fix every error without using `.clone()`, `unwrap()`, or
`expect()`.

```rust
fn totals(prices: Vec<f64>) -> (f64, f64) {
    let sum: f64 = prices.iter().sum();
    let avg = sum / prices.len() as f64;
    println!("sum={sum}, avg={avg}, n={}", prices.len());
    (sum, avg)
}

fn safe_totals(prices: Vec<f64>) -> Result<(f64, f64), &'static str> {
    if prices.is_empty() {
        return Err("no prices");
    }
    let (sum, avg) = totals(prices);
    Ok((sum, avg))
}

fn main() {
    let prices = vec![9.99, 4.50, 12.00];
    let report = safe_totals(prices)?;
    println!("{report:?}");

    // Re-use prices to print a summary
    println!("{} items", prices.len());
}
```

Four bugs to find and explain in one sentence each:

1. `main` uses `?` but its return type is `()`. What's the signature fix
   that lets `?` work from `main`?
2. `prices` is moved into `safe_totals` and cannot be used again
   afterwards. Which ownership tool from Week 2 fixes this without
   cloning?
3. `safe_totals` moves `prices` into `totals`, which also returns a
   value — fine for the happy path, but what if you wanted to log
   something about `prices` *after* calling `totals`?
4. The error type `&'static str` is fine to start but is a code smell as
   the surface grows. Swap it for an `enum` with a `From` impl, or use
   the `thiserror` crate (available in the Playground) to derive it.

When your version compiles *and* handles an empty `Vec` cleanly, you
have demonstrated fluency in ownership, borrowing, `Result`, `?`, and
iterator consumption all at once.

---

## 4. How to Submit Your Work

The Playground's **Share** button generates a permalink (backed by a
GitHub Gist). For each of the six exercises plus the final challenge:

1. Paste your final, clippy-clean solution into a fresh Playground tab.
2. Press **Tools → Rustfmt**, then **Tools → Clippy**, then **Run**.
3. Click **Share**, copy the URL.
4. Collect the 7 URLs in a single markdown file or gist.

Add the gist to your LinkedIn profile as a portfolio artifact:

1. On your LinkedIn profile, choose **Add profile section** → **Projects**.
2. Title: *Rust Playground Mastery — Rust for Data Engineering*.
3. Description: Summarize what each exercise proved about the language.
4. Link: Your gist or GitHub repo containing the 7 Playground URLs.

---

## 5. Where to Go Next

- **Run the full crate locally:** `cargo test -p c1-rust-from-zero --lib`
  in the repo root executes the full test suite and compile-fail doctests
  from the six demos.
- **Read the contracts:** Each demo has a matching contract file under
  [`contracts/`](https://github.com/paiml/rust-de-specialization/tree/main/contracts)
  that names every invariant the code must satisfy. This is the pattern
  Course 7 of the specialization makes rigorous.
- **Keep breaking things:** The best way to master Rust is to write a
  program you believe will compile, be wrong about it, and learn why
  the compiler said no.

## References

Official Rust documentation:

- **The Book**: <https://doc.rust-lang.org/book/>
- **The Rust Playground help page**: <https://play.rust-lang.org/help>
- **Standard library: `Iterator`**: <https://doc.rust-lang.org/std/iter/trait.Iterator.html>
- **Standard library: `Option`** and **`Result`**: <https://doc.rust-lang.org/std/option/enum.Option.html>, <https://doc.rust-lang.org/std/result/enum.Result.html>

The Playground is your forever sandbox. Every future course in this
specialization will give you Playground-shaped snippets; if they fit
here, they fit anywhere.
