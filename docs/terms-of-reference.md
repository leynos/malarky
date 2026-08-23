# Malarky — terms of reference

**Status:** Accepted (v0.1)

**Audience:** Product owner, engineering contributors, and agent-tooling
integrators.

**Companion documents:**

- [Domain context](context.md)
- [Technical design](malarky-design.md)
- [Development roadmap](roadmap.md)
- [ADR 001](adr-001-source-preserving-semantic-model.md)
- [ADR 002](adr-002-atomic-mutation-and-file-replacement.md)
- [ADR 003](adr-003-agent-cli-and-layered-configuration.md)

The three ADRs govern the accepted architecture.

**Last substantive revision:** 20 August 2026

## 1. Background and motivation

Editing agents need a token-efficient way to add review annotations to Markdown
without reproducing or rewrapping the surrounding document.

Autonomous and semi-autonomous agents increasingly perform line editing and
criticism, but general-purpose editing interfaces make them restate source
material that already exists in the document.

## 2. Domain

Malarky operates in agent-assisted editing of Markdown documents using the
established CriticMarkup notation. Its initial annotation vocabulary covers
deletion, insertion, highlighting, comments, and replacement.

The source document should retain its existing layout apart from the inserted
annotation.

Successful mutation commands edit the source file in place. The invoking agent
is expected to use version control to inspect or reverse the change.

The matcher presents Markdown spans and blocks as searchable text, but hides
link, reference, and image destinations, front matter, CriticMarkup delimiters,
and comments. As defined by the [domain glossary](context.md), semantic
matching always uses searchable net-result Markdown and preserves hidden
metadata boundaries.

Matching operates on a semantic rendering of each Markdown block, not on the
raw source codepoint stream. The semantic view retains a mapping to the
original source span, so an edit does not rewrite unrelated Markdown syntax or
formatting.

Existing CriticMarkup is exposed to matching through the net result of its
transformation. CriticMarkup comments are invisible to ordinary matching.
Reapplying a comment operation to the same matched text updates that comment.

Agents can inspect existing annotations through a `list` verb. Comments and
highlights can be removed by passing `--delete` to their respective verbs.
Deletions, insertions, and substitutions are undone by applying their inverse
operation rather than through a generic removal verb.

The inverse pairs are exact: `insert` over a deletion restores its deleted text,
`delete` over an insertion removes its inserted text, and `replace new old`
over `{~~old~>new~~}` restores `old`. Each inverse removes the original
annotation and restores plain text; it does not create a nested
counter-annotation.

The net-result view omits deleted text and comments, includes inserted text,
uses the new text from a replacement, and exposes the enclosed text of a
highlight.

Every emitted CriticMarkup annotation must remain within one Markdown block,
such as one paragraph or list item. An operation whose target crosses a block
boundary is rejected.

An operation may encompass an existing annotated span, but an operation that
intersects an annotated span without fully encompassing it is rejected with a
clear explanation.

## 3. Market context

CriticMarkup and the CriticMarkup toolkit establish the annotation format and
provide parsing and rendering tools. Manual document rewriting and generic
text-editing tools are current alternatives.

Literal search-and-replace tools are insufficiently forgiving, while general
patch generation consumes unnecessary tokens and can disturb layout.

Agentic editing tools such as Pi, OpenCode, and Codex provide adjacent prior
art. They show that agents benefit from unique context-based matches, explicit
ambiguity failures, source preservation, and compact diff feedback. The
[technical design](malarky-design.md) evaluates their implementation choices.

## 4. Users and stakeholders

Table 1: Stakeholder groups and their relationship to Malarky.

| Group                       | Relationship to Malarky                                         |
| --------------------------- | --------------------------------------------------------------- |
| Line-editor agents          | Primary users                                                   |
| Critic agents               | Primary users                                                   |
| Human authors and reviewers | Secondary users of the annotated artefact                       |
| Human CLI operators         | Non-users; direct use is incidental rather than a design target |
| Project maintainer          | Scope, release, and maintenance stakeholder                     |

Primary agents invoke Malarky as a tool. The command shape therefore favours
compact, stable, machine-actionable interaction over human-oriented
interactivity.

## 5. Job to be done

When an editing or critic agent identifies a localized change in a Markdown
document, it wants to describe the target and annotation concisely, so it can
preserve the document's formatting while producing reviewable CriticMarkup.

When matching yields exactly one candidate above the configured threshold, the
agent expects Malarky to apply it without an additional confirmation round
trip. Multiple qualifying candidates require an explicit selection.

## 6. Scope

### 6.1 Goals

- Add deletion, insertion, highlighting, comment, and replacement
  annotations.
- Preserve unaffected document text and wrapping.
- Detect ambiguous matches and present indexed choices.
- Allow explicit selection of one match or all matches.
- Support exact semantic-text matching followed by whitespace-normalized
  matching. Keep later normalization tiers explicit, configurable, and
  independently testable.
