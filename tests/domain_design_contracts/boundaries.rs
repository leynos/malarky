//! Boundary and generated invariant checks for the normative domain contract.

use super::{
    domain::{
        Annotation,
        AnnotationKind,
        AnnotationPayload,
        CrossSegmentJoin,
        MappedSegment,
        MutationPlan,
        SourceBoundary,
        SourceEdit,
        SourceSpan,
        Utf8SourceMap,
    },
    source_span,
};

macro_rules! require {
    ($value:expr, $message:literal $(,)?) => {{
        let Some(resolved) = $value else {
            panic!($message)
        };
        resolved
    }};
}

/// Constructs a source span that must be valid for this named test case.
fn span(source_map: &Utf8SourceMap<'_>, start: usize, end: usize) -> SourceSpan {
    require!(
        source_span(source_map, start, end),
        "test span should be valid"
    )
}

/// Creates a no-op replacement edit with a chosen source range.
const fn edit(source: SourceSpan) -> SourceEdit {
    SourceEdit {
        source,
        replacement: String::new(),
    }
}

/// Pins source-map identity independently of matching source bytes.
#[test]
fn source_spans_reject_identical_foreign_source_maps() {
    let source = "aé";
    let map = Utf8SourceMap::new(source);
    let foreign_map = Utf8SourceMap::new(source);
    let local = span(&map, 0, source.len());
    let foreign = span(&foreign_map, 0, source.len());
    let short_local = span(&map, 0, 1);

    assert!(
        foreign_map.slice(&local).is_none(),
        "foreign spans must not slice another map"
    );
    assert!(
        !local.contains(&foreign),
        "foreign spans must not be contained"
    );
    assert!(!local.overlaps(&foreign), "foreign spans must not overlap");
    assert!(
        MappedSegment::new(
            &map,
            source.to_owned(),
            foreign,
            identity_boundaries(source),
        )
        .is_none(),
        "replacement annotations require replacement payloads",
    );
    assert!(
        MappedSegment::new(
            &map,
            source.to_owned(),
            short_local,
            identity_boundaries(source),
        )
        .is_none(),
        "non-replacement annotations require a single payload",
    );
}

/// Checks complete UTF-8 mapping state and named malformed-boundary cases.
#[test]
fn mapped_segments_require_complete_ordered_utf8_boundaries() {
    let source = "é猫";
    let map = Utf8SourceMap::new(source);
    let source_span = span(&map, 0, source.len());
    let complete = identity_boundaries(source);
    let [initial, middle, terminal] = complete.as_slice() else {
        panic!("two-character source should have three boundaries");
    };
    let segment = require!(
        MappedSegment::new(
            &map,
            source.to_owned(),
            source_span.clone(),
            complete.clone(),
        ),
        "complete UTF-8 boundaries should construct a mapped segment",
    );
    assert_eq!(segment.semantic_text(), source);
    assert_eq!(segment.source(), source_span.clone());
    assert_eq!(segment.boundaries(), complete.as_slice());

    let mut missing = complete.clone();
    missing.pop();
    assert!(
        MappedSegment::new(&map, source.to_owned(), source_span.clone(), missing).is_none(),
        "missing terminal boundary must fail"
    );
    let reordered = vec![*initial, *terminal, *middle];
    assert!(
        MappedSegment::new(&map, source.to_owned(), source_span.clone(), reordered).is_none(),
        "reordered boundaries must fail"
    );
    let mut out_of_range = complete.clone();
    require!(
        out_of_range.get_mut(1),
        "complete map should retain its middle boundary",
    )
    .source_offset = source.len() + 1;
    assert!(
        MappedSegment::new(&map, source.to_owned(), source_span.clone(), out_of_range).is_none()
    );
    let mut split_code_point = complete;
    require!(
        split_code_point.get_mut(1),
        "complete map should retain its middle boundary",
    )
    .source_offset = 1;
    assert!(
        MappedSegment::new(&map, source.to_owned(), source_span, split_code_point).is_none(),
        "split UTF-8 boundaries must fail"
    );
}

/// Checks joins retain ordered UTF-8 boundaries and reject invalid endpoints.
#[test]
fn cross_segment_joins_require_ordered_utf8_endpoints() {
    let source = "é猫";
    let map = Utf8SourceMap::new(source);
    let join = require!(
        CrossSegmentJoin::new(&map, 0, 2),
        "ordered UTF-8 boundaries should construct a join",
    );
    assert_eq!(join.source_boundaries(), (0, 2));
    assert!(
        CrossSegmentJoin::new(&map, 2, 0).is_none(),
        "reversed joins must fail"
    );
    assert!(
        CrossSegmentJoin::new(&map, source.len() + 1, source.len() + 1).is_none(),
        "out-of-range joins must fail"
    );
    assert!(
        CrossSegmentJoin::new(&map, 1, 2).is_none(),
        "split UTF-8 joins must fail"
    );
}

/// Covers `Annotation` payload shape, containment, and replacement ordering.
#[test]
fn annotations_reject_invalid_payload_shapes_and_ranges() {
    let source = "alpha beta";
    let map = Utf8SourceMap::new(source);
    let foreign_map = Utf8SourceMap::new(source);
    let whole = span(&map, 0, source.len());
    let old = span(&map, 0, 5);
    let new = span(&map, 6, source.len());
    let foreign = span(&foreign_map, 0, 5);

    assert_invalid_annotation_shapes(&whole, &old, &new);
    assert_invalid_annotation_ranges((&whole, &old, &new, span(&map, 4, 8)), foreign);
    let adjacent = span(&map, 5, 5);
    let annotation = require!(
        Annotation::new(
            AnnotationKind::Replacement,
            whole,
            AnnotationPayload::Replacement {
                old: old.clone(),
                new: adjacent.clone(),
            },
        ),
        "adjacent replacement payloads should be valid",
    );
    assert_eq!(annotation.kind(), AnnotationKind::Replacement);
    assert_eq!(
        annotation.payload(),
        AnnotationPayload::Replacement { old, new: adjacent }
    );
}

