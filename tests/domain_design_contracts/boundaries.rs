//! Boundary and generated invariant checks for the normative domain contract.

use proptest::prelude::*;

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

fn span(source_map: &Utf8SourceMap<'_>, start: usize, end: usize) -> SourceSpan {
    require!(
        source_span(source_map, start, end),
        "test span should be valid"
    )
}

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

    assert!(foreign_map.slice(&local).is_none());
    assert!(!local.contains(&foreign));
    assert!(!local.overlaps(&foreign));
    assert!(
        MappedSegment::new(
            &map,
            source.to_owned(),
            foreign,
            identity_boundaries(source),
        )
        .is_none()
    );
    assert!(
        MappedSegment::new(
            &map,
            source.to_owned(),
            short_local,
            identity_boundaries(source),
        )
        .is_none()
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
    assert!(
        Annotation::new(
            AnnotationKind::Replacement,
            whole,
            AnnotationPayload::Replacement {
                old,
                new: span(&map, 5, 5),
            },
        )
        .is_some()
    );
}

fn assert_invalid_annotation_shapes(whole: &SourceSpan, old: &SourceSpan, new: &SourceSpan) {
    assert!(
        Annotation::new(
            AnnotationKind::Replacement,
            whole.clone(),
            AnnotationPayload::Single(old.clone()),
        )
        .is_none()
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
        .is_none()
    );
}

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
        .is_none()
    );
    assert!(
        Annotation::new(
            AnnotationKind::Comment,
            whole.clone(),
            AnnotationPayload::Single(foreign),
        )
        .is_none()
    );
    assert_invalid_replacement_order(whole, old, new, overlapping);
}

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
        .is_none()
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
        .is_none()
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

fn assert_valid_mutation_plans(source_map: &Utf8SourceMap<'_>) {
    assert!(MutationPlan::new(Vec::new(), Vec::new()).is_some());
    assert!(MutationPlan::new(Vec::new(), vec![edit(span(source_map, 5, 5))]).is_some());
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 7, 9)), edit(span(source_map, 2, 5))],
        )
        .is_some()
    );
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 5, 8)), edit(span(source_map, 2, 5))],
        )
        .is_some()
    );
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 8, 8)), edit(span(source_map, 5, 8))],
        )
        .is_some()
    );
}

fn assert_invalid_mutation_plans(source_map: &Utf8SourceMap<'_>, foreign_map: &Utf8SourceMap<'_>) {
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 2, 5)), edit(span(source_map, 7, 9))],
        )
        .is_none()
    );
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 6, 8)), edit(span(source_map, 5, 9))],
        )
        .is_none()
    );
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 5, 5)), edit(span(source_map, 5, 8))],
        )
        .is_none()
    );
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 8, 8)), edit(span(source_map, 8, 8))],
        )
        .is_none()
    );
    assert!(
        MutationPlan::new(
            Vec::new(),
            vec![edit(span(source_map, 8, 9)), edit(span(foreign_map, 2, 5))],
        )
        .is_none()
    );
}

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

// Generates arbitrary semantic and source offsets around UTF-8 boundaries.
proptest! {
    #[test]
    fn mapped_segments_enforce_generated_boundary_safety(
        tail in prop::collection::vec(any::<char>(), 0..12),
        raw_offset in any::<usize>(),
    ) {
        let source = format!("é{}", tail.into_iter().collect::<String>());
        let map = Utf8SourceMap::new(&source);
        let span = SourceSpan::new(&map, 0, source.len())
            .ok_or_else(|| TestCaseError::fail("whole source span should be valid"))?;
        let valid = identity_boundaries(&source);
        let candidate = raw_offset.rem_euclid(source.len() + 3);
        let previous = valid
            .get(valid.len().saturating_sub(2))
            .ok_or_else(|| TestCaseError::fail("map should include two boundaries"))?
            .source_offset;
        let mut altered = valid.clone();
        let terminal = altered
            .last_mut()
            .ok_or_else(|| TestCaseError::fail("map should include a terminal boundary"))?;
        terminal.source_offset = candidate;
        let expected = candidate >= previous
            && candidate <= source.len()
            && source.is_char_boundary(candidate);

        prop_assert!(MappedSegment::new(
            &map,
            source.clone(),
            span.clone(),
            valid,
        ).is_some());
        prop_assert_eq!(
            MappedSegment::new(&map, source.clone(), span, altered).is_some(),
            expected,
        );
    }

    #[test]
    fn joins_and_constructor_ranges_reject_generated_invalid_offsets(
        tail in prop::collection::vec(any::<char>(), 0..12),
        left_raw in any::<usize>(),
        right_raw in any::<usize>(),
        a in 0usize..11,
        b in 0usize..11,
        c in 0usize..11,
        d in 0usize..11,
    ) {
        let unicode_source = format!("é{}", tail.into_iter().collect::<String>());
        let unicode_map = Utf8SourceMap::new(&unicode_source);
        let left = left_raw.rem_euclid(unicode_source.len() + 3);
        let right = right_raw.rem_euclid(unicode_source.len() + 3);
        let join_is_valid = left <= right
            && unicode_source.is_char_boundary(left)
            && unicode_source.is_char_boundary(right);
        prop_assert_eq!(
            CrossSegmentJoin::new(&unicode_map, left, right).is_some(),
            join_is_valid,
        );

        let ascii_map = Utf8SourceMap::new("0123456789");
        let scope = SourceSpan::new(&ascii_map, 2, 8)
            .ok_or_else(|| TestCaseError::fail("scope span should be valid"))?;
        let first = SourceSpan::new(&ascii_map, a.min(b), a.max(b))
            .ok_or_else(|| TestCaseError::fail("first span should be valid"))?;
        let second = SourceSpan::new(&ascii_map, c.min(d), c.max(d))
            .ok_or_else(|| TestCaseError::fail("second span should be valid"))?;
        let payload_is_valid = scope.contains(&first)
            && scope.contains(&second)
            && first.range().end <= second.range().start;
        prop_assert_eq!(
            Annotation::new(
                AnnotationKind::Replacement,
                scope,
                AnnotationPayload::Replacement {
                    old: first.clone(),
                    new: second.clone(),
                },
            ).is_some(),
            payload_is_valid,
        );
        let plan_is_valid = first.range().start > second.range().start
            && first.range().start >= second.range().end
            && !first.overlaps(&second);
        prop_assert_eq!(
            MutationPlan::new(Vec::new(), vec![edit(first), edit(second)]).is_some(),
            plan_is_valid,
        );
    }
}
