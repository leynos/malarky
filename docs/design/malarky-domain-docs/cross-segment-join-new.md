# Create a cross-segment join

Create an ordered join between adjacent source-mapped segments.

## Parameters

- `source_map`: source identity and UTF-8 boundary validator.
- `left_source_boundary`: ending boundary of the left segment.
- `right_source_boundary`: starting boundary of the right segment.

## Returns

`Some(CrossSegmentJoin)` for ordered UTF-8 boundaries, otherwise `None`.

## Errors

None; reversed or invalid boundaries are represented as `None`.

## Examples

```rust,no_run
use malarky_domain::{CrossSegmentJoin, Utf8SourceMap};
let map = Utf8SourceMap::new("aé");
assert!(CrossSegmentJoin::new(&map, 1, 3).is_some());
assert!(CrossSegmentJoin::new(&map, 2, 3).is_none());
```
