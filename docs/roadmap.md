# Malarky roadmap

This roadmap translates the accepted
[terms of reference](terms-of-reference.md), the
[technical design](malarky-design.md), and its normative artefacts under
`docs/design/` into an outcome-oriented delivery sequence. The governing
decisions are
[ADR 001](adr-001-source-preserving-semantic-model.md), [ADR 002](adr-002-atomic-mutation-and-file-replacement.md),
and [ADR 003](adr-003-agent-cli-and-layered-configuration.md). No Request for
Comments (RFC) documents currently govern the project.

The roadmap follows Goals, Ideas, Steps, and Tasks (GIST): each phase carries a
falsifiable idea, each step is a workstream that tests part of that idea, and
each task is a review-sized execution unit. It does not promise dates or
durations.

## 1. Foundational contracts that resist interface drift

Idea: if Malarky makes its command, domain, error, configuration, and module
contracts executable before feature work, each vertical slice can extend one
coherent pipeline instead of redefining agent-visible behaviour.

This phase leaves the generated project skeleton capable of compiling the
normative types and parsing the agreed command surface. It does not build an
otherwise unusable parser layer.

### 1.1. Turn the document contracts into compile-time interfaces

This step answers whether the design artefacts are precise enough to become
code without reinterpretation. Its outcome fixes the vocabulary and result
boundaries used by every later slice. See `malarky-design.md` §§3-4 and
`design/malarky-domain.rs`.

- [ ] 1.1.1. Introduce feature-oriented modules and the normative domain
  newtypes.
  - Define source spans, mapped semantic segments, candidates, annotations,
    source edits, and mutation plans without filesystem or terminal I/O.
  - Add parameterized unit tests for accepted and rejected constructor
    boundaries: complete, monotonic, in-range UTF-8 segment maps; ordered,
    in-range UTF-8 cross-segment joins; kind-correct, contained annotation
    payloads with replacement `OLD` before `NEW`; and descending, disjoint
    mutation plans.
  - Cover empty and single-edit plans, adjacent edits, valid empty insertions,
    and rejected overlapping, nested, equal-start, ascending, and
    equal-boundary empty-insertion plans. Add `proptest` 1.10 as a development
    dependency and implement property tests over Unicode, block source maps,
    joins, payload ranges, and edit ranges.
  - Keep `main` responsible only for configuration, dispatch, output, and exit
    status mapping.
  - See `malarky-design.md` §§3 and 4.1.
  - Success: the public and internal type boundaries compile with missing
    documentation and Clippy warnings denied, and their constructor tests prove
    mapping safety, containment, ordering, and disjointness.
- [ ] 1.1.2. Define typed command outcomes and the stable failure taxonomy.
  - Requires 1.1.1.
  - Represent success, no match, ambiguity, invalid mutation, parse failure,
    I/O failure, and validation failure without stringly typed dispatch.
  - See `malarky-design.md` §11 and `design/malarky-cli.txt`.
  - Success: every documented exit status maps from exactly one typed outcome,
    and no domain module chooses a process exit code.

### 1.2. Make command and configuration precedence executable

This step answers whether agents can invoke the documented surface with stable
defaults before commands gain behaviour. It settles precedence and rejects
invalid combinations early. See `malarky-design.md` §8,
`design/malarky-cli.txt`, and `design/malarky-config.toml`.

- [ ] 1.2.1. Add the pinned `markdown`, `ortho_config`, `similar`, `tempfile`,
  `cap-std`, and `camino` dependencies with bounded adapter modules.
  - Requires 1.1.1.
  - Record each dependency's ownership boundary in the developer guide.
  - See `malarky-design.md` §2.
  - Success: dependency APIs do not appear in the pure domain modules.
- [ ] 1.2.2. Parse every documented verb, positional argument, and option with
  `ortho_config`.
  - Requires 1.1.2 and 1.2.1.
  - Reject `--match` with `--all`, comment deletion with a comment payload,
    and mutation-only flags on inspection commands.
  - See `malarky-design.md` §8 and `design/malarky-cli.txt`.
  - Success: generated help matches the normative CLI contract and invalid
    combinations exit 2 without reading a document.
