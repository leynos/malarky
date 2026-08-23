//! Normative domain shapes for Malarky's source-mapped matching design.
//!
//! [ADR 001](../adr-001-source-preserving-semantic-model.md) governs source
//! mapping. [ADR 002](../adr-002-atomic-mutation-and-file-replacement.md)
//! governs mutation planning.
use std::rc::Rc;
/// The UTF-8 source against which domain offsets are validated.
#[derive(Clone, Debug)]
pub struct Utf8SourceMap<'source> {
    /// Original UTF-8 text whose byte boundaries this map validates.
    source: &'source str,
    /// Opaque identity shared with every span from this exact source map.
    identity: Rc<SourceMapIdentity>,
}
impl<'source> Utf8SourceMap<'source> {
    #[doc = include_str!("malarky-domain-docs/utf8-source-map-new.md")]
    #[must_use]
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            identity: Rc::new(SourceMapIdentity),
        }
    }
    #[doc = include_str!("malarky-domain-docs/utf8-source-map-slice.md")]
    #[must_use]
    pub fn slice(&self, span: &SourceSpan) -> Option<&'source str> {
        span.belongs_to(self)
            .then(|| self.source.get(span.range()))?
    }
}
/// An opaque identity for one UTF-8 source map.
#[derive(Debug)]
struct SourceMapIdentity;
/// A half-open byte range in the original UTF-8 source.
#[derive(Clone, Debug)]
pub struct SourceSpan {
    /// Opaque identity of the source map that validated this span.
    source_map: Rc<SourceMapIdentity>,
    /// Inclusive lower bound of the half-open byte range.
    start: usize,
    /// Exclusive upper bound of the half-open byte range.
    end: usize,
}
impl PartialEq for SourceSpan {
    fn eq(&self, other: &Self) -> bool {
        self.same_source_map(other) && self.range() == other.range()
    }
}
impl Eq for SourceSpan {}
impl SourceSpan {
    #[doc = include_str!("malarky-domain-docs/source-span-new.md")]
    #[must_use]
    pub fn new(source_map: &Utf8SourceMap<'_>, start: usize, end: usize) -> Option<Self> {
        (start <= end
            && source_map.source.is_char_boundary(start)
            && source_map.source.is_char_boundary(end))
        .then_some(Self {
            source_map: Rc::clone(&source_map.identity),
            start,
            end,
        })
    }
    #[doc = include_str!("malarky-domain-docs/source-span-range.md")]
    #[must_use]
    pub const fn range(&self) -> std::ops::Range<usize> { self.start..self.end }
    #[doc = include_str!("malarky-domain-docs/source-span-overlaps.md")]
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.same_source_map(other) && self.start < other.end && other.start < self.end
    }
    #[doc = include_str!("malarky-domain-docs/source-span-contains.md")]
    #[must_use]
    pub fn contains(&self, other: &Self) -> bool {
        self.same_source_map(other) && self.start <= other.start && other.end <= self.end
    }
    /// Reports whether this span was created by `source_map`.
    fn belongs_to(&self, source_map: &Utf8SourceMap<'_>) -> bool {
        Rc::ptr_eq(&self.source_map, &source_map.identity)
    }
    /// Reports whether both spans retain the same source-map identity.
    fn same_source_map(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source_map, &other.source_map)
    }
}
/// One semantic-text boundary and its corresponding source boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceBoundary {
    /// Byte offset in the semantic-text value.
    pub semantic_offset: usize,
    /// Corresponding UTF-8 byte offset in the original source.
    pub source_offset: usize,
}
/// A contiguous semantic-text segment with a map for every text boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MappedSegment {
    /// Searchable net-result text emitted by this segment.
    semantic_text: String,
    /// Contiguous source interval that produced the semantic text.
    source: SourceSpan,
    /// Source mapping for every UTF-8 boundary in the semantic text.
    boundaries: Vec<SourceBoundary>,
}
impl MappedSegment {
    #[doc = include_str!("malarky-domain-docs/mapped-segment-new.md")]
    #[must_use]
    pub fn new(
        source_map: &Utf8SourceMap<'_>,
        semantic_text: String,
        source: SourceSpan,
        boundaries: Vec<SourceBoundary>,
    ) -> Option<Self> {
        let expected_semantic_boundaries = semantic_text
            .char_indices()
            .map(|(offset, _)| offset)
            .chain(std::iter::once(semantic_text.len()));
        let has_complete_map = expected_semantic_boundaries
            .eq(boundaries.iter().map(|boundary| boundary.semantic_offset));
        let has_monotonic_source = boundaries
            .windows(2)
            .all(|pair| matches!(pair, [left, right] if left.source_offset <= right.source_offset));
        let has_safe_source_boundaries = boundaries.iter().all(|boundary| {
            source_map.source.is_char_boundary(boundary.source_offset)
                && source.start <= boundary.source_offset
                && boundary.source_offset <= source.end
        });
        (source.belongs_to(source_map)
            && has_complete_map
            && has_monotonic_source
            && has_safe_source_boundaries)
            .then_some(Self {
                semantic_text,
                source,
                boundaries,
            })
    }
    #[doc = include_str!("malarky-domain-docs/mapped-segment-semantic-text.md")]
    #[must_use]
    pub fn semantic_text(&self) -> &str { &self.semantic_text }
    #[doc = include_str!("malarky-domain-docs/mapped-segment-source.md")]
    #[must_use]
    pub fn source(&self) -> SourceSpan { self.source.clone() }
    #[doc = include_str!("malarky-domain-docs/mapped-segment-boundaries.md")]
    #[must_use]
    pub fn boundaries(&self) -> &[SourceBoundary] { &self.boundaries }
}
/// An explicit join between adjacent mapped segments after projection.
#[derive(Clone, Debug)]
pub struct CrossSegmentJoin {
    /// Opaque identity of the source map that validated both endpoints.
    source_map: Rc<SourceMapIdentity>,
    /// Source boundary at the end of the preceding segment.
    left_source_boundary: usize,
    /// Source boundary at the start of the following segment.
    right_source_boundary: usize,
}
impl PartialEq for CrossSegmentJoin {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source_map, &other.source_map)
            && self.source_boundaries() == other.source_boundaries()
    }
}
impl Eq for CrossSegmentJoin {}
impl CrossSegmentJoin {
    #[doc = include_str!("malarky-domain-docs/cross-segment-join-new.md")]
    #[must_use]
    pub fn new(
        source_map: &Utf8SourceMap<'_>,
        left_source_boundary: usize,
        right_source_boundary: usize,
    ) -> Option<Self> {
        (left_source_boundary <= right_source_boundary
            && source_map.source.is_char_boundary(left_source_boundary)
            && source_map.source.is_char_boundary(right_source_boundary))
        .then_some(Self {
            source_map: Rc::clone(&source_map.identity),
            left_source_boundary,
            right_source_boundary,
        })
    }
    #[doc = include_str!("malarky-domain-docs/cross-segment-join-boundaries.md")]
    #[must_use]
    pub const fn source_boundaries(&self) -> (usize, usize) {
        (self.left_source_boundary, self.right_source_boundary)
    }
    /// Reports whether this join was created by `source_map`.
    fn belongs_to(&self, source_map: &Utf8SourceMap<'_>) -> bool {
        Rc::ptr_eq(&self.source_map, &source_map.identity)
    }
}
/// Searchable text contained by one Markdown block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticBlock {
    /// Source extent of the single Markdown block.
    source: SourceSpan,
    /// Ordered source-mapped semantic segments in this block.
    segments: Vec<MappedSegment>,
    /// Exact source joins between adjacent semantic segments.
    joins: Vec<CrossSegmentJoin>,
}
impl SemanticBlock {
    #[doc = include_str!("malarky-domain-docs/semantic-block-new.md")]
    #[must_use]
    pub fn new(
        source_map: &Utf8SourceMap<'_>,
        source: SourceSpan,
        segments: Vec<MappedSegment>,
        joins: Vec<CrossSegmentJoin>,
    ) -> Option<Self> {
        let has_contained_segments = segments
            .iter()
            .all(|segment| source.contains(&segment.source));
        let has_ordered_segments = segments
            .windows(2)
            .all(|pair| matches!(pair, [left, right] if left.source.end <= right.source.start));
        let joins_link_adjacent_segments = joins.len() == segments.len().saturating_sub(1)
            && joins.iter().zip(segments.windows(2)).all(|(join, pair)| {
                let [left, right] = pair else { return false };
                join.belongs_to(source_map)
                    && source.start <= join.left_source_boundary
                    && join.right_source_boundary <= source.end
                    && join.left_source_boundary == left.source.end
                    && join.right_source_boundary == right.source.start
            });
        (source.belongs_to(source_map)
            && has_contained_segments
            && has_ordered_segments
            && joins_link_adjacent_segments)
            .then_some(Self {
                source,
                segments,
                joins,
            })
    }
    #[doc = include_str!("malarky-domain-docs/semantic-block-source.md")]
    #[must_use]
    pub fn source(&self) -> SourceSpan { self.source.clone() }
    #[doc = include_str!("malarky-domain-docs/semantic-block-segments.md")]
    #[must_use]
    pub fn segments(&self) -> &[MappedSegment] { &self.segments }
    #[doc = include_str!("malarky-domain-docs/semantic-block-joins.md")]
    #[must_use]
    pub fn joins(&self) -> &[CrossSegmentJoin] { &self.joins }
}
/// The maximum normalization applied while matching semantic text.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MatchingPolicy {
    /// Match semantic text byte-for-byte.
    Exact,
    /// Match only after documented whitespace normalization.
    #[default]
    Whitespace,
}
/// The tier that produced a candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchTier {
    /// Candidate came from an exact semantic-text match.
    Exact,
    /// Candidate came from a whitespace-normalized match.
    Whitespace,
}
/// A deterministic candidate exposed to selection and mutation planning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchCandidate {
    /// Semantic block that contains the candidate.
    pub block: SourceSpan,
    /// Exact source range selected by the candidate.
    pub target: SourceSpan,
    /// Matching tier that produced this candidate.
    pub tier: MatchTier,
    /// Human-readable excerpt reported to the caller.
    pub excerpt: String,
}
/// `CriticMarkup` annotation kinds recognized by the overlay parser.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnnotationKind {
    /// A deletion annotation whose payload is absent from the net result.
    Deletion,
    /// An insertion annotation whose payload appears in the net result.
    Insertion,
    /// A replacement annotation with distinct old and new payloads.
    Replacement,
    /// A highlight annotation whose payload remains in the net result.
    Highlight,
    /// A comment annotation whose payload is not semantic text.
    Comment,
}
/// Payload spans whose shape is specific to the annotation kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnnotationPayload {
    /// Payload for a non-replacement annotation.
    Single(SourceSpan),
    /// Ordered old and new payloads for a replacement annotation.
    Replacement {
        /// Original payload removed by the replacement.
        old: SourceSpan,
        /// Net-result payload introduced by the replacement.
        new: SourceSpan,
    },
}
/// An existing annotation and its complete source extent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Annotation {
    /// `CriticMarkup` form represented by this annotation.
    kind: AnnotationKind,
    /// Complete source extent, including delimiters and payloads.
    source: SourceSpan,
    /// Kind-specific payload spans inside `source`.
    payload: AnnotationPayload,
}
impl Annotation {
    #[doc = include_str!("malarky-domain-docs/annotation-new.md")]
    #[must_use]
    pub fn new(
        kind: AnnotationKind,
        source: SourceSpan,
        payload: AnnotationPayload,
    ) -> Option<Self> {
        let has_expected_shape = matches!(
            (kind, &payload),
            (
                AnnotationKind::Replacement,
                AnnotationPayload::Replacement { .. }
            ) | (
                AnnotationKind::Deletion
                    | AnnotationKind::Insertion
                    | AnnotationKind::Highlight
                    | AnnotationKind::Comment,
                &AnnotationPayload::Single(_),
            )
        );
        let payload_is_contained = match &payload {
            AnnotationPayload::Single(span) => source.contains(span),
            AnnotationPayload::Replacement { old, new } => {
                source.contains(old) && source.contains(new) && old.end <= new.start
            }
        };
        (has_expected_shape && payload_is_contained).then_some(Self {
            kind,
            source,
            payload,
        })
    }
    #[doc = include_str!("malarky-domain-docs/annotation-kind.md")]
    #[must_use]
    pub const fn kind(&self) -> AnnotationKind { self.kind }
    #[doc = include_str!("malarky-domain-docs/annotation-source.md")]
    #[must_use]
    pub fn source(&self) -> SourceSpan { self.source.clone() }
    #[doc = include_str!("malarky-domain-docs/annotation-payload.md")]
    #[must_use]
    pub fn payload(&self) -> AnnotationPayload { self.payload.clone() }
}
/// A source replacement produced only after matching and validation succeed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceEdit {
    /// Source range to replace.
    pub source: SourceSpan,
    /// Replacement bytes for `source`.
    pub replacement: String,
}
/// A complete mutation plan with descending, disjoint source edits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MutationPlan {
    /// Candidates selected for this mutation.
    pub selected: Vec<MatchCandidate>,
    /// Descending, disjoint source replacements ready for application.
    edits: Vec<SourceEdit>,
}
impl MutationPlan {
    #[doc = include_str!("malarky-domain-docs/mutation-plan-new.md")]
    #[must_use]
    pub fn new(selected: Vec<MatchCandidate>, edits: Vec<SourceEdit>) -> Option<Self> {
        let has_one_source_map = edits.first().is_none_or(|first| {
            edits
                .iter()
                .all(|edit| first.source.same_source_map(&edit.source))
        });
        let has_descending_disjoint_edits = edits.windows(2).all(|pair| {
            let [earlier, later] = pair else { return false };
            earlier.source.start > later.source.start
                && earlier.source.start >= later.source.end
                && !earlier.source.overlaps(&later.source)
        });
        (has_one_source_map && has_descending_disjoint_edits).then_some(Self { selected, edits })
    }

    #[doc = include_str!("malarky-domain-docs/mutation-plan-edits.md")]
    #[must_use]
    pub fn edits(&self) -> &[SourceEdit] { &self.edits }
}
