# Return semantic block segments

Return this block's ordered semantic segments.

## Parameters

None.

## Returns

The validated segment sequence without copying it.

## Errors

None.

## Examples

```rust,no_run
use malarky_domain::{MappedSegment, SemanticBlock, SourceBoundary, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("a");
let span = SourceSpan::new(&map, 0, 1).unwrap();
let segment = MappedSegment::new(&map, "a".into(), span.clone(), vec![
    SourceBoundary { semantic_offset: 0, source_offset: 0 },
    SourceBoundary { semantic_offset: 1, source_offset: 1 },
]).unwrap();
let block = SemanticBlock::new(&map, span, vec![segment], vec![]).unwrap();
assert!(block.segments().windows(2).all(|pair| pair[0].source().range().end <= pair[1].source().range().start));
```