- [ ] 1.2.3. Implement layered matching and diff configuration.
  - Requires 1.2.2.
  - Compose built-in defaults, a discovered `.malarky.toml` or the project file
    selected by `--config-path`, `MALARKY_*` environment variables, and
    command-line overrides in documented order.
  - See `malarky-design.md` §8 and `design/malarky-config.toml`.
  - Success: provenance-aware scenarios demonstrate command line over
    environment, environment over either project file, either project file over
    defaults, and explicit `--config-path` selection instead of discovery.

### 1.3. Establish a source-fidelity corpus

This step answers which representative inputs can falsify later claims about
UTF-8 mapping and byte preservation. The corpus becomes shared evidence for
inspection and mutation slices. See `malarky-design.md` §§3, 5, and 12.

- [ ] 1.3.1. Add compact fixtures for Markdown blocks, spans, links, code,
  references, line endings, and multibyte text.
  - Requires 1.1.1.
  - Include valid and malformed CriticMarkup, nested spans, block boundaries,
    and every relation in `design/mutation-matrix.md`.
  - See `malarky-design.md` §§3.1-3.3 and 12.1.
  - Success: every semantic-text inclusion, exclusion, and containment rule
    has at least one fixture whose expected source bytes are explicit.

## 2. Vertical slice: trustworthy document inspection

Idea: if `validate` and `list` can expose CriticMarkup and Markdown structure
with stable source locations while never changing a file, the parsing and
source-mapping model is trustworthy enough to support mutation.

This slice gives agents useful inspection commands and tests the hardest
representation boundary before file replacement enters the system.

### 2.1. Prove the CriticMarkup overlay can reject unsafe documents

This step answers whether Malarky can recognize the annotation algebra
independently of Markdown. Its interval tree and projections unlock semantic
parsing and inverse operations. See `malarky-design.md` §§3.2, 3.3, and 5.2.

- [ ] 2.1.1. Implement the CriticMarkup overlay scanner and annotation interval
  tree.
  - Requires 1.1.1 and 1.3.1.
  - Recognize all five annotation forms, escaped delimiters, payload spans,
    malformed delimiters, and nested containment.
  - Produce net-result and original-result projections with monotonic source
    maps.
  - See `malarky-design.md` §§3.2 and 5.2.
  - Success: generated valid overlays preserve their specified projection, and
    malformed or partial intersections return positioned errors without panic.
- [ ] 2.1.2. Deliver overlay-aware `validate` diagnostics.
  - Requires 1.1.2, 1.2.2, and 2.1.1.
  - Report every detected overlay problem on standard error and reserve exit 8
    for invalid documents.
  - See `malarky-design.md` §§9 and 11.
  - Success: valid input emits `valid<TAB>FILE`; invalid input remains
    byte-identical and emits bounded, positioned diagnostics.

### 2.2. Prove semantic Markdown maps back to original source

This step answers whether visible text can be separated from Markdown syntax
without losing source identity. A successful result unlocks all matchers. See
`malarky-design.md` §§3.1, 5.3, and 12.1.

- [ ] 2.2.1. Adapt `markdown::to_mdast` into source-mapped semantic blocks.
  - Requires 2.1.1.
  - Compose mdast positions with overlay projection maps and exclude link
    destinations, reference definitions, image destinations, front matter,
    CriticMarkup delimiters, and comments.
  - Treat code, emphasis, links labels, and other visible spans as semantic
    text while retaining their span intervals.
  - See `malarky-design.md` §§3.1 and 5.3.
  - Success: multibyte and mixed-syntax fixtures map every semantic boundary to
    an ordered UTF-8 source boundary without exposing invisible metadata.
