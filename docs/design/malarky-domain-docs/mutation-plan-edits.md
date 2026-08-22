# Return mutation plan edits

Return edits in the descending order required for application.

## Parameters

None.

## Returns

The complete validated edit sequence without copying it.

## Errors

None.

## Examples

```rust,no_run
use malarky_domain::{MutationPlan, SourceEdit, SourceSpan, Utf8SourceMap};
let map = Utf8SourceMap::new("abc");
let later = SourceSpan::new(&map, 2, 3).unwrap();
let earlier = SourceSpan::new(&map, 0, 1).unwrap();
let plan = MutationPlan::new(vec![], vec![
    SourceEdit { source: later, replacement: "C".into() },
    SourceEdit { source: earlier, replacement: "A".into() },
]).unwrap();
assert!(plan.edits().windows(2).all(|pair| pair[0].source.range().start > pair[1].source.range().start));
```