- Apply a sole qualifying match automatically.
- Edit the selected Markdown file in place after a successful mutation command.
- Match the visible textual result of existing CriticMarkup rather than
  its annotation delimiters or comments.
- Reject cross-block annotations and unsafe partial intersections with
  annotated spans.
- List existing CriticMarkup annotations.
- Remove comments and highlights explicitly with `--delete`.
- Undo deletion, insertion, and substitution annotations through their
  inverse operations.
- Provide a non-mutating validation command that reports malformed,
  cross-block, or partially intersecting CriticMarkup. This is lower priority
  than the core annotation and matching workflow.
- Produce stable, compact diagnostics suitable for agents.
- Emit a localized diff after every successful edit, so the agent can
  verify the result without rereading the document.

### 6.2 Non-goals

- Rendering CriticMarkup to HTML or accepting and rejecting all
  changes in bulk.
- Acting as an interactive full-screen editor.
- Reformatting or rewrapping Markdown.
- Providing general-purpose patching or arbitrary Markdown
  transformation.
- Matching or modifying link destinations and reference definitions.

## 7. Success criteria

- A target that matches exact or whitespace-normalized semantic text can be
  annotated without reproducing the surrounding document.
- Unaffected bytes remain unchanged.
- Ambiguous operations never silently choose a match.
- A sole candidate above the configured threshold is applied without
  confirmation.
- Invalid cross-block and partial-intersection operations fail without
  changing the file and explain the violated boundary.
- Normalized matching must not normalize unrelated source text. Only
  the requested CriticMarkup delimiters and payload may change the original
  Markdown source.
- Diagnostics and exit statuses let an agent decide whether to retry,
  select a match, or correct its input.
- Successful feedback shows enough local context for the agent to
  verify the inserted CriticMarkup while remaining substantially smaller than
  the document.

## 8. Constraints and assumptions

### 8.1 Hard constraints

- Malarky is a command-line tool implemented in Rust.
- The project intends to use the Rust `markdown` crate.
- Emitted annotations must conform to CriticMarkup syntax.
- CriticMarkup must be contained within one Markdown block.
- A mutation must either avoid an existing annotated span or encompass
  it fully; partial intersection is invalid.

### 8.2 Assumptions

- Malarky's initial release assumes input files contain UTF-8 Markdown. Other
  encodings require an explicit future compatibility decision.
- Initial usage targets individual chapters and papers rather than
  book-length manuscripts. If agents apply Malarky to substantially larger
  files, acceptable matching latency and memory use are not yet established.
- Matching uses a conservative built-in policy. From highest to lowest
  precedence, `ortho_config` applies command-line flags, `MALARKY_*`
  environment variables, an explicit `--config-path` or discovered
  `.malarky.toml` project file, and built-in defaults. Command-line flags and
  environment variables are invocation settings; project configuration is
  persistent local policy.
- The localized diff for a successful edit uses standard output.
  Diagnostics and errors use standard error, so they cannot corrupt the success
  payload.
- The invoking agent has placed the document under version control.
  If this assumption fails, Malarky provides no established recovery mechanism
  for an unwanted in-place edit.

### 8.3 Dependencies

- The CriticMarkup syntax is the external annotation contract.
- The Rust `markdown` crate is the intended Markdown parser.
- `ortho_config` is the selected command-parsing and layered-configuration
  dependency.

## 9. Open questions

- What quantitative accuracy, performance, and token-efficiency thresholds
  define success?
- What exact localized-diff format and context size should form the stable
  success-output contract?
- Which additional power-loss durability and network-filesystem guarantees are
  valuable beyond the accepted command-level replacement contract?

## References

- CriticMarkup syntax, MultiMarkdown 6,
  <https://fletcher.github.io/MultiMarkdown-6/syntax/critic.html>, accessed
  9 July 2026.
- CriticMarkup toolkit,
  <https://github.com/CriticMarkup/CriticMarkup-toolkit>, accessed 9 July 2026.
- `markdown` crate documentation,
  <https://docs.rs/markdown/latest/markdown/>, accessed 9 July 2026.
- OrthoConfig repository, <https://github.com/leynos/ortho-config>, accessed
  10 July 2026.

## Glossary pointer

[Domain context](context.md) defines CriticMarkup, semantic text, match
context, normalized match, and ambiguous match.

## Design handoff candidates

- Record the selected use of `leynos/ortho-config` for layered configuration in
  [ADR 003](adr-003-agent-cli-and-layered-configuration.md) and the
  [technical design](malarky-design.md). Its documented precedence and
  command-line/configuration-file support fit Malarky's override model.
- Represent semantic text as source-mapped block spans. Prior art demonstrates
  that normalized matching without a safe mapping back to original source can
  alter unrelated formatting.
- Evaluate ordered matcher tiers rather than one global fuzzy algorithm. Each
  tier should expose its normalization contract, preserve ambiguity, and be
  independently testable.