- [ ] 2.2.2. Extend `validate` through Markdown and source-map invariants.
  - Requires 2.1.2 and 2.2.1.
  - Reject cross-block CriticMarkup, invalid composed positions, non-contiguous
    blocks, and boundaries that split UTF-8.
  - See `malarky-design.md` §§5.3 and 9.
  - Success: every parser-adapter invariant has a falsifying fixture, and a
    valid document passes both overlay and Markdown validation in one command.

### 2.3. Deliver stable annotation inventory

This step answers whether agents can inspect existing annotations without
reading the full document. It also proves annotation ordering and net-result
semantics needed by inverse mutations. See `malarky-design.md` §§3.2 and 9.

- [ ] 2.3.1. Implement the tab-separated `list` contract.
  - Requires 2.1.1, 2.2.1, and 1.1.2.
  - Emit source-ordered indexes, annotation kinds, line and column ranges, and
    escaped net text on standard output.
  - See `malarky-design.md` §9 and `design/malarky-cli.txt`.
  - Success: repeated listing of identical bytes is deterministic, comments
    have empty net text, and tabs, newlines, carriage returns, and backslashes
    round-trip through the escaping rules.

## 3. Vertical slice: safe highlights and comments

Idea: if an agent can select visible text, add or remove a highlight or
comment, and inspect a byte-faithful diff without risking unrelated source,
Malarky delivers useful editorial markup through the complete mutation path.

Highlight and comment operations form the narrowest useful mutation slice: they
require semantic matching, containment, prospective validation, file
replacement, and agent feedback without the added complexity of contextual
insertion or replacement inverses.

### 3.1. Deliver exact highlight mutation end to end

This step answers whether a semantic candidate can become a safe source edit
and durable-enough in-place replacement. It establishes the mutation loop that
later verbs extend. See `malarky-design.md` §§6, 7, and 10.

- [ ] 3.1.1. Implement deterministic exact semantic matching and candidate
  selection.
  - Requires phase 2.
  - Search each block independently, preserve all qualifying candidates, and
    order them by block and source position.
  - See `malarky-design.md` §6.
  - Success: zero, one, and many matches produce exits 3, 0, and 4
    respectively, and ambiguity never mutates the file.
- [ ] 3.1.2. Implement highlight planning and containment validation.
  - Requires 3.1.1.
  - Produce source edits for plain text and reject cross-block or partial span
    intersections before rendering prospective bytes.
  - Use generated interval relations to check preservation, block containment,
    span containment, and mapping safety properties.
  - See `malarky-design.md` §§3.3, 7.1, and 12.1.
  - Success: every accepted highlight reparses, and rejected plans leave the
    source unchanged with exit 5.
- [ ] 3.1.3. Generate the successful localized unified diff.
  - Requires 3.1.2 and 1.2.3.
  - Generate and buffer the diff from the original and validated prospective
    bytes before opening a temporary file. Use the requested path, omit
    timestamps, and honour the configured context count.
  - See `malarky-design.md` §10 and `design/malarky-config.toml`.
  - Success: injected diff failures retain the original target, and an empty
    diff cannot accompany success.
- [ ] 3.1.4. Replace the file and emit the buffered diff.
  - Requires 3.1.3 and 1.2.3.
  - Reject symlinks, create the temporary file beside the target, preserve
    permissions, and synchronize content. Hold an exclusive replacement lock
    from content-fingerprint verification through rename, or use an atomic
    compare-and-replace primitive with the same fingerprint.
  - See `malarky-design.md` §§5.1 and 10.
  - Success: injected read, short-write, synchronization, fingerprint, lock,
    and rename failures retain the original target and map to exit 7. Standard
    output receives the buffered diff only after replacement succeeds.

### 3.2. Make matching forgiving without hiding ambiguity

This step answers whether whitespace tolerance can improve agent success while
retaining explainable candidates and exact source mapping. Its result
determines whether the default policy is safe. See `malarky-design.md` §6.

