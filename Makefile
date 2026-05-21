SHELL := /bin/bash
.DEFAULT_GOAL := help

DEMOS := c1-rust-from-zero c2-sqlite-rust c3-etl-pipelines c4-sql-databases \
         c5-polars c6-duckdb c7-provable-contracts

.PHONY: help build test test-fast test-capstones lint lint-capstones lint-contracts \
        fmt fmt-check coverage check bench run-all clean \
        $(addprefix run-,$(subst -,_,c1 c2 c3 c4 c5 c6 c7))

# ─── Help ───────────────────────────────────────────────────────────────────────

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ─── Build ──────────────────────────────────────────────────────────────────────

build: ## Build all demo crates
	cargo build --workspace

build-release: ## Build all demo crates (release)
	cargo build --workspace --release

# ─── Run individual demos ────────────────────────────────────────────────────────

run-c1: ## Run Course 1: Rust from Zero (fruit parser)
	cargo run -p c1-rust-from-zero

run-c2: ## Run Course 2: SQLite for Rust (fruit inventory)
	cargo run -p c2-sqlite-rust

run-c3: ## Run Course 3: ETL Pipelines (fruit ETL)
	cargo run -p c3-etl-pipelines

run-c4: ## Run Course 4: SQL Databases (fruit orders)
	cargo run -p c4-sql-databases

run-c5: ## Run Course 5: Polars (fruit analytics)
	cargo run -p c5-polars

run-c6: ## Run Course 6: DuckDB (fruit warehouse)
	cargo run -p c6-duckdb

run-c7: ## Run Course 7: Provable Contracts (fruit pipeline)
	cargo run -p c7-provable-contracts

run-all: ## Run all 7 demos sequentially
	@for pkg in $(DEMOS); do \
		echo ""; \
		echo "━━━ $$pkg ━━━"; \
		cargo run -p $$pkg || exit 1; \
	done

# ─── Test ────────────────────────────────────────────────────────────────────────

test: test-capstones ## Run all Rust tests + capstone validation
	@echo "=== Rust tests ==="
	cargo test --workspace
	@echo "=== All tests passed ==="

test-fast: ## Rust tests only (no capstone validation)
	cargo test --workspace

test-capstones: ## Validate 7 capstones and course structure
	@echo "=== Validating 7-course structure ==="
	@test $$(ls capstones/c0[0-9]-capstone.md 2>/dev/null | wc -l) -eq 7 || \
		{ echo "FAIL: expected 7 capstones, found $$(ls capstones/c0[0-9]-capstone.md 2>/dev/null | wc -l)"; exit 1; }
	@echo "  7 capstone files present"
	@for i in $$(seq -w 1 7); do \
		f="capstones/c0$$i-capstone.md"; \
		grep -q "## Deliverables" "$$f" || { echo "FAIL: $$f missing Deliverables section"; exit 1; }; \
		grep -q "## Evaluation Criteria" "$$f" || { echo "FAIL: $$f missing Evaluation Criteria"; exit 1; }; \
		grep -q "## Share Your Work" "$$f" || { echo "FAIL: $$f missing Share Your Work"; exit 1; }; \
	done
	@echo "  All capstones have required sections"
	@grep -qi "31-course" README.md || { echo "FAIL: README must reference the \"31-Course\" specialization"; exit 1; }
	@echo "  README references 31 courses"
	@echo "=== Capstone validation passed ==="

# ─── Lint ────────────────────────────────────────────────────────────────────────

lint: lint-capstones lint-contracts ## Clippy + capstone lint + contract lint
	@echo "=== Clippy ==="
	cargo clippy --workspace -- -D warnings
	@echo "=== All lint passed ==="

lint-capstones: ## Lint markdown + verify capstone links
	@echo "=== Markdown lint ==="
	@find . -name '*.md' -not -path './.git/*' -not -path './target/*' | \
		xargs -I{} sh -c 'grep -Pn "\t" "$$1" && echo "FAIL: tabs in $$1" && exit 1 || true' _ {}
	@echo "=== Checking capstone links ==="
	@for i in $$(seq -w 1 7); do \
		f="capstones/c0$$i-capstone.md"; \
		[ -f "$$f" ] || { echo "MISSING: $$f"; exit 1; }; \
	done
	@echo "=== Capstone lint passed ==="

lint-contracts: ## Validate all provable contracts with pv
	@echo "=== Contract validation (pv lint) ==="
	@for f in contracts/fruit-*.yaml; do \
		pv validate "$$f" 2>&1 | tail -1 | grep -q "valid" || \
			{ echo "FAIL: $$f"; pv validate "$$f"; exit 1; }; \
	done
	@echo "  All contracts valid"
	@echo "=== Contract lint passed ==="

# ─── Format ──────────────────────────────────────────────────────────────────────

fmt: ## Format all Rust code
	cargo fmt --all

fmt-check: ## Check formatting (CI mode)
	cargo fmt --all -- --check

# ─── Coverage ────────────────────────────────────────────────────────────────────

coverage: ## Run tests with coverage (lib 98% line + 100% function gate)
	@echo "=== Coverage gate ==="
	cargo llvm-cov --workspace --lib --fail-under-functions 100 --fail-under-lines 98
	@echo "=== Coverage gate passed (100% functions, 98%+ lines) ==="

coverage-report: ## Generate HTML coverage report
	cargo llvm-cov --workspace --html
	@echo "Open target/llvm-cov/html/index.html"

# ─── Quality gates ───────────────────────────────────────────────────────────────

check: fmt-check lint test coverage ## Full quality gate (format + lint + test + coverage)
	@echo "=== All quality gates passed ==="

comply: ## Run pmat compliance check
	pmat comply

score: ## Run pmat unified score
	pmat score

# ─── Bench ───────────────────────────────────────────────────────────────────────

bench: ## Benchmark (no-op)
	@echo "No benchmarks configured"

# ─── Clean ───────────────────────────────────────────────────────────────────────

clean: ## Remove build artifacts
	cargo clean
