# Malarky domain context

This glossary defines the vocabulary used by Malarky's terms of reference and
technical design.

## Terms

**Ambiguous match:** Two or more candidate source spans that satisfy the active
matching policy. Malarky requires the agent to select one candidate or request
all candidates; it does not choose silently.

**CriticMarkup:** A plain-text annotation syntax for additions, deletions,
substitutions, highlights, and comments embedded in Markdown.

**Match context:** Text supplied before or after a requested mutation to
disambiguate the target span.

**Net-result view:** The text represented by existing CriticMarkup after
omitting deletions and comments, retaining additions, selecting replacement
text, and retaining highlighted text.

**Normalized match:** A match found after applying an explicitly defined
normalization to both the requested text and semantic text. The initial policy
normalizes whitespace only.

**Semantic text:** Searchable net-result Markdown derived from a Markdown
block. It includes visible text represented by spans and blocks but excludes
link, reference, and image destinations, front matter, CriticMarkup delimiters,
and comments. Matching always operates on this net-result view, so hidden
metadata boundaries remain unavailable as candidates. This contract is governed
by [ADR 001](adr-001-source-preserving-semantic-model.md).

**Source span:** A half-open byte range in the original Markdown source that
corresponds to semantic text or an existing annotation.
