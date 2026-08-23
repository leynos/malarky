# Test source span containment

Report whether this span fully contains another span from the same source map.

## Parameters

- `other`: span whose boundaries are checked.

## Returns

`true` when both boundaries lie within this span, including empty boundaries.

## Errors

None; spans from different maps are not contained.

## Examples

```rust,no_run
use malarky::domain_contract::{SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("abc");
let whole = SourceSpan::new(&map, 0, 3).unwrap();
assert!(whole.contains(&SourceSpan::new(&map, 1, 2).unwrap()));
```