- [ ] 3.2.1. Add whitespace-normalized semantic matching with boundary maps.
  - Requires 3.1.1.
  - Run it only when exact matching finds no candidates, collapse Unicode
    whitespace runs, and discard non-contiguous source mappings.
  - Check mapping safety and deterministic ordering with generated Unicode and
    whitespace cases.
  - See `malarky-design.md` §§6 and 12.1.
  - Success: normalized matching changes only qualification, never the bytes
    copied from source into a mutation plan.
- [ ] 3.2.2. Implement indexed ambiguity reports and selection.
  - Requires 3.1.1 and 3.2.1.
  - Report line, column, tier, bounded excerpt, and a fingerprint of the
    document and complete ordered candidate set. Accept one-based `--match N`
    only with that selection token; reject stale tokens and out-of-range
    indexes as usage errors.
  - See `malarky-design.md` §§6 and 11.
  - Success: an agent can rerun the documented example with `--match` and edit
    only the reported candidate.

### 3.3. Extend the slice to comments and explicit removal

This step answers whether adjacent and existing annotations can be mutated
without confusing their ownership. It establishes update and deletion rules
reused by inverses. See `malarky-design.md` §§3.2 and 7.1.

- [ ] 3.3.1. Implement comment creation and update.
  - Requires steps 3.1-3.2.
  - Emit `{==TEXT==}{>>COMMENT<<}` for plain text, append a comment to an
    existing highlight, and replace an existing comment payload on reapplication.
  - See `malarky-design.md` §7.1 and `design/mutation-matrix.md`.
  - Success: comments remain invisible to ordinary matching, while reapplying
    a comment updates exactly one payload.
- [ ] 3.3.2. Implement `highlight --delete` and `comment --delete`.
  - Requires 3.3.1.
  - Remove only the requested annotation; comment deletion retains an adjacent
    highlight.
  - See `malarky-design.md` §7.1 and `design/mutation-matrix.md`.
  - Success: removing a highlight preserves its visible payload, removing a
    comment preserves the highlight, and neither operation changes neighbours.

## 4. Vertical slice: contextual changes and exact inverses

Idea: if the established mutation loop can express deletion, insertion, and
replacement with context, ambiguity controls, bulk selection, and exact
inverses, agents can perform the complete v1 editing job without general patch
generation.

This phase extends the proven source-edit path rather than introducing a
parallel editor. Each step adds one distinct range shape and its inverse law.

### 4.1. Add contextual deletion and replacement

This step answers whether `BEFORE`, target, and `AFTER` can identify the
requested payload without entering the annotation. It validates non-empty range
mutations before insertion boundaries. See `malarky-design.md` §§6 and 7.1.

- [ ] 4.1.1. Match contextual target ranges and implement deletion.
  - Requires phase 3.
  - Match concatenated context within one semantic block, expose only the
    target range, and emit `{--TEXT--}`.
  - See `malarky-design.md` §§6 and 7.1.
  - Success: empty and non-empty context distinguish repeated targets without
    including context in the annotation.
- [ ] 4.1.2. Implement contextual replacement.
  - Requires 4.1.1.
  - Emit `{~~OLD~>NEW~~}` and apply the same block and span containment rules
    as deletion.
  - See `malarky-design.md` §7.1 and `design/mutation-matrix.md`.
  - Success: the prospective net-result view contains `NEW`, while unrelated
    source and context bytes remain identical.

### 4.2. Add insertion at a mapped semantic boundary

This step answers whether an empty source range can be selected safely between
`BEFORE` and `AFTER`. It exercises a boundary shape not covered by target
mutations. See `malarky-design.md` §§3.3, 6, and 7.1.

- [ ] 4.2.1. Resolve insertion contexts to a unique in-block boundary.
  - Requires 3.2.1 and 4.1.1.
  - Match `BEFORE` followed by `AFTER`, reject block boundaries, and retain
    stable ambiguity ordering for repeated boundaries.
  - See `malarky-design.md` §§3.3 and 6.
  - Success: every candidate maps to one UTF-8 source boundary and no candidate
    can split Markdown or CriticMarkup delimiters.
