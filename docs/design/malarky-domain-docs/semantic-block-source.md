# Return semantic block source extent

Return the complete source extent of this Markdown block.

## Parameters

None.

## Returns

A clone retaining the block's source-map identity.

## Errors

None.

## Examples

```rust,no_run
use malarky::domain_contract::{SemanticBlock, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("text");
let span = SourceSpan::new(&map, 0, 4).unwrap();
let block = SemanticBlock::new(&map, span, vec![], vec![]).unwrap();
assert_eq!(block.source().range(), 0..4);
```
