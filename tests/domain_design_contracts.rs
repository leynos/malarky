//! Executable contracts for Malarky's normative source-mapped domain design.

#[path = "../docs/design/malarky-domain.rs"]
mod domain;

use domain::{
    Annotation,
    AnnotationKind,
    AnnotationPayload,
    CrossSegmentJoin,
    MappedSegment,
    MatchCandidate,
    MatchTier,
    MatchingPolicy,
    MutationPlan,
    SemanticBlock,
    SourceBoundary,
    SourceEdit,
    SourceSpan,
    Utf8SourceMap,
};
use proptest::prelude::*;

macro_rules! require {
    ($value:expr, $message:literal $(,)?) => {{
        let Some(resolved) = $value else {
            panic!($message)
        };
        resolved
    }};
}

fn source_span(source_map: &Utf8SourceMap<'_>, start: usize, end: usize) -> Option<SourceSpan> {
    SourceSpan::new(source_map, start, end)
}

fn identity_segment(
    source_map: &Utf8SourceMap<'_>,
    source: &str,
    start: usize,
    end: usize,
) -> Option<MappedSegment> {
    let semantic_text = source.get(start..end)?.to_owned();
    let boundaries = semantic_text
        .char_indices()
        .map(|(semantic_offset, _)| SourceBoundary {
            semantic_offset,
            source_offset: start + semantic_offset,
        })
        .chain(std::iter::once(SourceBoundary {
            semantic_offset: semantic_text.len(),
            source_offset: end,
        }))
        .collect();

    MappedSegment::new(
        source_map,
        semantic_text,
        source_span(source_map, start, end)?,
        boundaries,
    )
}

#[test]
fn source_spans_do_not_slice_foreign_source_maps() {
    let source = String::from("aé");
    let foreign_source = format!("{source}x");
    let source_map = Utf8SourceMap::new(&source);
    let foreign_map = Utf8SourceMap::new(&foreign_source);
    let span = require!(
        source_span(&source_map, 0, source.len()),
        "test range should create a span",
    );

    assert!(foreign_map.slice(&span).is_none());
}

#[test]
fn multibyte_boundaries_construct_and_slice_safely() {
    let source = "aé猫🦀z";
    let source_map = Utf8SourceMap::new(source);

    for start in 0..=source.len() {
        for end in start..=source.len() {
            let span = SourceSpan::new(&source_map, start, end);
            let boundaries_are_valid =
                source.is_char_boundary(start) && source.is_char_boundary(end);
            assert_eq!(span.is_some(), boundaries_are_valid);

            if let Some(valid_span) = span {
                assert_eq!(source_map.slice(&valid_span), source.get(start..end));
            }
        }
    }
}

#[test]
fn semantic_blocks_require_ordered_segments_and_adjacent_joins() {
    let source = "alpha beta";
    let source_map = Utf8SourceMap::new(source);
    let block = require!(
        source_span(&source_map, 0, source.len()),
        "block span should be valid",
    );
    let first = require!(
        identity_segment(&source_map, source, 0, 5),
        "first segment should be valid",
    );
    let second = require!(
        identity_segment(&source_map, source, 6, source.len()),
        "second segment should be valid",
    );
    let join = require!(
        CrossSegmentJoin::new(&source_map, 5, 6),
        "adjacent join should be valid",
    );

    let semantic_block = require!(
        SemanticBlock::new(
            &source_map,
            block.clone(),
            vec![first.clone(), second.clone()],
            vec![join],
        ),
        "ordered segments and their join should form a block",
    );
    assert_eq!(semantic_block.source(), block);
    assert_eq!(semantic_block.segments(), &[first.clone(), second.clone()]);
    let first_segment = require!(
        semantic_block.segments().first(),
        "first segment should exist",
    );
    let first_join = require!(semantic_block.joins().first(), "first join should exist");
    assert_eq!(first_segment.semantic_text(), "alpha");
    assert_eq!(first_segment.source(), first.source());
    assert_eq!(first_segment.boundaries().len(), 6);
    assert_eq!(first_join.source_boundaries(), (5, 6));

    assert!(
        SemanticBlock::new(&source_map, block.clone(), vec![second, first], Vec::new()).is_none()
    );
    let invalid_join = require!(
        CrossSegmentJoin::new(&source_map, 5, 5),
        "source boundaries should be valid",
    );
    let first_again = require!(
        identity_segment(&source_map, source, 0, 5),
        "first segment should be valid",
    );
    let second_again = require!(
        identity_segment(&source_map, source, 6, source.len()),
        "second segment should be valid",
    );
    assert!(
        SemanticBlock::new(
            &source_map,
            block,
            vec![first_again, second_again],
            vec![invalid_join]
        )
        .is_none()
    );
}

