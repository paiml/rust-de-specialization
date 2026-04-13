# Contributing

This repository contains the **Rust for Data Engineering: 7-Course Specialization** on Coursera.

## Repository Structure

- `capstones/` — One capstone project per course (`c01-capstone.md` through `c07-capstone.md`)
- `coursera-assets/` — Marketing assets (key-terms, reflections, images)
- `coursera-output/` — Generated output from slash commands
- `assets/` — SVG hero images and visual assets

## Capstone Requirements

Every capstone file must contain these three sections:

- `## Deliverables` — What the learner produces
- `## Evaluation Criteria` — How the work is graded
- `## Share Your Work` — LinkedIn sharing prompt for portfolio building

## Development Workflow

### Run all checks before committing

```bash
make check
```

This runs both `make lint` (tab detection, capstone link verification) and `make test` (structural validation of all 7 capstones).

### Pre-commit hooks

Install pre-commit hooks to run validation automatically:

```bash
pre-commit install
```

Or use the git hook directly:

```bash
git config core.hooksPath .githooks
```

### Available targets

```bash
make help      # Show all targets
make lint      # Lint markdown, verify capstone links
make test      # Validate 7 capstones + README
make coverage  # Structural validation (runs test)
make check     # lint + test
```

## Adding or Editing a Capstone

1. Edit the file in `capstones/cNN-capstone.md`
2. Ensure all three required sections are present
3. Run `make check` to validate
4. Commit and push
