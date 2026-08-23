use malarky::domain_contract::{
    Annotation,
    AnnotationKind,
    AnnotationPayload,
    CrossSegmentJoin,
    MappedSegment,
    MutationPlan,
    SemanticBlock,
    SourceBoundary,
    SourceEdit,
    SourceSpan,
    Utf8SourceMap,
};

fn main() {
    let source = "é";
    let source_map = Utf8SourceMap::new(source);
    let Some(span) = SourceSpan::new(&source_map, 0, source.len()) else {
        return;
    };
    let Some(segment) = MappedSegment::new(
        &source_map,
        source.to_owned(),
        span.clone(),
        vec![
            SourceBoundary {
                semantic_offset: 0,
                source_offset: 0,
            },
            SourceBoundary {
                semantic_offset: source.len(),
                source_offset: source.len(),
            },
        ],
    ) else {
        return;
    };
    let Some(_join) = CrossSegmentJoin::new(&source_map, 0, source.len()) else {
        return;
    };
    let Some(_block) = SemanticBlock::new(&source_map, span.clone(), vec![segment], Vec::new())
    else {
        return;
    };
    let Some(_annotation) = Annotation::new(
        AnnotationKind::Highlight,
        span.clone(),
        AnnotationPayload::Single(span.clone()),
    ) else {
        return;
    };
    let Some(_plan) = MutationPlan::new(
        Vec::new(),
        vec![SourceEdit {
            source: span,
            replacement: String::from("é"),
        }],
    ) else {
        return;
    };
}
