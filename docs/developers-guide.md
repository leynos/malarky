# Developer Guide

This guide explains the contributor workflow for the generated Malarky project.

## Local Workflow

Use `make all` as the public entrypoint for formatting, linting, and tests.
`make lint` runs rustdoc, Clippy, and Whitaker. `make test` prefers
`cargo nextest run` and falls back to `cargo test` when cargo-nextest is not
available. `make audit` derives the Rust workspace root with `cargo metadata`,
logs workspace member manifests, and runs `cargo audit` once from the workspace
root. `make coverage` uses `cargo llvm-cov` with `lld`.

## Malarky test ownership

`proptest` 1.10.0 and `trybuild` are development dependencies. Pure `document`,
`matching`, and `mutation` modules own focused unit and `proptest` invariant
tests. They validate source-map identity, UTF-8 boundaries, interval relations,
and mutation-plan ordering without command parsing or filesystem I/O. Command
modules own behavioural command-contract tests. Filesystem adapters own
capability and injected I/O-failure tests. The
[technical design](malarky-design.md#12-verification) defines the required
properties and combination coverage.

`tests/domain_design_contracts.rs` owns the top-level executable domain
contract. `tests/domain_design_contracts/boundaries.rs` owns focused accepted
and rejected boundary cases and generated range invariants.
`tests/domain_design_ui.rs` is the `trybuild` harness; its
`tests/ui/domain_contract/{pass,fail}/` fixtures verify public API visibility
and construction boundaries. UTF-8, range, containment, payload, and edit
ordering remain runtime unit and property-test coverage, rather than `trybuild`
cases. `docs/design/malarky-domain.rs` remains the normative domain artefact;
tests import it to verify the documented contract without adding test-only
behaviour to it.

### Standard and focused test commands

Run the complete test surface through the established Make target:

```sh
make test
```

During development, run the focused domain contract target or its boundary
module before the full suite:

```sh
cargo test --test domain_design_contracts
cargo test --test domain_design_contracts boundaries::
```

Run the implemented UI harness independently:

```sh
cargo test --test domain_design_ui
```

These commands validate the generated project today; the Malarky command-line
tool remains in the design stage until implementation lands.

GitHub Actions Act validation lives in `.github/workflows/act-validation.yml`.
The main `.github/workflows/ci.yml` workflow deliberately does not run
`make test WITH_ACT=1`; the separate Act workflow runs those slower
container-backed checks in parallel.

## Tooling

Development builds use the standard LLVM backend for debug code generation. On
Linux targets, `.cargo/config.toml` configures clang to link with `mold` so
debug builds link quickly. Coverage generation uses `lld` because LLVM coverage
tooling expects LLVM-compatible linker behaviour.

For a faster local edit-compile-test loop, `make dev-build` and `make dev-test`
apply the opt-in Cranelift backend and mold linker configured in
`tools/dev-fast/config.toml`. This fragment is passed explicitly with
`cargo --config`, so it never touches release, coverage, or verification
builds, and it requires a nightly toolchain. The pinned toolchain retains the
`rustc-codegen-cranelift-preview` and `llvm-tools-preview` components that this
fragment and `make coverage` depend on; `tools/dev-fast/config.toml` is what
controls whether Cranelift is actually activated for a given build, not the
toolchain pin itself.

Install `clang`, `lld`, `mold`, `python3`, and `cargo-audit` before running the
full generated workflow locally on Linux.

## Lint baseline

`Cargo.toml`'s `[lints.clippy]`, `[lints.rust]`, and `[lints.rustdoc]` tables
carry this project's lint policy directly, since Malarky is a single crate with
no `[workspace]` table to inherit from. They follow the Rust estate's phase 2
baseline, so treat `Cargo.toml` as the authoritative list rather than
duplicating it here; the one addition on top of that baseline is `pedantic`,
kept at `warn` rather than `deny`.

Where a lint fires on a genuine deferral rather than a real defect, add
`#[expect(clippy::<lint>, reason = "…")]` at the site, never `allow`. An
`#[expect]` warns the moment the site stops triggering the lint, so fixed sites
surface on their own instead of leaving a silent, stale suppression behind.

`clippy.toml` complements the lint tables: it sets the CodeScene-aligned
complexity thresholds and lists the raw `std::env` accessors (`var`, `var_os`,
`vars`, `vars_os`, `set_var`, `remove_var`) as `disallowed-methods`, each with
a reason pointing at injecting an environment reader instead of reading the
process environment directly.

The pinned nightly toolchain in `rust-toolchain.toml` supplies the `rustfmt`,
`clippy`, and `rust-analyzer` components these checks and IDE tooling depend on.

### Security audit ignores

Security audit jobs may set `CARGO_AUDIT_IGNORES` for narrowly scoped RustSec
advisories that affect unused or tooling-only dependency paths. Keep each
ignore tied to a documented runtime impact analysis, and remove it when the
affected dependency leaves the graph or the project starts using the advised
runtime path.
