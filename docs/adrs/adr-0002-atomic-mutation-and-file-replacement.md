# Architectural decision record (ADR) 002: Make mutation and replacement failure-atomic

## Status

Accepted on 22 July 2026. Malarky validates and renders a complete prospective
mutation, generates its diff, and conditionally replaces the target as one
failure-atomic command.

## Date

2026-07-22.

## Context and problem statement

A source-preserving plan is insufficient if a later failure commits only part
of the command. Diff generation can fail, multiple edits can overlap, and
another process can modify a target after it is read. Checking filesystem
identity before rename does not detect an in-place write that retains the same
identity. A check separated from replacement also leaves a race in which newer
content can be overwritten.

## Decision drivers

- Every failure must leave the selected target unchanged.
- Bulk edits must be all-or-nothing and preserve unaffected bytes.
- A successful diff must describe the bytes that were committed.
- Intervening writes must never be overwritten silently.
- Replacement must remain capability-scoped and avoid following symlinks.

## Options considered

- Check target identity and then perform an ordinary same-directory rename.
- Lock the target and verify content while holding the lock through rename.
- Use a platform atomic compare-and-replace primitive with the read fingerprint.

## Decision outcome

Mutation plans contain non-overlapping source edits in descending start-offset
order. Planning applies all edits in memory, reparses and validates the
complete prospective document, and generates a buffered localized diff before
any file replacement begins.

The filesystem adapter uses either an exclusive lock held from final
content-fingerprint verification through same-directory replacement, or a
platform atomic compare-and-replace operation that couples the same
verification to replacement. A mismatched fingerprint is a concurrent
modification error. Identity checks alone are not sufficient. The temporary
file is synchronized before replacement, and the buffered diff is emitted only
after replacement succeeds.

Replacement inverses remove the original annotation. In particular, reversing
`{~~OLD~>NEW~~}` selects the exact new-payload span, restores the old bytes,
and does not create a nested counter-annotation.

## Consequences

Diff failures and concurrent writes cannot follow or be overwritten by a
committed edit. Implementations need injectable lock, fingerprint, write,
synchronization, and replacement failures. The guarantee is command-level
atomicity, not power-loss durability; directory synchronization remains a
separate durability decision.
