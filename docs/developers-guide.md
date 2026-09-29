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

The test suite runs once per pull request, in `ci.yml`'s coverage step. That
step runs the same tests `make test` runs except the doctests, which
`build-test` runs in a step of its own with
`cargo test --doc --workspace --all-features`. The repository used to carry an
`act-validation.yml` workflow that ran `make test WITH_ACT=1`, but nothing reads
`WITH_ACT` and no test is gated on Act, so that workflow ran the whole suite a
second time and was removed. The crate declares no features, so `make test`'s
`--all-features` selects the same tests as the coverage run's default.
`tests/workflow_suite_contract.rs` holds the split.

## Tooling

Development builds use the build standard described below: `-Zthreads=8` and,
on Linux, clang linking with `mold`, from `.cargo/config.toml`. The standard
make targets also select the Cranelift backend by passing
`--config tools/dev-fast/config.toml`, as do `make dev-build` and
`make dev-test`; that fragment holds only the backend selection and is never
applied to release, coverage, or verification builds (see
[Fast development builds](../AGENTS.md#fast-development-builds) in
`AGENTS.md`). Coverage generation uses `lld` and LLVM because coverage tooling
expects them. The pinned nightly toolchain retains the `llvm-tools-preview` and
`rustc-codegen-cranelift-preview` components, so both paths have what they need
installed.

Install `clang`, `lld`, `mold`, `python3`, and `cargo-audit` before running the
full generated workflow locally on Linux.

## The build standard

Development, test, lint, and typecheck builds use the parallel `rustc` frontend
(`-Zthreads=8`) and, on Linux, the `mold` linker (`-Clink-arg=-fuse-ld=mold`).
These are defaults in `.cargo/config.toml`, which Cargo discovers on its own,
so a bare `cargo build` gets them. `mold` ships for Linux only, so the linker
flag lives in a Linux-only table and macOS and Windows keep their platform
linker. Cargo selects one `rustflags` source rather than merging them, so every
source repeats the same flags apart from the linker.

An assigned `RUSTFLAGS` replaces the configuration's flags, so the Makefile
recipes that set it compose the standard's flags onto any inherited value (CI's
`setup-rust` exports one). Two builds are deliberately excluded: coverage
assigns `RUSTFLAGS` without the fast flags, because a measurement should not
depend on them, and the release recipe and workflow keep the platform linker,
because they assign `RUSTFLAGS` (even an empty value displaces the
configuration). Cargo has no per-profile `rustflags`, so a direct
`cargo build --release` takes the configuration's flags unless `RUSTFLAGS` is
assigned too.

On Linux, install `mold` before building: the configuration names it, so a
build without it fails at link time. CI installs it through `setup-rust`'s
`install-mold` input. `tests/build_standard_contract.rs` holds the standard. It
reads the configuration sources, the commands `make -n` prints for each
development target on a Linux host and a macOS host (each keeping the caller's
own `RUSTFLAGS`) and for each coverage and release target on a Linux host, and
the `setup-rust` steps of the CI workflows (each must pass `install-mold`), so
a flag lost through a recipe or workflow edit fails there.

### Cranelift

Exception: Cranelift is not the default backend in `.cargo/config.toml`. The
repository pins `nightly-2026-05-28`, but the release workflow
(`cross +stable build --release`) builds on a stable toolchain against
`.cargo/config.toml`, and stable Cargo refuses a
`[profile.dev] codegen-backend` key ("config profile `dev` is not valid"), so a
backend selected there would break every release (recorded 2026-09-29). The
standard make targets select Cranelift instead, by passing
`--config tools/dev-fast/config.toml`, which holds only the backend selection;
coverage and release builds never pass it. A contract fails if a
`codegen-backend` key reaches `.cargo/config.toml` while the release builds on
`+stable`. `tests/stable_cargo_config.rs` also asks stable Cargo itself, through
`rustup run stable cargo build --release --bin no-such-bin`: stable Cargo
resolves every configured profile before it looks up the target, so a refused
configuration and an accepted one differ in the message, and nothing compiles.
The probe must run on stable, because a nightly Cargo accepts a backend that
stable refuses. The test therefore needs the stable toolchain installed
(`rustup toolchain install stable --profile minimal`); CI installs it before
the tests run. Revisit if the release moves to the pinned nightly.

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
