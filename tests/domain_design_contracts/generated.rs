//! Generated Unicode range checks for the normative domain contract.

use proptest::prelude::*;

use super::domain::{
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
};

/// Creates the complete identity boundary map for a UTF-8 source string.
fn identity_boundaries(source: &str) -> Vec<SourceBoundary> {
    source
        .char_indices()
        .map(|(offset, _)| SourceBoundary {
            semantic_offset: offset,
            source_offset: offset,
        })
        .chain(std::iter::once(SourceBoundary {
            semantic_offset: source.len(),
            source_offset: source.len(),
        }))
        .collect()
}

// Generates mappings whose acceptance follows the documented boundary rules.
proptest! {
    #[test]
    fn mapped_segments_follow_generated_utf8_boundary_predicates(
        tail in prop::collection::vec(any::<char>(), 0..12),
        raw_offset in any::<usize>(),
    ) {
        let source = format!("é{}", tail.into_iter().collect::<String>());
        let map = Utf8SourceMap::new(&source);
        let span = SourceSpan::new(&map, 0, source.len())
            .ok_or_else(|| TestCaseError::fail("whole source span should be valid"))?;
        let boundaries = identity_boundaries(&source);
        let segment = MappedSegment::new(
            &map,
            source.clone(),
            span.clone(),
            boundaries.clone(),
        )
        .ok_or_else(|| TestCaseError::fail("complete identity map should be valid"))?;
        prop_assert_eq!(segment.semantic_text(), source.as_str());
        prop_assert_eq!(segment.source(), span.clone());
        prop_assert_eq!(segment.boundaries(), boundaries.as_slice());

        let candidate = raw_offset.rem_euclid(source.len() + 3);
        let previous = boundaries
            .get(boundaries.len().saturating_sub(2))
            .ok_or_else(|| TestCaseError::fail("map should include two boundaries"))?
            .source_offset;
        let mut altered = boundaries;
        let terminal = altered
            .last_mut()
            .ok_or_else(|| TestCaseError::fail("map should include a terminal boundary"))?;
        terminal.source_offset = candidate;
        let expected = candidate >= previous
            && candidate <= source.len()
            && source.is_char_boundary(candidate);
        prop_assert_eq!(
            MappedSegment::new(&map, source.clone(), span, altered).is_some(),
            expected,
        );
    }

    #[test]
    fn joins_payloads_and_edits_follow_generated_range_predicates(
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
        let generated_join = CrossSegmentJoin::new(&unicode_map, left, right);
        prop_assert_eq!(generated_join.is_some(), join_is_valid);
        if let Some(join) = generated_join {
            prop_assert_eq!(join.source_boundaries(), (left, right));
        }

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
        let generated_annotation = Annotation::new(
            AnnotationKind::Replacement,
            scope.clone(),
            AnnotationPayload::Replacement {
                old: first.clone(),
                new: second.clone(),
            },
        );
        prop_assert_eq!(generated_annotation.is_some(), payload_is_valid);
        if let Some(annotation) = generated_annotation {
            prop_assert_eq!(annotation.kind(), AnnotationKind::Replacement);
            prop_assert_eq!(annotation.source(), scope);
            prop_assert_eq!(
                annotation.payload(),
                AnnotationPayload::Replacement {
                    old: first.clone(),
                    new: second.clone(),
                },
            );
        }

        let first_edit = SourceEdit {
            source: first.clone(),
            replacement: String::new(),
        };
        let second_edit = SourceEdit {
            source: second.clone(),
            replacement: String::new(),
        };
        let plan_is_valid = first.range().start > second.range().start
            && first.range().start >= second.range().end
            && !first.overlaps(&second);
        let generated_plan = MutationPlan::new(
            Vec::new(),
            vec![first_edit.clone(), second_edit.clone()],
        );
        prop_assert_eq!(generated_plan.is_some(), plan_is_valid);
        if let Some(plan) = generated_plan {
            prop_assert!(plan.selected.is_empty());
            prop_assert_eq!(plan.edits(), &[first_edit, second_edit]);
        }
    }
}
