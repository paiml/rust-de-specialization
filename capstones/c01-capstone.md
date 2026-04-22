# Capstone: Rust from Zero

## Project Overview

Build a command-line data-processing tool that reads structured input (CSV or
JSON), transforms it using iterators and closures, and writes the output in a
different format. The project exercises everything from Course 1: ownership and
borrowing, recoverable error handling with `Result` and `?`, modules and a
deliberate public API, generics and trait bounds, and lazy iterator pipelines.

Two tracks are available — choose the one that matches your setup:

1. **Local track** (this reading) — build a full Cargo project with tests, linting,
   and a clippy-clean build. This is the standard path.
2. **Playground track** — complete a zero-install version in the Rust Playground.
   See the *Playground Capstone Reading* for the alternative path, which uses the
   same concepts in six smaller exercises plus a final mastery challenge.

## Sample Input

To keep evaluations comparable, the project uses a small shared dataset. Either
construct or download a CSV with the following shape (roughly 50–200 rows):

```
id,name,region,amount,active
1,Alice,EU,1200.50,true
2,Bob,US,850.00,true
3,Carol,APAC,0.00,false
...
```

You will filter, transform, aggregate, or reshape this data. A JSON version of
the same shape is equally acceptable.

## Deliverables

### 1. CLI data transformer

A Cargo project with:

- At least two library modules (e.g., `parser`, `transform`, `io`) split into
  separate files under `src/`, wired together from `src/lib.rs` with `mod`
  declarations.
- One binary crate (`src/main.rs` or `src/bin/<n>.rs`) that parses CLI
  arguments, reads the input file, applies a user-specified transformation
  (filter, map, aggregate), and writes the result to stdout or an output file.
- All I/O and parsing errors propagated via `Result` with meaningful context —
  either a custom error enum or `Box<dyn Error>`. No `panic!`, `unwrap`, or
  `expect` on anything that handles external input.

### 2. Ownership and borrowing demonstration

At least three functions in the codebase must illustrate distinct borrowing
patterns, each with a doc comment explaining why that signature was chosen:

- A function that takes `&T` or `&[T]` for shared read-only access.
- A function that takes `&mut T` or `&mut [T]` for in-place mutation.
- A function that takes `T` or `Vec<T>` by value because it genuinely consumes
  the data.

At least one function should also demonstrate a generic bound (e.g.,
`fn summarize<T: Into<f64> + Copy>(xs: &[T]) -> f64`).

### 3. Test suite

- **Unit tests** for each library module covering the happy path and at least
  two distinct error cases per public function.
- **At least one integration test** under `tests/` that runs the binary
  end-to-end against a small fixture input.
- The full suite must pass with `cargo test`.
- The full build must pass with `cargo clippy -- -D warnings` (warnings treated
  as errors).

Coverage measurement is optional; a useful rule of thumb is "every public
function has at least one test and every error path has at least one test."

## Evaluation Criteria

### Distinction

- **Zero `unwrap` or `expect` on external-input paths.** Startup-time invariant
  assertions with `expect("documented invariant")` are acceptable.
- Iterator chains replace manual `for` loops wherever applicable, and the chains
  stay lazy — no premature `.collect()` materializing throwaway `Vec`s between
  adapters.
- Modules are split into separate files with a deliberate public API (minimal
  `pub`, `pub(crate)` for internal sharing, optional `pub use` re-exports at
  `lib.rs` for ergonomic flatter paths).
- At least one generic function with a minimal, appropriate trait bound.
- Custom error type defined as an `enum` with `From` impls (or `thiserror`) —
  not just `Box<dyn Error>` everywhere.

### Proficient

- Tool compiles, runs, and produces correct output for the sample dataset.
- Ownership and borrowing patterns are correct; no unnecessary `.clone()` calls.
- Tests cover happy path and at least two error cases.
- Clippy passes with default lints (without `-D warnings`).

### Developing

- Tool compiles and produces partial output.
- Some borrowing issues resolved with `.clone()` rather than by restructuring.
- Minimal test coverage (happy path only).
- Clippy warnings present.

## Resources

Official Rust documentation you'll use throughout the capstone:

- **The Book**: <https://doc.rust-lang.org/book/>
- **The Cargo Book**: <https://doc.rust-lang.org/cargo/>
- **The Rustonomicon** — if you find yourself needing `unsafe`, stop and reach for a safer design: <https://doc.rust-lang.org/nomicon/>
- **Standard library: `Iterator`**: <https://doc.rust-lang.org/std/iter/trait.Iterator.html>

Ecosystem crates that pair well with this capstone:

- **`clap`** — command-line argument parsing: <https://docs.rs/clap/>
- **`serde`** + **`serde_json`** / **`csv`** — structured input parsing: <https://serde.rs/>
- **`thiserror`** — derive `Error` impls for a custom error enum: <https://docs.rs/thiserror/>
- **`anyhow`** — ergonomic `Result<T>` for application `main`: <https://docs.rs/anyhow/>

Reference implementation and contract-test fixtures live in the specialization
repository:

- **Course 1 demos and capstone reference**: <https://github.com/paiml/rust-de-specialization/tree/main/demos/c1-rust-from-zero>

## Share Your Work

Add this capstone to your LinkedIn profile as a portfolio project:

1. On your LinkedIn profile, choose **Add profile section** → **Projects**.
2. **Title:** *Rust from Zero — Rust for Data Engineering Specialization*.
3. **Description:** Summarize the deliverables — CLI tool, module split, test
   coverage, and the specific ownership/borrowing/iterator decisions you made.
4. **Link:** the URL to your public GitHub repository for the project.
