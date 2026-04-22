# Capstone Reading: Master Rust in the Playground

> **Goal:** Turn everything you learned in *Course 1 — Rust from Zero* into
> muscle memory by running, mutating, and breaking short programs in the
> **Rust Playground** — <https://play.rust-lang.org/>.

You will not install anything. You will not touch `cargo`. Every exercise
below is a self-contained program you paste into the Playground, click
**Run**, observe, then *modify* until you can predict the compiler's
verdict before pressing Run.

Pair this reading with the runnable demos in
[`demos/c1-rust-from-zero/src/`](../demos/c1-rust-from-zero/src/) — each
exercise links to its source demo so you can see the same contract
exercised inside a real Cargo crate.

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
| **Mode** (Debug / Release) | Optimization level | **Release** to see closures inline away |
| **Edition** (2015 / 2018 / 2021 / 2024) | Language edition | **2024** — the default this course targets |
| **Share** | Generates a permalink | Save your solutions |

> ⚠️ The Playground has **no network access** and **no filesystem**.
> You cannot `reqwest::get(...)` or read files. That limitation is a
> feature — it forces you to reason about the *language*, not I/O.

Click **Tools → Clippy** on the default hello-world template. Read the
output even when clippy is silent — silence is a grade.

---

## 2. Six Exercises, One Per Lesson

Each exercise maps 1:1 to a lesson you already watched. Work in order —
later exercises assume the earlier contracts.

### Exercise 1 — Memory Safety Without a Garbage Collector

**Lesson 1.1.1 · Demo:** [`l111_memory_safety.rs`](../demos/c1-rust-from-zero/src/l111_memory_safety.rs)

**[Try it in Playground →](https://play.rust-lang.org/?channel=stable&mode=debug&edition=2024&gist=3c10851a578b59352f8eef4eb8e6192c)**

**Contract to feel:** `Drop` runs **exactly once** per owned value, and
a double-free is not just unlikely — it is **unrepresentable** in source.

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
of `main`. Press Run. Read the error: `use of moved value`. That is
the *compile-time* proof that a double-free cannot ship in Rust.

### Exercise 2 — Stack vs Heap, One Owner, One Drop

**Lesson 2.1.1 · Demo:** [`l211_ownership.rs`](../demos/c1-rust-from-zero/src/l211_ownership.rs)

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
`borrow of moved value`. Read the suggestion — the compiler tells you
*how* to fix it. Apply the fix and verify the program still prints
both values.

**Break it on purpose:** Replace `take(hello)` with `take(&hello)` and
fix `take`'s signature accordingly. You just converted an owning API
into a borrowing one. Notice what the last line needs now.

### Exercise 3 — Move vs Copy vs Clone

**Lesson 2.1.2 · Demo:** [`l212_move_copy_clone.rs`](../demos/c1-rust-from-zero/src/l212_move_copy_clone.rs)

**[Try it in Playground →](https://play.rust-lang.org/?channel=stable&mode=debug&edition=2024&gist=eede4541b7569bea8a30afd1db4d8a20)** (this one is a *provable compile failure* — `E0382: borrow of moved value`)

**Contract to feel:** Primitive types implement `Copy` and are duplicated
implicitly. Heap-owning types (`String`, `Vec<T>`) are **moved** by
default, and require an explicit `.clone()` to duplicate.

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

**Lesson 2.3.2 · Demo:** [`l232_result_question.rs`](../demos/c1-rust-from-zero/src/l232_result_question.rs)

**[Try it in Playground →](https://play.rust-lang.org/?channel=stable&mode=debug&edition=2024&gist=b6626bbd41a71e75cb041bf05cc5ed3a)** (prints `oracle: 4/4 equivalent` when `?` and hand-written `match` agree across every input)

**Contract to feel:** `?` is **literally** the nested-match form you'd
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

### Exercise 5 — Closures: Fn, FnMut, FnOnce

**Lesson 3.3.1 · Demo:** [`l331_closures.rs`](../demos/c1-rust-from-zero/src/l331_closures.rs)

**Contract to feel:** How a closure *captures* its environment decides
which trait it implements. `Fn` is pure, `FnMut` mutates captured state,
`FnOnce` consumes it.

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
closure. Read the error. What trait did the compiler infer instead?

### Exercise 6 — Iterators: Lazy Pipelines

**Lesson 3.3.2 · Demo:** [`l332_iterators.rs`](../demos/c1-rust-from-zero/src/l332_iterators.rs)

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
is easier to *refactor*?

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

The four bugs to find and explain in one sentence each:

1. `main` uses `?` but its return type is `()`.
2. `prices` is moved into `safe_totals` and cannot be used again.
3. `safe_totals` moves `prices` into `totals`, which also returns —
   fine for the happy path, but what if you want to print *inside*
   `safe_totals` too?
4. Error type `&'static str` is fine to start but is a code smell —
   swap it for an `enum` with `thiserror::Error` so your errors carry
   context.

When your version compiles *and* handles an empty `Vec` cleanly, you
have demonstrated fluency in ownership, borrowing, `Result`, `?`, and
iterator consumption all at once.

---

## 4. How to Submit Your Work

The Playground's **Share** button generates a permalink. For each of
the six exercises + the final challenge:

1. Paste your final, clippy-clean solution into a fresh Playground tab.
2. Press **Tools → Rustfmt**, then **Tools → Clippy**, then **Run**.
3. Click **Share**, copy the URL.
4. Collect the 7 URLs in a single markdown file or gist.

Add the gist to your LinkedIn profile as a portfolio artifact:

1. Go to your LinkedIn profile → **Add profile section** → **Projects**
2. Title: *"Rust Playground Mastery — Rust for Data Engineering"*
3. Description: Summarize what each exercise proved about the language
4. Link: Your gist or GitHub repo containing the 7 Playground URLs

---

## 5. Where to Go Next

- **Run the full crate locally:** `cargo test -p c1-rust-from-zero --lib`
  in the repo root executes all 58 unit tests plus 3 `compile_fail` doc
  tests from the six demos.
- **Read the contracts:** Each demo has a matching YAML contract under
  [`contracts/`](../contracts/) that names every invariant the code
  must satisfy. This is the pattern Course 7 makes rigorous.
- **Keep breaking things:** The best way to master Rust is to write a
  program you believe will compile, be wrong about it, and learn why
  the compiler said no.

The Playground is your forever sandbox. Every future course in this
specialization will give you Playground-shaped snippets; if they fit
here, they fit anywhere.
