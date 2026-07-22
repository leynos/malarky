//! Normative domain shapes for Malarky's source-mapped matching design.
//!
//! [ADR 001](../adr-001-source-preserving-semantic-model.md) governs source
//! mapping. [ADR 002](../adr-002-atomic-mutation-and-file-replacement.md)
//! governs mutation planning.

/// The UTF-8 source against which domain offsets are validated.
#[derive(Clone, Copy, Debug)]
pub struct Utf8SourceMap<'source> {
    source: &'source str,
}

impl<'source> Utf8SourceMap<'source> {
    /// Associate a source map with original UTF-8 source.
    pub const fn new(source: &'source str) -> Self { Self { source } }

    /// Return the source text selected by a validated span.
    pub fn slice(self, span: SourceSpan) -> Option<&'source str> { self.source.get(span.range()) }
}

/// A half-open byte range in the original UTF-8 source.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SourceSpan {
    start: usize,
    end: usize,
}

impl SourceSpan {
    /// Create a non-inverted span whose offsets are UTF-8 boundaries.
    ///
    /// # Parameters
    ///
    /// - `source_map`: the original UTF-8 source map against which both offsets
    ///   are validated.
    /// - `start`: the inclusive source-byte offset.
    /// - `end`: the exclusive source-byte offset.
    ///
    /// # Returns
    ///
    /// `Some(SourceSpan)` when `start <= end`, both offsets are in bounds, and
    /// neither offset splits a multibyte code point; otherwise, `None`.
    /// Empty spans are valid at any code-point boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use malarky_domain::{SourceSpan, Utf8SourceMap};
    ///
    /// let source = Utf8SourceMap::new("aéz");
    /// assert_eq!(SourceSpan::new(&source, 1, 3).map(SourceSpan::range), Some(1..3));
    /// assert!(SourceSpan::new(&source, 2, 3).is_none());
    /// assert!(SourceSpan::new(&source, 3, 1).is_none());
    /// assert!(SourceSpan::new(&source, 3, 3).is_some());
    /// ```
    pub fn new(source_map: &Utf8SourceMap<'_>, start: usize, end: usize) -> Option<Self> {
        (start <= end
            && source_map.source.is_char_boundary(start)
            && source_map.source.is_char_boundary(end))
            .then_some(Self { start, end })
    }

    /// Return the original source-byte range.
    pub const fn range(self) -> std::ops::Range<usize> { self.start..self.end }

    /// Report whether two non-empty spans overlap.
    ///
    /// # Parameters
    ///
    /// - `other`: the span to compare with this span.
    ///
    /// # Returns
    ///
    /// `true` only when both spans share at least one source byte. Empty and
    /// adjacent spans do not overlap.
    ///
    /// # Examples
    ///
    /// ```
    /// use malarky_domain::{SourceSpan, Utf8SourceMap};
    ///
    /// let source = Utf8SourceMap::new("aéz");
    /// let accented = SourceSpan::new(&source, 1, 3).unwrap();
    /// let adjacent = SourceSpan::new(&source, 3, 4).unwrap();
    /// let empty = SourceSpan::new(&source, 1, 1).unwrap();
    /// assert!(!accented.overlaps(adjacent));
    /// assert!(!accented.overlaps(empty));
    /// ```
    pub const fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Report whether this span fully contains another span.
    ///
    /// # Parameters
    ///
    /// - `other`: the span whose boundaries must lie within this span.
    ///
    /// # Returns
    ///
    /// `true` when both boundaries of `other` lie within this span. A span
    /// contains itself, an empty span at either boundary, and every adjacent
    /// internal boundary; it does not contain an empty span beyond its end.
    ///
    /// # Examples
    ///
    /// ```
    /// use malarky_domain::{SourceSpan, Utf8SourceMap};
    ///
    /// let source = Utf8SourceMap::new("aéz");
    /// let whole = SourceSpan::new(&source, 0, 4).unwrap();
    /// let accented = SourceSpan::new(&source, 1, 3).unwrap();
    /// let at_end = SourceSpan::new(&source, 4, 4).unwrap();
    /// assert!(whole.contains(accented));
    /// assert!(whole.contains(at_end));
    /// ```
    pub const fn contains(self, other: Self) -> bool {
        self.start <= other.start && other.end <= self.end
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
    /// Create a segment only when every semantic boundary has a safe mapping.
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
        let has_complete_map = expected_semantic_boundaries.eq(
            boundaries.iter().map(|boundary| boundary.semantic_offset),
        );
        let has_monotonic_source = boundaries.windows(2).all(|pair| {
            pair[0].source_offset <= pair[1].source_offset
        });
        let has_safe_source_boundaries = boundaries.iter().all(|boundary| {
            source_map.source.is_char_boundary(boundary.source_offset)
                && source.start <= boundary.source_offset
                && boundary.source_offset <= source.end
        });
        (has_complete_map && has_monotonic_source && has_safe_source_boundaries)
            .then_some(Self { semantic_text, source, boundaries })
    }