/// Rejects annotation kinds paired with payload shapes they do not support.
fn assert_invalid_annotation_shapes(whole: &SourceSpan, old: &SourceSpan, new: &SourceSpan) {
    assert!(
        Annotation::new(
            AnnotationKind::Replacement,
            whole.clone(),
            AnnotationPayload::Single(old.clone()),
        )
        .is_none(),
        "payloads outside their annotation source must fail",
    );
    assert!(
        Annotation::new(
            AnnotationKind::Deletion,
            whole.clone(),
            AnnotationPayload::Replacement {
                old: old.clone(),
                new: new.clone(),
            },
        )
        .is_none(),
        "foreign payload spans must fail",
    );
}

/// Rejects payload spans that escape their container or cross ordering bounds.
fn assert_invalid_annotation_ranges(
    (whole, old, new, overlapping): (&SourceSpan, &SourceSpan, &SourceSpan, SourceSpan),
    foreign: SourceSpan,
) {
    assert!(
        Annotation::new(
            AnnotationKind::Comment,
            old.clone(),
            AnnotationPayload::Single(new.clone()),
        )
        .is_none(),
        "replacement payloads must retain OLD before NEW order",
    );
    assert!(
        Annotation::new(
            AnnotationKind::Comment,
            whole.clone(),
            AnnotationPayload::Single(foreign),
        )
        .is_none(),
        "overlapping replacement payloads must fail",
    );
    assert_invalid_replacement_order(whole, old, new, overlapping);
}

/// Rejects replacement payloads that are reversed or intersect each other.
fn assert_invalid_replacement_order(
    whole: &SourceSpan,
    old: &SourceSpan,
    new: &SourceSpan,
    overlapping: SourceSpan,
) {
    assert!(
        Annotation::new(
            AnnotationKind::Replacement,
            whole.clone(),
            AnnotationPayload::Replacement {
                old: new.clone(),
                new: old.clone(),
            },
        )
        .is_none(),
        "reversed replacement payloads must fail",
    );
    assert!(
        Annotation::new(
            AnnotationKind::Replacement,
            whole.clone(),
            AnnotationPayload::Replacement {
                old: old.clone(),
                new: overlapping,
            },
        )
        .is_none(),
        "overlapping replacement payloads must fail",
    );
}

/// Covers accepted and rejected mutation-plan ordering boundaries.
#[test]
fn mutation_plans_validate_ordering_empty_edits_and_map_identity() {
    let source = "0123456789";
    let map = Utf8SourceMap::new(source);
    let foreign_map = Utf8SourceMap::new(source);

    assert_valid_mutation_plans(&map);
    assert_invalid_mutation_plans(&map, &foreign_map);
}

/// Asserts every accepted plan preserves its supplied descending edit sequence.
fn assert_valid_mutation_plans(source_map: &Utf8SourceMap<'_>) {
    assert_plan_state(&[]);
    assert_plan_state(&[edit(span(source_map, 5, 5))]);
    assert_plan_state(&[edit(span(source_map, 7, 9)), edit(span(source_map, 2, 5))]);
    assert_plan_state(&[edit(span(source_map, 5, 8)), edit(span(source_map, 2, 5))]);
    assert_plan_state(&[edit(span(source_map, 8, 8)), edit(span(source_map, 5, 8))]);
}

/// Constructs an accepted plan and verifies it stores the supplied edits intact.
fn assert_plan_state(expected_edits: &[SourceEdit]) {
    let plan = require!(
        MutationPlan::new(Vec::new(), expected_edits.to_vec()),
        "documented descending edits should form a plan",
    );
    assert!(
        plan.selected.is_empty(),
        "empty selection must remain empty"
    );
    assert_eq!(
        plan.edits(),
        expected_edits,
        "plan must retain ordered edits"
    );
}

/// Rejects edit orderings, overlaps, and source-map identity mismatches.
fn assert_invalid_mutation_plans(source_map: &Utf8SourceMap<'_>, foreign_map: &Utf8SourceMap<'_>) {
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 2, 5)), edit(span(source_map, 7, 9))],
        )
        .is_none(),
        "ascending edits must fail",
    );
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 6, 8)), edit(span(source_map, 5, 9))],
        )
        .is_none(),
        "overlapping edits must fail",
    );
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 5, 5)), edit(span(source_map, 5, 8))],
        )
        .is_none(),
        "equal-start edits must fail",
    );
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 8, 8)), edit(span(source_map, 8, 8))],
        )
        .is_none(),
        "equal-boundary empty insertions must fail",
    );
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 8, 9)), edit(span(foreign_map, 2, 5))],
        )
        .is_none(),
        "foreign-map edits must fail",
    );
}

/// Maps every UTF-8 semantic boundary to the identical source offset.
fn identity_boundaries(source: &str) -> Vec<SourceBoundary> {
    source
        .char_indices()
        .map(|(semantic_offset, _)| SourceBoundary {
            semantic_offset,
            source_offset: semantic_offset,
        })
        .chain(std::iter::once(SourceBoundary {
            semantic_offset: source.len(),
            source_offset: source.len(),
        }))
        .collect()
}
