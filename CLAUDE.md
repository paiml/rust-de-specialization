# rust-de-specialization

Rust for Data Engineering — 7-course Coursera specialization. Each course has a fruit-themed
Rust demo crate in `demos/` with provable contracts in `contracts/`.

## Structure

- `demos/c{1-7}-*/` — Cargo workspace members, each a runnable demo
- `contracts/fruit-*-v1.yaml` — Provable contract YAMLs (invariants, pre/postconditions)
- `contracts/binding.yaml` — Maps contracts → implemented functions
- `capstones/` — Per-course capstone project descriptions

## Theme

All 7 demos use **fruit** as the domain: fruit parser, fruit inventory, fruit ETL,
fruit orders, fruit analytics, fruit warehouse, fruit processing pipeline.

## Commands

```bash
make build          # Build all demos
make test           # Run all tests + capstone validation
make lint           # Clippy + capstone lint + pv validate contracts
make coverage       # cargo llvm-cov --fail-under-lines 100
make check          # Full gate: fmt + lint + test + coverage
make run-c1         # Run individual demo (c1 through c7)
make run-all        # Run all 7 demos
make comply         # pmat compliance check
make score          # pmat unified score
```

## Code Search

**MANDATORY: Use `pmat query` for ALL code search. NEVER use grep, rg, Glob, or Grep.**

```bash
pmat query "parse_line" --include-source --limit 5
pmat query "transaction" --literal --limit 10
pmat query "fruit" --faults --exclude-tests
```

## Contracts

Every public function has a provable contract in `contracts/`. Use `pv` to validate:

```bash
pv validate contracts/fruit-parser-v1.yaml
pv lint contracts/fruit-parser-v1.yaml
pv scaffold contracts/fruit-parser-v1.yaml    # Generate trait stubs
```

## Quality Gates

- 100% line coverage (enforced by `cargo llvm-cov --fail-under-lines 100`)
- Zero clippy warnings (`-D warnings`)
- All contracts valid (`pv validate`)
- All capstones structurally valid (`make test-capstones`)
