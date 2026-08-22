//! Normative domain shapes for Malarky's source-mapped matching design.
//!
//! [ADR 001](../adr-001-source-preserving-semantic-model.md) governs source
//! mapping. [ADR 002](../adr-002-atomic-mutation-and-file-replacement.md)
//! governs mutation planning.
use std::rc::Rc;
/// The UTF-8 source against which domain offsets are validated.
#[derive(Clone, Debug)]
pub struct Utf8SourceMap<'source> {
    source: &'source str,
    identity: Rc<SourceMapIdentity>,
}
impl<'source> Utf8SourceMap<'source> {
    #[doc = include_str!("malarky-domain-docs/utf8-source-map-new.md")]
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            identity: Rc::new(SourceMapIdentity),
        }
    }
    #[doc = include_str!("malarky-domain-docs/utf8-source-map-slice.md")]
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
    source_map: Rc<SourceMapIdentity>,
    start: usize,
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
    pub const fn range(&self) -> std::ops::Range<usize> { self.start..self.end }
    #[doc = include_str!("malarky-domain-docs/source-span-overlaps.md")]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.same_source_map(other) && self.start < other.end && other.start < self.end
    }
    #[doc = include_str!("malarky-domain-docs/source-span-contains.md")]
    pub fn contains(&self, other: &Self) -> bool {
        self.same_source_map(other) && self.start <= other.start && other.end <= self.end
    }
    fn belongs_to(&self, source_map: &Utf8SourceMap<'_>) -> bool {
        Rc::ptr_eq(&self.source_map, &source_map.identity)
    }
    fn same_source_map(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source_map, &other.source_map)
    }
}
/// One semantic-text boundary and its corresponding source boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceBoundary {
    pub semantic_offset: usize,
    pub source_offset: usize,
}
/// A contiguous semantic-text segment with a map for every text boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MappedSegment {
    semantic_text: String,
    source: SourceSpan,
    boundaries: Vec<SourceBoundary>,
}
impl MappedSegment {
    #[doc = include_str!("malarky-domain-docs/mapped-segment-new.md")]
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
    pub fn semantic_text(&self) -> &str { &self.semantic_text }
    #[doc = include_str!("malarky-domain-docs/mapped-segment-source.md")]
    pub fn source(&self) -> SourceSpan { self.source.clone() }
    #[doc = include_str!("malarky-domain-docs/mapped-segment-boundaries.md")]
    pub fn boundaries(&self) -> &[SourceBoundary] { &self.boundaries }
}
/// An explicit join between adjacent mapped segments after projection.
#[derive(Clone, Debug)]
pub struct CrossSegmentJoin {
    source_map: Rc<SourceMapIdentity>,
    left_source_boundary: usize,
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
    pub const fn source_boundaries(&self) -> (usize, usize) {
        (self.left_source_boundary, self.right_source_boundary)
    }
    fn belongs_to(&self, source_map: &Utf8SourceMap<'_>) -> bool {
        Rc::ptr_eq(&self.source_map, &source_map.identity)
    }
}
/// Searchable text contained by one Markdown block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticBlock {
    source: SourceSpan,
    segments: Vec<MappedSegment>,
    joins: Vec<CrossSegmentJoin>,
}
impl SemanticBlock {
    #[doc = include_str!("malarky-domain-docs/semantic-block-new.md")]
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
    pub fn source(&self) -> SourceSpan { self.source.clone() }
    #[doc = include_str!("malarky-domain-docs/semantic-block-segments.md")]
    pub fn segments(&self) -> &[MappedSegment] { &self.segments }
    #[doc = include_str!("malarky-domain-docs/semantic-block-joins.md")]
    pub fn joins(&self) -> &[CrossSegmentJoin] { &self.joins }
}
/// The maximum normalization applied while matching semantic text.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MatchingPolicy {
    Exact,
    #[default]
    Whitespace,
}
/// The tier that produced a candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchTier {
    Exact,
    Whitespace,
}
/// A deterministic candidate exposed to selection and mutation planning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchCandidate {
    pub block: SourceSpan,
    pub target: SourceSpan,
    pub tier: MatchTier,
    pub excerpt: String,
}
/// `CriticMarkup` annotation kinds recognized by the overlay parser.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnnotationKind {
    Deletion,
    Insertion,
    Replacement,
    Highlight,
    Comment,
}
/// Payload spans whose shape is specific to the annotation kind.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnnotationPayload {
    Single(SourceSpan),
    Replacement { old: SourceSpan, new: SourceSpan },
}
/// An existing annotation and its complete source extent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Annotation {
    kind: AnnotationKind,
    source: SourceSpan,
    payload: AnnotationPayload,
}
impl Annotation {
    #[doc = include_str!("malarky-domain-docs/annotation-new.md")]
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
    pub const fn kind(&self) -> AnnotationKind { self.kind }
    #[doc = include_str!("malarky-domain-docs/annotation-source.md")]
    pub fn source(&self) -> SourceSpan { self.source.clone() }
    #[doc = include_str!("malarky-domain-docs/annotation-payload.md")]
    pub fn payload(&self) -> AnnotationPayload { self.payload.clone() }
}
/// A source replacement produced only after matching and validation succeed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceEdit {
    pub source: SourceSpan,
    pub replacement: String,
}
/// A complete mutation plan with descending, disjoint source edits.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MutationPlan {
    pub selected: Vec<MatchCandidate>,
    edits: Vec<SourceEdit>,
}
impl MutationPlan {
    #[doc = include_str!("malarky-domain-docs/mutation-plan-new.md")]
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
    pub fn edits(&self) -> &[SourceEdit] { &self.edits }
}
