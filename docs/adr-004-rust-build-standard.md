# Architectural decision record (ADR) 004: Adopt the Rust build standard for development builds

## Status

Accepted on 4 October 2026. Malarky's development builds use the parallel
`rustc` frontend and, on Linux, the `mold` linker, while coverage and release
builds stay on the defaults and Cranelift stays outside the auto-discovered
Cargo configuration.

## Date

2026-10-04.

## Context and problem statement

Edit-compile cycles on a large workspace are dominated by frontend time and by
link time. The estate's build standard (`rust-build-defaults`) addresses both
with `-Zthreads=8` and the `mold` linker on Linux. Cargo applies exactly one
`rustflags` source, and an assigned `RUSTFLAGS` replaces every configuration
source, so a recipe or workflow step that assigns `RUSTFLAGS` silently loses
the flags unless it restates them. Measurements and shipped artefacts also need
a stable baseline that the fast flags would disturb.

## Decision drivers

- Faster local and CI development builds without changing what the code means.
- Coverage numbers that do not depend on the frontend flag or the linker.
- A release that ships from the platform linker on the stable toolchain.
- A flag lost through a recipe or workflow edit must fail a test, not pass
  quietly.

## Options considered

- Configure the flags only in `.cargo/config.toml`. This leaves every recipe
  and CI step that assigns `RUSTFLAGS` without them.
- Restate the flags in each recipe and step, held by a contract test. This is
  the option taken.
- Select Cranelift in `.cargo/config.toml`. Stable Cargo, which the release
  workflow uses through `cross +stable`, refuses a `codegen-backend` key there,
  so the backend would break every release.

## Decision outcome

Development, test, lint and typecheck builds use `-Zthreads=8` and, on Linux,
`mold`, from `.cargo/config.toml` and from the Makefile and workflow steps that
assign `RUSTFLAGS`. Coverage assigns its own flags and ignores the caller's.
The release recipe assigns `RUSTFLAGS="${RUSTFLAGS-}"`, which keeps the
caller's value and adds nothing. Cranelift is selected by the opt-in
`dev-build` and `dev-test` targets through `tools/dev-fast/config.toml`, never
by `.cargo/config.toml`.

`tests/build_standard_contract.rs` reads the configuration sources and the
commands `make -n` prints for each target, and a further contract holds the CI
doctest step's own `RUSTFLAGS`.

## Consequences

- Linux builds require `mold`, and its flag reaches the linker through `cc`,
  which must be GCC 12.1 or newer, or clang.
- A change to a recipe or workflow step that assigns `RUSTFLAGS` must restate
  the flags, and the contracts say which one fails when it does not.
- Revisit Cranelift if the release moves to the pinned nightly.