- [ ] 4.2.2. Implement insertion planning and prospective validation.
  - Requires 4.2.1.
  - Emit `{++TEXT++}` at the selected boundary and reparse the complete
    prospective document before replacement.
  - See `malarky-design.md` §§7.1 and 7.3.
  - Success: insertion preserves both anchors byte for byte and rejects a
    result that would cross an annotation or Markdown span.

### 4.3. Prove the inverse laws on real annotations

This step answers whether agents can undo textual change annotations without a
generic removal command or nested counter-annotation. It validates the overlay
index against the mutation planner. See `malarky-design.md` §§3.2 and 7.2.

- [ ] 4.3.1. Undo insertion with deletion and replacement with its reverse.
  - Requires 4.1.2 and 4.2.2.
  - Recognize exact matching annotation payloads and restore plain source.
  - Verify the inverse law with generated Unicode payloads and nested,
    fully-contained Markdown spans.
  - See `malarky-design.md` §§7.2 and 12.1.
  - Success: applying each operation and inverse restores the exact original
    bytes rather than merely the same rendered text.
- [ ] 4.3.2. Undo deletion through overlay-aware insertion matching.
  - Requires 4.3.1.
  - Search deleted payloads outside the net-result view while matching their
    visible surrounding context.
  - See `malarky-design.md` §7.2 and `design/mutation-matrix.md`.
  - Success: the inverse restores the deleted payload once and does not expose
    deleted text to ordinary matching.

### 4.4. Apply all safe candidates as one transaction

This step answers whether bulk editing preserves the same safety contract as a
single mutation. It closes the selection surface required by the terms of
reference (ToR). See `malarky-design.md` §§6, 7.3, and 12.1.

- [ ] 4.4.1. Implement `--all` planning for disjoint candidates.
  - Requires steps 4.1-4.3.
  - Sort source edits in descending order, reject overlapping candidates, and
    validate the complete prospective result before writing.
  - Check all-or-nothing selection and preservation properties over generated
    candidate sets.
  - See `malarky-design.md` §§7.3 and 12.1.
  - Success: every candidate is applied in one replacement or no candidate is
    applied; overlap and one invalid plan leave the file unchanged.

## 5. Vertical slice: a trustworthy agent-facing v1

Idea: if the complete command surface remains deterministic across
configuration, matching, annotation state, and filesystem failures on
chapter-sized documents, Malarky is ready to replace token-heavy manual
rewrites in agent workflows.

This phase does not add another editing feature. It supplies system-level
evidence for the interaction surface and turns the generated guides into an
accurate operator contract.

### 5.1. Exercise the combinatorial command surface end to end

This step answers whether individually correct features remain correct in
combination. Its scenarios are first-class because flags, candidate counts,
annotation states, and I/O failures cross module boundaries. See
`malarky-design.md` §12.2.

- [ ] 5.1.1. Build the mutation command combinatorial suite.
  - Requires phase 4.
  - Cover every verb across exact and whitespace matching, zero/one/many
    candidates, `--match`/`--all`, plain and annotated targets, valid and
    partial intersections, and successful and failed replacement.
  - Use pairwise coverage for independent combinations and direct scenarios
    for every exit status, inverse, and intersection relation.
  - See `malarky-design.md` §§11 and 12.2.
  - Success: the suite invokes the installed binary, asserts stdout and stderr
    independently, and proves failure paths retain the original bytes.
- [ ] 5.1.2. Lock inspection and feedback contracts with focused snapshots.
  - Requires 2.3.1, 3.1.4, 3.2.2, and 5.1.1.
  - Snapshot help, ambiguity, list, validation, and unified-diff surfaces only
    after semantic assertions establish their meaning.
  - See `malarky-design.md` §§8-11.
  - Success: snapshots contain no paths, timestamps, colours, or other
    nondeterminism and fail only on agent-visible contract changes.

### 5.2. Establish the initial scale envelope

This step answers whether the chapter-and-paper assumption holds without
inventing a book-scale promise. Its evidence resolves the remaining ToR
question about practical response. See `terms-of-reference.md` §§7-9 and
`malarky-design.md` §§1 and 13.

