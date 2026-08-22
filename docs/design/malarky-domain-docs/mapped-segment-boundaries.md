# Return mapped segment boundaries

Return the complete monotonic semantic-to-source boundary map.

## Parameters

None.

## Returns

The validated boundary sequence without copying it.

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
assert_eq!(segment.boundaries().first().unwrap().semantic_offset, 0);
```
