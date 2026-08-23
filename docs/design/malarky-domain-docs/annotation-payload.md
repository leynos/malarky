# Return annotation payloads

Return this annotation's kind-specific payload span or spans.

## Parameters

None.

## Returns

A clone of the validated payload representation.

## Errors

None.

## Examples

```rust,no_run
use malarky::domain_contract::{Annotation, AnnotationKind, AnnotationPayload, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("text");
let source = SourceSpan::new(&map, 0, 4).unwrap();
let annotation = Annotation::new(
    AnnotationKind::Highlight,
    source.clone(),
    AnnotationPayload::Single(source),
).unwrap();
assert!(matches!(annotation.payload(), malarky::domain_contract::AnnotationPayload::Single(_)));
```
