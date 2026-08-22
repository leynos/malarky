# Return mapped segment semantic text

Return the searchable text projected by this segment.

## Parameters

None.

## Returns

The semantic text without copying it.

## Errors

None.

## Examples

```rust,no_run
use malarky_domain::{MappedSegment, SourceBoundary, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("a");
let span = SourceSpan::new(&map, 0, 1).unwrap();
let segment = MappedSegment::new(&map, "a".into(), span, vec![
    SourceBoundary { semantic_offset: 0, source_offset: 0 },
    SourceBoundary { semantic_offset: 1, source_offset: 1 },
]).unwrap();
assert_eq!(segment.semantic_text(), "a");
```
