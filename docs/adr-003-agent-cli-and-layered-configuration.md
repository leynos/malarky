# Architectural decision record (ADR) 003: Stabilize the agent CLI and layered configuration

## Status

Accepted on 22 July 2026. Malarky exposes a deterministic agent-oriented CLI,
uses `ortho_config` for four configuration layers, and binds indexed retries to
the reported candidate set.

## Date

2026-07-22.

## Context and problem statement

Agents need compact commands, stable machine-actionable failures, and a clear
override model. An ambiguity index has meaning only for the document and
ordered candidates that produced it. If a later retry accepts an index against
changed content, the same number can select a different passage silently.

## Decision drivers

- Repeated commands over unchanged bytes must produce identical outcomes.
- Invocation settings must override persistent project policy predictably.
- Standard output must contain only successful command data.
- Ambiguous selection must never drift to another candidate.
- Configuration parsing and command validation need one bounded adapter.

## Options considered

- Build command and configuration layering independently.
- Accept candidate indexes whenever they remain in range.
- Use `ortho_config` and require a document-and-candidate-set fingerprint for
  indexed retries.

## Decision outcome

Malarky uses `ortho_config` to apply, from highest to lowest precedence:
command-line flags, `MALARKY_*` environment variables, an explicit or
discovered project configuration file, and built-in defaults. Command-line
flags and environment variables are invocation settings. Project configuration
is persistent local policy. Positional file paths and mutation payloads are not
persistent configuration.

Ambiguity output reports stable one-based indexes and a fingerprint derived
from the input document and complete ordered candidate set. `--match N`
requires that fingerprint and rejects a stale retry even when `N` remains in
range. Successful mutation commands persist the selected edit and then emit the
already generated localized unified diff. Diagnostics use standard error, and
documented exit statuses remain stable.

## Consequences

Agents can distinguish retries, configuration errors, ambiguity, and mutation
failures without rereading the document. Indexed retries require one additional
opaque value, but cannot silently select a shifted candidate. Configuration
provenance tests must cover every adjacent precedence layer.
