# Test source span overlap

Report whether two non-empty spans from the same source map overlap.

## Parameters

- `other`: span to compare.

## Returns

`true` only when both spans share a source byte; adjacent and empty spans do
not.

## Errors

None; spans from different maps do not overlap.

## Examples

```rust,no_run
use malarky_domain::{SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("abc");
let left = SourceSpan::new(&map, 0, 1).unwrap();
let right = SourceSpan::new(&map, 1, 2).unwrap();
assert!(!left.overlaps(&right));
```
