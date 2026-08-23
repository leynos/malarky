# Return an annotation kind

Return this annotation's validated `CriticMarkup` kind.

## Parameters

None.

## Returns

The annotation kind by value.

## Errors

None.

## Examples

```rust,no_run
use malarky::domain_contract::{Annotation, AnnotationKind, AnnotationPayload, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("text");
let source = SourceSpan::new(&map, 0, 4).unwrap();
let annotation = Annotation::new(
    AnnotationKind::Comment,
    source.clone(),
    AnnotationPayload::Single(source),
).unwrap();
assert_eq!(annotation.kind(), malarky::domain_contract::AnnotationKind::Comment);
```
