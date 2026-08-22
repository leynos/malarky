# Return a source span range

Return this span's original source-byte range.

## Parameters

None.

## Returns

The half-open range validated during construction.

## Errors

None.

## Examples

```rust,no_run
use malarky_domain::{SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("abc");
assert_eq!(SourceSpan::new(&map, 1, 2).unwrap().range(), 1..2);
```