fn assert_annotation_payloads(whole: &SourceSpan, old: &SourceSpan, new: &SourceSpan) {
    for kind in [
        AnnotationKind::Deletion,
        AnnotationKind::Insertion,
        AnnotationKind::Highlight,
        AnnotationKind::Comment,
    ] {
        let annotation = require!(
            Annotation::new(
                kind,
                (*whole).clone(),
                AnnotationPayload::Single((*old).clone()),
            ),
            "single payload should match its annotation kind",
        );
        assert_eq!(annotation.kind(), kind);
        assert_eq!(annotation.source(), (*whole).clone());
    }
    let replacement = require!(
        Annotation::new(
            AnnotationKind::Replacement,
            (*whole).clone(),
            AnnotationPayload::Replacement {
                old: (*old).clone(),
                new: (*new).clone(),
            },
        ),
        "ordered replacement payloads should be valid",
    );
    assert_eq!(
        replacement.payload(),
        AnnotationPayload::Replacement {
            old: (*old).clone(),
            new: (*new).clone(),
        }
    );
}

fn assert_valid_mutation_plan(whole: &SourceSpan, old: &SourceSpan, new: SourceSpan) {
    assert_eq!(MatchingPolicy::default(), MatchingPolicy::Whitespace);
    assert!(matches!(MatchingPolicy::Exact, MatchingPolicy::Exact));
    let candidates = vec![
        MatchCandidate {
            block: (*whole).clone(),
            target: (*old).clone(),
            tier: MatchTier::Exact,
            excerpt: String::from("alpha"),
        },
        MatchCandidate {
            block: (*whole).clone(),
            target: new.clone(),
            tier: MatchTier::Whitespace,
            excerpt: String::from("beta"),
        },
    ];
    let plan = require!(
        MutationPlan::new(
            candidates,
            vec![SourceEdit {
                source: new,
                replacement: String::from("gamma"),
            }],
        ),
        "a single edit should form a valid plan",
    );
    assert_eq!(plan.selected.len(), 2);
    assert_eq!(plan.edits().len(), 1);
}

#[test]
fn annotations_and_valid_mutation_plans_preserve_their_contracts() {
    let source = "alpha beta";
    let source_map = Utf8SourceMap::new(source);
    let whole = require!(
        source_span(&source_map, 0, source.len()),
        "whole source span should be valid",
    );
    let old = require!(
        source_span(&source_map, 0, 5),
        "old payload span should be valid",
    );
    let new = require!(
        source_span(&source_map, 6, source.len()),
        "new payload span should be valid",
    );

    assert_annotation_payloads(&whole, &old, &new);
    assert_valid_mutation_plan(&whole, &old, new);
}

proptest! {
    #[test]
    fn unicode_maps_reject_foreign_spans_and_malformed_inputs(
        characters in prop::collection::vec(any::<char>(), 1..24),
    ) {
        let source = characters.into_iter().collect::<String>();
        let foreign_source = format!("{source}x");
        let source_map = Utf8SourceMap::new(&source);
        let foreign_map = Utf8SourceMap::new(&foreign_source);
        let span = source_span(&source_map, 0, source.len())
            .ok_or_else(|| TestCaseError::fail("generated source should produce a span"))?;
        let valid_boundaries = source
            .char_indices()
            .map(|(semantic_offset, _)| SourceBoundary {
                semantic_offset,
                source_offset: semantic_offset,
            })
            .chain(std::iter::once(SourceBoundary {
                semantic_offset: source.len(),
                source_offset: source.len(),
            }))
            .collect::<Vec<_>>();

        prop_assert!(foreign_map.slice(&span).is_none());
        prop_assert!(MappedSegment::new(
            &source_map,
            source.clone(),
            span.clone(),
            valid_boundaries.clone(),
        ).is_some());

        let mut incomplete_boundaries = valid_boundaries.clone();
        incomplete_boundaries.pop();
        prop_assert!(MappedSegment::new(
            &source_map,
            source.clone(),
            span.clone(),
            incomplete_boundaries,
        ).is_none());

        let mut non_monotonic_boundaries = valid_boundaries;
        let (first_boundary, remaining_boundaries) = non_monotonic_boundaries
            .split_first_mut()
            .ok_or_else(|| TestCaseError::fail("map should include an initial boundary"))?;
        let second_boundary = remaining_boundaries
            .first_mut()
            .ok_or_else(|| TestCaseError::fail("map should include a terminal boundary"))?;
        first_boundary.source_offset = source.len();
        second_boundary.source_offset = 0;
        prop_assert!(MappedSegment::new(
            &source_map,
            source.clone(),
            span,
            non_monotonic_boundaries,
        ).is_none());
    }

    #[test]
    fn malformed_joins_and_overlapping_edits_are_rejected(
        characters in prop::collection::vec(any::<char>(), 1..24),
    ) {
        let prefix = characters.into_iter().collect::<String>();
        let source = format!("{prefix}abc");
        let source_map = Utf8SourceMap::new(&source);
        let prefix_end = prefix.len();
        let first = source_span(&source_map, prefix_end, prefix_end + 2)
            .ok_or_else(|| TestCaseError::fail("first edit range should be valid"))?;
        let second = source_span(&source_map, prefix_end + 1, prefix_end + 3)
            .ok_or_else(|| TestCaseError::fail("second edit range should be valid"))?;

        prop_assert!(CrossSegmentJoin::new(&source_map, source.len(), 0).is_none());
        let overlapping_plan = MutationPlan::new(
            Vec::new(),
            vec![
                SourceEdit { source: second, replacement: String::new() },
                SourceEdit { source: first, replacement: String::new() },
            ],
        );
        prop_assert!(overlapping_plan.is_none());
    }
}
