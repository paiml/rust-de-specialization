SHELL := /bin/bash

.PHONY: help lint test test-fast coverage fmt fmt-check check bench

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

lint: ## Lint markdown files
	@echo "=== Markdown lint ==="
	@find . -name '*.md' -not -path './.git/*' -not -path './node_modules/*' | \
		xargs -I{} sh -c 'grep -Pn "\t" "$$1" && echo "FAIL: tabs in $$1" && exit 1 || true' _ {}
	@echo "=== Checking capstone links ==="
	@for i in $$(seq -w 1 7); do \
		f="capstones/c0$$i-capstone.md"; \
		[ -f "$$f" ] || { echo "MISSING: $$f"; exit 1; }; \
	done
	@echo "=== Lint passed ==="

test: ## Validate all capstones and course structure
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
	@grep -q "7-Course" README.md || { echo "FAIL: README does not reference 7 courses"; exit 1; }
	@echo "  README references 7 courses"
	@echo "=== All tests passed ==="

test-fast: test ## Fast test (same as test for this repo)

coverage: test ## Coverage (structural validation)
	@echo "=== Coverage: 7/7 capstones validated (100%) ==="

fmt: ## Format (no-op for markdown repo)
	@echo "No formatter configured for markdown-only repo"

fmt-check: ## Check formatting
	@echo "No formatter configured for markdown-only repo"

check: lint test ## Run all checks
	@echo "=== All checks passed ==="

bench: ## Benchmark (no-op)
	@echo "No benchmarks for this repo"
