# Create a semantic block

Create a block from one source map and ordered source-mapped segments.

## Parameters

- `source_map`: map shared by the block, segments, and joins.
- `source`: complete Markdown block extent.
- `segments`: ordered, contained semantic segments.
- `joins`: joins corresponding to adjacent segments.

## Returns

`Some(SemanticBlock)` only when all values share the map and satisfy
containment.

## Errors

None; identity, ordering, containment, or join failures are represented as
`None`.

## Examples

```rust,no_run
use malarky_domain::{SemanticBlock, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("text");
let source = SourceSpan::new(&map, 0, 4).unwrap();
assert!(SemanticBlock::new(&map, source, vec![], vec![]).is_some());
```
