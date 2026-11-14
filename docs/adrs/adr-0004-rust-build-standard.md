# Architectural decision record (ADR) 004: Adopt the Rust build standard for development builds

## Status

Accepted on 4 October 2026. Malarky's development builds use the parallel
`rustc` frontend and, on Linux, the `mold` linker, while coverage and release
builds stay on the defaults. Amended 13 November 2026: the release workflow
moved to the pinned nightly, so Cranelift became the development default in
`.cargo/config.toml`. Amended 14 November 2026: the Cranelift selection is
commented out and LLVM is the estate standard again, with review due
14 May 2027; the component and the opt-in stay.

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
- Select Cranelift in `.cargo/config.toml`. Originally rejected, because stable
  Cargo, which the release workflow then used through `cross +stable`, refuses a
  `codegen-backend` key there. Taken on 13 November 2026, once the release
  moved to the pinned nightly.

## Decision outcome

Development, test, lint and typecheck builds use `-Zthreads=8` and, on Linux,
`mold`, from `.cargo/config.toml` and from the Makefile and workflow steps that
assign `RUSTFLAGS`. Coverage assigns its own flags and ignores the caller's.
The release recipe assigns `RUSTFLAGS="${RUSTFLAGS-}"`, which keeps the
caller's value and adds nothing. Development builds use LLVM; the Cranelift
selection in `.cargo/config.toml` is commented out (it was the default from 13
to 14 November 2026). Coverage selects LLVM explicitly, and the release builds
on the pinned nightly with LLVM.

`tests/build_standard_contract.rs` reads the configuration sources and the
commands `make -n` prints for each target, and a further contract holds the CI
doctest step's own `RUSTFLAGS`.

## Consequences

- Linux builds require `mold`, and its flag reaches the linker through `cc`,
  which must be GCC 12.1 or newer, or clang. For `x86_64-unknown-linux-gnu`,
  `.cargo/config.toml` selects `clang` explicitly, so `clang` is required there.
- A change to a recipe or workflow step that assigns `RUSTFLAGS` must restate
  the flags, and the contracts say which one fails when it does not.
- Every build, the release included, needs the pinned nightly: stable Cargo
  refuses the `codegen-backend` key in `.cargo/config.toml`.
