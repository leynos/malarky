# Return mapped segment source extent

Return the complete source extent of this segment.

## Parameters

None.

## Returns

A clone retaining the source-map identity.

## Errors

None.

## Examples

```rust,no_run
use malarky::domain_contract::{MappedSegment, SourceBoundary, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("text");
let span = SourceSpan::new(&map, 0, 4).unwrap();
let segment = MappedSegment::new(&map, "text".into(), span, vec![
    SourceBoundary { semantic_offset: 0, source_offset: 0 },
    SourceBoundary { semantic_offset: 4, source_offset: 4 },
]).unwrap();
assert_eq!(segment.source().range(), 0..4);
```