    /// Return the searchable text represented by this segment.
    pub fn semantic_text(&self) -> &str { &self.semantic_text }

    /// Return the complete source extent represented by this segment.
    pub const fn source(&self) -> SourceSpan { self.source }

    /// Return the complete monotonic semantic-to-source boundary map.
    pub fn boundaries(&self) -> &[SourceBoundary] { &self.boundaries }
}

/// An explicit join between adjacent mapped segments after projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CrossSegmentJoin {
    left_source_boundary: usize,
    right_source_boundary: usize,
}

impl CrossSegmentJoin {
    /// Create an ordered join whose endpoints are safe UTF-8 boundaries.
    pub fn new(
        source_map: &Utf8SourceMap<'_>,
        left_source_boundary: usize,
        right_source_boundary: usize,
    ) -> Option<Self> {
        (left_source_boundary <= right_source_boundary
            && source_map.source.is_char_boundary(left_source_boundary)
            && source_map.source.is_char_boundary(right_source_boundary))
            .then_some(Self { left_source_boundary, right_source_boundary })
    }

    /// Return the ordered source boundaries on either side of the join.
    pub const fn source_boundaries(self) -> (usize, usize) {
        (self.left_source_boundary, self.right_source_boundary)
    }
}

/// Searchable text contained by one Markdown block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticBlock {
    pub source: SourceSpan,
    pub segments: Vec<MappedSegment>,
    pub joins: Vec<CrossSegmentJoin>,
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

/// CriticMarkup annotation kinds recognized by the overlay parser.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnnotationKind {
    Deletion,
    Insertion,
    Replacement,
    Highlight,
    Comment,
}

/// Payload spans whose shape is specific to the annotation kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    /// Validate that the payload shape matches the kind and lies within source.
    pub fn new(kind: AnnotationKind, source: SourceSpan, payload: AnnotationPayload) -> Option<Self> {
        let has_expected_shape = matches!(
            (kind, payload),
            (AnnotationKind::Replacement, AnnotationPayload::Replacement { .. })
                | (
                    AnnotationKind::Deletion
                        | AnnotationKind::Insertion
                        | AnnotationKind::Highlight
                        | AnnotationKind::Comment,
                    AnnotationPayload::Single(_),
                )
        );
        let payload_is_contained = match payload {
            AnnotationPayload::Single(span) => source.contains(span),
            AnnotationPayload::Replacement { old, new } => {
                source.contains(old) && source.contains(new) && old.end <= new.start
            }
        };
        (has_expected_shape && payload_is_contained).then_some(Self { kind, source, payload })
    }

    /// Return the validated annotation kind.
    pub const fn kind(&self) -> AnnotationKind { self.kind }

    /// Return the complete annotation source extent.
    pub const fn source(&self) -> SourceSpan { self.source }

    /// Return the kind-specific payload span or spans.
    pub const fn payload(&self) -> AnnotationPayload { self.payload }
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
    /// Create a plan only when edits are in descending order and do not overlap.
    pub fn new(selected: Vec<MatchCandidate>, edits: Vec<SourceEdit>) -> Option<Self> {
        let is_valid = edits.windows(2).all(|pair| {
            let earlier = pair[0].source;
            let later = pair[1].source;
            earlier.start > later.start && earlier.start >= later.end && !earlier.overlaps(later)
        });
        is_valid.then_some(Self { selected, edits })
    }

    /// Return edits in the descending order required for application.
    pub fn edits(&self) -> &[SourceEdit] { &self.edits }
}

#[cfg(test)]
mod tests {
    use super::{SourceSpan, Utf8SourceMap};

    #[test]
    fn multibyte_boundaries_construct_and_slice_safely() {
        let source = "aé猫🦀z";
        let source_map = Utf8SourceMap::new(source);
        for start in 0..=source.len() {
            for end in start..=source.len() {
                let span = SourceSpan::new(&source_map, start, end);
                let boundaries_are_valid = source.is_char_boundary(start) && source.is_char_boundary(end);
                assert_eq!(span.is_some(), boundaries_are_valid);
                if let Some(valid_span) = span {
                    assert_eq!(source_map.slice(valid_span), source.get(start..end));
                }
            }
        }
    }
}
