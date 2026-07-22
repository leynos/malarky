# Documentation contents

[Documentation contents](contents.md) is the index for Malarky's documentation
set.

## Project guides

- [Terms of reference](terms-of-reference.md) defines Malarky's users, problem,
  scope, constraints, and success criteria.
- [Domain context](context.md) defines the vocabulary shared by product and
  design documents.
- [Malarky technical design](malarky-design.md) specifies the architecture,
  command contracts, mutation model, failure behaviour, and verification
  properties.
- [Development roadmap](roadmap.md) sequences Malarky's implementation as
  GIST-aligned, hypothesis-driven vertical slices.
- [User guide](users-guide.md) explains how to use the generated project and
  its public build and test commands.
- [Developer guide](developers-guide.md) explains the local workflow and
  implementation tooling for contributors.
- [Repository layout](repository-layout.md) explains the generated project's
  top-level files, directories, and ownership boundaries.
- [Documentation style guide](documentation-style-guide.md) defines the
  spelling, structure, Markdown, Architecture Decision Record (ADR), Request
  for Comments (RFC), and roadmap conventions used by this documentation set.

## Normative design artefacts

- [CLI contract](design/malarky-cli.txt) gives agent integrators the stable
  verbs, selection controls, output, and exit statuses.
- [Configuration example](design/malarky-config.toml) gives operators and
  implementers the supported project keys and built-in values.
- [Domain types](design/malarky-domain.rs) gives implementers the validated
  source-map, annotation, and mutation-plan shapes.
- [Component flow](design/malarky-components.mmd) gives architecture reviewers
  the ordered parsing, mutation, diff, and replacement pipeline.
- [Mutation matrix](design/mutation-matrix.md) gives implementers and test
  authors the operation, inverse, and intersection rules.

## Architecture decision records

- [ADR 001](adr-001-source-preserving-semantic-model.md) explains to
  implementers why semantic matching uses composed source maps.
- [ADR 002](adr-002-atomic-mutation-and-file-replacement.md) defines the
  transaction and concurrency guarantees for implementers.
- [ADR 003](adr-003-agent-cli-and-layered-configuration.md) defines the
  invocation, selection, and override contracts for integrators.

## Rust reference material

- [Reliable testing in Rust via dependency injection](reliable-testing-in-rust-via-dependency-injection.md)
  explains how to keep tests deterministic by injecting environment, clock,
  filesystem, and other external dependencies.
- [Rust doctest Don't Repeat Yourself guide](rust-doctest-dry-guide.md)
  explains how to write maintainable, executable Rust documentation examples.
- [Rust testing with `rstest` fixtures](rust-testing-with-rstest-fixtures.md)
  explains fixture-based, parameterized, and asynchronous testing with `rstest`.

## Engineering practice

- [Complexity antipatterns and refactoring strategies](complexity-antipatterns-and-refactoring-strategies.md)
  explains cognitive complexity, the bumpy-road antipattern, and refactoring
  approaches for maintainable code.
- [Scripting standards](scripting-standards.md) explains the preferred Python
  scripting stack, command execution patterns, and test expectations for helper
  scripts.
