# Create a mutation plan

Create a complete, descending, disjoint source-edit plan.

## Parameters

- `selected`: candidates selected for this command.
- `edits`: source edits in descending start-offset order.

## Returns

`Some(MutationPlan)` when edits share a source map and are disjoint.

## Errors

None; ordering, overlap, or map-identity failures are represented as `None`.

## Examples

```rust,no_run
use malarky::domain_contract::MutationPlan;
assert!(MutationPlan::new(vec![], vec![]).is_some());
```
