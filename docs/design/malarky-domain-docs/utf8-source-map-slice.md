# Slice a UTF-8 source map

Return text selected by a span belonging to this source map.

## Parameters

- `span`: the validated range to select.

## Returns

`Some(&str)` for a span from this map, otherwise `None`.

## Errors

None; map-identity mismatch is represented as `None`.

## Examples

```rust,no_run
use malarky_domain::{SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("aé");
let span = SourceSpan::new(&map, 1, 3).unwrap();
assert_eq!(map.slice(&span), Some("é"));
```
