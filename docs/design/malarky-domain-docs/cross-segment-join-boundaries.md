# Return cross-segment join boundaries

Return the ordered source boundaries on either side of this join.

## Parameters

None.

## Returns

`(left, right)` in the validated source order.

## Errors

None.

## Examples

```rust,no_run
use malarky_domain::{CrossSegmentJoin, Utf8SourceMap};
let map = Utf8SourceMap::new("a");
let join = CrossSegmentJoin::new(&map, 0, 1).unwrap();
assert!(join.source_boundaries().0 <= join.source_boundaries().1);
```
