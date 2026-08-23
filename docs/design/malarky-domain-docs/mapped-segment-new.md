# Create a mapped segment

Create one source-mapped semantic segment.

## Parameters

- `source_map`: source identity and UTF-8 boundary validator.
- `semantic_text`: searchable projected text.
- `source`: complete source extent producing the text.
- `boundaries`: one monotonic source mapping for every semantic boundary.

## Returns

`Some(MappedSegment)` only for a complete, monotonic, contained map.

## Errors

None; invalid maps or foreign spans are represented as `None`.

## Examples

```rust,no_run
use malarky::domain_contract::{MappedSegment, SourceBoundary, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("é");
let source = SourceSpan::new(&map, 0, 2).unwrap();
assert!(MappedSegment::new(&map, "é".into(), source, vec![
    SourceBoundary { semantic_offset: 0, source_offset: 0 },
    SourceBoundary { semantic_offset: 2, source_offset: 2 },
]).is_some());
```