- [ ] 5.2.1. Add representative chapter and paper benchmarks.
  - Requires phase 4.
  - Measure validation, listing, exact matching, whitespace matching, and one
    mutation across representative block and annotation densities.
  - See `terms-of-reference.md` §§7-9 and `malarky-design.md` §1.
  - Success: the results record document size, block count, annotation count,
    elapsed time, and peak allocation and establish an explicit v1 acceptance
    envelope without claiming book-length support.

### 5.3. Publish the implemented contract

This step answers whether an agent integrator and maintainer can use and change
the shipped tool without consulting the roadmap. It reconciles user-facing
documentation with the binary and design. See `malarky-design.md` §§8-13.

- [ ] 5.3.1. Replace generated user-guide content with agent-oriented command
  workflows.
  - Requires 5.1.1 and 5.2.1.
  - Document every verb, selection retry, configuration source, exit status,
    output channel, mutation guarantee, and version-control assumption.
  - See `malarky-design.md` §§8-11 and `design/malarky-cli.txt`.
  - Success: every documented command is executed as a documentation example
    against representative fixtures.
- [ ] 5.3.2. Reconcile maintainer documentation and repository layout.
  - Requires 5.3.1.
  - Document module ownership, adapter boundaries, property generators,
    contract artefacts, benchmark entry points, and release gates.
  - See `malarky-design.md` §§4.1 and 12.
  - Success: `docs/contents.md`, the developer guide, repository layout, ToR,
    design, and implemented module paths agree.

## 6. Deferred extensions after the core v1 promise

Idea: if the core v1 promise is trustworthy and boring to operate, broader
matching, representation, scale, and durability options can be evaluated on
their product value instead of destabilizing the source-preserving contract.

These items remain outside v1. Each step starts with evidence or a contract
decision; none may appear as an undocumented fallback in the core matcher.

### 6.1. Evaluate broader matching policies

This step asks which additional normalization, if any, improves real agent
success without raising incorrect unique matches. See `malarky-design.md` §13.

- [ ] 6.1.1. Compare punctuation, case, Unicode-compatibility, and spelling
  policies against captured no-match requests.
  - Requires phase 5.
  - Measure newly resolved requests, new ambiguities, and incorrect unique
    matches before proposing a contract change.
  - See `malarky-design.md` §13.
  - Success: an evidence-backed proposal either defines one named matcher tier
    with a safe source map or rejects the extension.

### 6.2. Evaluate additional automation contracts

This step asks whether integrations need richer output or stable annotation
identity after the text-first CLI has real use. See `malarky-design.md` §13.

- [ ] 6.2.1. Specify machine-readable JSON output only if tabular and diff
  contracts prove insufficient.
  - Requires phase 5.
  - Define versioning, stdout exclusivity, and parity with existing exits before
    implementation.
  - See `malarky-design.md` §13.
- [ ] 6.2.2. Specify persistent annotation identifiers only if workflows need
  references that survive source movement.
  - Requires phase 5.
  - Define identity, serialization, and compatibility with CriticMarkup before
    implementation.
  - See `malarky-design.md` §13.

### 6.3. Evaluate broader file and scale guarantees

This step asks whether observed use justifies expanding the initial UTF-8,
regular-file, chapter-sized, command-atomic boundary. See `malarky-design.md`
§§10 and 13.

- [ ] 6.3.1. Decide whether non-UTF-8 input, symlink following, or book-length
  documents enter scope.
  - Requires phase 5.
  - Treat each capability as a separate contract with preservation and threat
    analysis.
  - See `malarky-design.md` §13.
- [ ] 6.3.2. Decide whether power-loss durability warrants file and directory
  synchronization guarantees.
  - Requires phase 5.
  - Validate semantics on supported platforms and filesystems before changing
    the command-level atomicity claim.
  - See `malarky-design.md` §10.
  - Success: the accepted decision states exactly which crash points and
    filesystem behaviours the guarantee covers.
