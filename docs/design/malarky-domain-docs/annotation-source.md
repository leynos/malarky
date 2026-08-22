# Return annotation source extent

Return the complete annotation source extent.

## Parameters

None.

## Returns

A clone retaining the annotation's source-map identity.

## Errors

None.

## Examples

```rust,no_run
use malarky_domain::{Annotation, AnnotationKind, AnnotationPayload, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("text");
let source = SourceSpan::new(&map, 0, 4).unwrap();
let annotation = Annotation::new(
    AnnotationKind::Comment,
    source.clone(),
    AnnotationPayload::Single(source),
).unwrap();
assert!(annotation.source().range().start <= annotation.source().range().end);
```
