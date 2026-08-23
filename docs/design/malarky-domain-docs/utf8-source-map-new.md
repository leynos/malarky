# Create a UTF-8 source map

Associate a source map with immutable UTF-8 source.

## Parameters

- `source`: original document bytes decoded as UTF-8.

## Returns

A source map with a unique identity for subsequent spans.

## Errors

None.

## Examples

```rust,no_run
use malarky::domain_contract::{SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("é");
let span = SourceSpan::new(&map, 0, 2).unwrap();
assert_eq!(map.slice(&span), Some("é"));
```
