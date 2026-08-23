# User Guide

This guide explains how to use the generated Malarky project after rendering it
from the template.

## Malarky command status

Malarky is currently in the design stage. The
[normative CLI contract](design/malarky-cli.txt) describes the planned agent
interface, but no Malarky command is runnable yet. Consult the
[technical design](malarky-design.md) and [roadmap](roadmap.md) for the planned
behaviour and delivery sequence.

## Planned command interface

The following syntax is planned documentation, not runnable command examples.
The [normative CLI contract](design/malarky-cli.txt) takes precedence over this
guide if the documents differ.

```plaintext
malarky delete [OPTIONS] <FILE> <BEFORE> <TEXT> <AFTER>
malarky insert [OPTIONS] <FILE> <BEFORE> <TEXT> <AFTER>
malarky replace [OPTIONS] <FILE> <BEFORE> <OLD> <NEW> <AFTER>
malarky highlight [OPTIONS] [--delete] <FILE> <TEXT>
malarky comment [OPTIONS] [--delete] <FILE> <TEXT> [COMMENT]
malarky list [--config-path <PATH>] <FILE>
malarky validate [--config-path <PATH>] <FILE>
```

The planned mutation commands select visible semantic text, preserve unrelated
source bytes, and emit a localized unified diff only after a successful
replacement. `list` and `validate` are planned inspection commands that do not
modify a file. The [technical design](malarky-design.md) defines semantic text,
source mapping, replacement safety, diagnostics, and exit statuses.

Planned mutation is command-atomic: either the selected file is replaced and
the planned diff is written, or the file remains unchanged. Planned successful
data goes to standard output: mutations emit the localized diff, `list` emits
annotation records, and `validate` emits its valid-file record. Diagnostics go
to standard error. The planned failure statuses distinguish invalid invocation
or configuration (2), no match (3), ambiguity (4), invalid mutation (5),
pre-mutation or inspection parsing failure (6), replacement failure (7), and
`validate` document failure (8). These output and status rules are not yet
available from a runnable command.

### Planned selection and configuration

Planned mutation options include `--match <INDEX>`, `--selection <TOKEN>`,
`--all`, `--matching <POLICY>`, and `--config-path <PATH>`. An ambiguity report
will provide the selection token required with `--match`; it prevents a retry
from selecting a changed candidate set. Configuration will apply command-line
flags, `MALARKY_*` environment variables, the selected or discovered project
file, and built-in defaults in that order. These behaviours remain design
contracts until an implementation is released.

## Generated Tooling

Generated projects use Rust 2024, a pinned nightly toolchain, strict lint
settings, and documented starter code. Library projects render `src/lib.rs`.
Application projects render `src/main.rs`, `src/lib.rs`, release automation, and
`[package.metadata.binstall]` metadata for binary installation.

Coverage generation uses `lld` because LLVM coverage tools expect
LLVM-compatible linker behaviour. See the developer guide for local build
tooling and linker configuration.

## Makefile Targets

The generated `Makefile` exposes these public targets:

- `make all` runs formatting checks, linting, and tests.
- `make check-fmt` verifies Rust formatting.
- `make lint` runs rustdoc, Clippy, and Whitaker with warnings denied.
- `make test` runs `cargo nextest run` when cargo-nextest is installed and
  falls back to `cargo test` otherwise. All projects also run doctests.
- `make build` builds the debug target.
- `make dev-build` builds the debug target using the opt-in accelerated
  toolchain described in the developer guide.
- `make dev-test` runs tests using the opt-in accelerated toolchain
  described in the developer guide.
- `make release` builds the release target.
- `make coverage` writes `lcov.info` using `cargo llvm-cov` and `lld`.
- `make audit` derives the Rust workspace root with `cargo metadata` and runs
  `cargo audit` once from that root.
- `make markdownlint` checks Markdown files.
- `make nixie` validates Mermaid diagrams.

Install `clang`, `lld`, `python3`, and `cargo-audit` before running the full
generated workflow locally on Linux. See the developer guide for additional
local build tooling.
