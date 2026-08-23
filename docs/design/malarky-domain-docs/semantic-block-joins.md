# Return semantic block joins

Return joins between this block's adjacent semantic segments.

## Parameters

None.

## Returns

The validated join sequence without copying it.

## Errors

None.

## Examples

```rust,no_run
use malarky::domain_contract::{CrossSegmentJoin, MappedSegment, SemanticBlock, SourceBoundary, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("ab");
let block_span = SourceSpan::new(&map, 0, 2).unwrap();
let left_span = SourceSpan::new(&map, 0, 1).unwrap();
let right_span = SourceSpan::new(&map, 1, 2).unwrap();
let left = MappedSegment::new(&map, "a".into(), left_span, vec![
    SourceBoundary { semantic_offset: 0, source_offset: 0 },
    SourceBoundary { semantic_offset: 1, source_offset: 1 },
]).unwrap();
let right = MappedSegment::new(&map, "b".into(), right_span, vec![
    SourceBoundary { semantic_offset: 0, source_offset: 1 },
    SourceBoundary { semantic_offset: 1, source_offset: 2 },
]).unwrap();
let join = CrossSegmentJoin::new(&map, 1, 1).unwrap();
let block = SemanticBlock::new(&map, block_span, vec![left, right], vec![join]).unwrap();
assert_eq!(block.joins().len(), block.segments().len().saturating_sub(1));
```
