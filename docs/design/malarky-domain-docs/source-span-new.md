# Create a source span

Create a non-inverted span on UTF-8 boundaries.

## Parameters

- `source_map`: map that validates byte offsets.
- `start`: inclusive byte offset.
- `end`: exclusive byte offset.

## Returns

`Some(SourceSpan)` for ordered in-range character boundaries, otherwise `None`.

## Errors

None; invalid offsets are represented as `None`.

## Examples

```rust,no_run
use malarky::domain_contract::{SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("aé");
assert!(SourceSpan::new(&map, 1, 3).is_some());
assert!(SourceSpan::new(&map, 2, 3).is_none());
```
