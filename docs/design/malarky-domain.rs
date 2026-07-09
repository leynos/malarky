//! Normative domain shapes for Malarky's source-mapped matching design.

/// A half-open byte range in the original UTF-8 source.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}

impl SourceSpan {
    /// Create a non-inverted span.
    pub const fn new(start: usize, end: usize) -> Option<Self> {
        if start <= end { Some(Self { start, end }) } else { None }
    }

    /// Report whether two non-empty spans overlap.
    pub const fn overlaps(self, other: Self) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Report whether this span fully contains another span.
    pub const fn contains(self, other: Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }
}

/// A contiguous semantic-text segment and its corresponding source range.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MappedSegment {
    pub semantic_text: String,
    pub source: SourceSpan,
}

/// Searchable text contained by one Markdown block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticBlock {
    pub source: SourceSpan,
    pub segments: Vec<MappedSegment>,
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

/// An existing annotation and its complete source extent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Annotation {
    pub kind: AnnotationKind,
    pub source: SourceSpan,
    pub payload: SourceSpan,
}

/// A source replacement produced only after matching and validation succeed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceEdit {
    pub source: SourceSpan,
    pub replacement: String,
}

/// A complete mutation plan. Its edits are disjoint and sorted by source span.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MutationPlan {
    pub selected: Vec<MatchCandidate>,
    pub edits: Vec<SourceEdit>,
}
