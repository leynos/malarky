# Architectural decision record (ADR) 001: Preserve source through mapped semantic text

## Status

Accepted on 22 July 2026. Malarky uses separate, source-mapped CriticMarkup and
Markdown representations and permits mutation only through validated UTF-8
source boundaries.

## Date

2026-07-22.

## Context and problem statement

Agents identify visible prose, while Malarky must edit the original Markdown
without rewriting syntax, wrapping, or hidden metadata. Raw-source matching
exposes link destinations and annotation delimiters. Rendered-text matching
loses the byte positions required for source-preserving edits.

CriticMarkup also changes the visible result before Markdown is parsed.
Replacement annotations expose their new payload, while deletions and comments
contribute no searchable text. A single parser cannot represent both syntaxes
and preserve their distinct boundaries reliably.

## Decision drivers

- Unaffected source bytes must remain identical.
- Search must use visible net-result Markdown and hide structural metadata.
- Every selected semantic boundary must map to a UTF-8 source boundary.
- Parser-library offsets must not enter the domain core unchecked.
- Normalization must affect candidate qualification, not replacement bytes.

## Options considered

- Search and mutate raw Markdown source.
- Render Markdown and recover source locations heuristically.
- Compose source maps from a CriticMarkup overlay and a Markdown syntax tree.

## Decision outcome

Malarky first parses CriticMarkup as a source-mapped overlay and projects its
net-result view. It then parses Markdown and constructs semantic blocks from
visible text. Link, reference, and image destinations, front matter,
CriticMarkup delimiters, and comments remain outside searchable semantic text.

Every semantic-text boundary maps monotonically to an original UTF-8 byte
boundary. Cross-segment joins are explicit, and a candidate is valid only when
its complete normalized range maps to one contiguous source interval. Domain
types accept no unchecked parser offsets. The pure document, matching, and
mutation modules consume typed maps; parser adapters own external
representations and offset conversion.

## Consequences

Source preservation and mapping safety can be tested independently of parser
libraries. Multibyte text, projection joins, Markdown syntax, and whitespace
normalization require property coverage. The composed representation is more
complex than raw search, but failures remain explicit instead of risking an
incorrect edit.
