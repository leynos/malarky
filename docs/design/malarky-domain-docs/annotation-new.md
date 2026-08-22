# Create an annotation

Create an annotation with a kind-correct, contained payload.

## Parameters

- `kind`: `CriticMarkup` annotation form.
- `source`: complete annotation extent.
- `payload`: single payload or ordered replacement payloads.

## Returns

`Some(Annotation)` for a matching payload shape contained by `source`.

## Errors

None; shape, containment, or replacement-order failures are represented as
`None`.

## Examples

```rust,no_run
use malarky_domain::{Annotation, AnnotationKind, AnnotationPayload, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("text");
let source = SourceSpan::new(&map, 0, 4).unwrap();
assert!(Annotation::new(AnnotationKind::Highlight, source.clone(), AnnotationPayload::Single(source)).is_some());
```
