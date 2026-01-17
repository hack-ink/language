# Guide

Use this guide for workflows and validation steps.

## Setup and build

- `cargo build`.
- `cargo test`.
- Optional task wrappers: `cargo make fmt`, `cargo make clippy`, `cargo make nextest`.

## Code generation

- Run: `cargo run --features codegen --bin language`.
- Optional output path: `--out /path/to/output.rs`.
- Requires network access to download the dataset.

## Workflow

- Data refresh: run codegen, review `src/generated.rs`, run validation.
- API or behavior change: update implementation and update `docs/spec/index.md`.
- Interop change: update implementation, update `docs/spec/index.md`.

## Validation

- Default: `cargo test`.
- All features: `cargo test --all-features`.

## Rust style

- Read `docs/guide/rust_style_guide.md`.
