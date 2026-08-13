# User Guide

This guide explains how to use the generated Malarky project after rendering it
from the template.

## Generated Tooling

Generated projects use Rust 2024, a pinned nightly toolchain, strict lint
settings, and documented starter code. Library projects render `src/lib.rs`.
Application projects render `src/main.rs`, `src/lib.rs`, release automation, and
`[package.metadata.binstall]` metadata for binary installation.

Development builds use the standard LLVM backend for debug code
generation. On Linux targets, `.cargo/config.toml` configures clang to
link with `mold` so local debug builds link quickly. Coverage generation
uses `lld` instead because LLVM coverage tools expect LLVM-compatible
linker behaviour.

For a faster local edit-compile-test loop, `make dev-build` and
`make dev-test` apply the opt-in Cranelift backend and mold linker
configured in `tools/dev-fast/config.toml`. This fragment is passed
explicitly with `cargo --config`, so it never applies to release,
coverage, or verification builds, and it requires a nightly toolchain.

## Makefile Targets

The generated `Makefile` exposes these public targets:

- `make all` runs formatting checks, linting, and tests.
- `make check-fmt` verifies Rust formatting.
- `make lint` runs rustdoc, Clippy, and Whitaker with warnings denied.
- `make test` runs `cargo nextest run` when cargo-nextest is installed and
  falls back to `cargo test` otherwise. All projects also run doctests.
- `make build` builds the debug target.
- `make dev-build` builds the debug target with the opt-in Cranelift
  backend and mold linker (requires a nightly toolchain).
- `make dev-test` runs tests with the opt-in Cranelift backend and mold
  linker (requires a nightly toolchain).
- `make release` builds the release target.
- `make coverage` writes `lcov.info` using `cargo llvm-cov` and `lld`.
- `make audit` derives the Rust workspace root with `cargo metadata` and runs
  `cargo audit` once from that root.
- `make markdownlint` checks Markdown files.
- `make nixie` validates Mermaid diagrams.

Install `clang`, `lld`, `mold`, `python3`, and `cargo-audit` before running the
full generated workflow locally on Linux.
