# Malarky mutation matrix

Table 1: Required behaviour when a requested mutation encounters existing
CriticMarkup.

| Requested operation     | Existing state                                    | Required result                                       |
| ----------------------- | ------------------------------------------------- | ----------------------------------------------------- |
| `delete`                | Plain text                                        | Wrap the target in `{--` and `--}`                    |
| `delete`                | Matching insertion                                | Remove the insertion and its delimiters               |
| `insert`                | Plain boundary                                    | Insert `{++TEXT++}` at the selected boundary          |
| `insert`                | Matching deletion                                 | Restore the deleted payload as plain text             |
| `replace OLD NEW`       | Plain `OLD`                                       | Emit `{~~OLD~>NEW~~}`                                 |
| `replace NEW OLD`       | Matching `OLD` to `NEW` replacement               | Restore `OLD` as plain text                           |
| `highlight`             | Plain text                                        | Wrap the target in `{==` and `==}`                    |
| `highlight --delete`    | Matching highlight                                | Remove highlight delimiters                           |
| `comment TEXT COMMENT`  | Plain text                                        | Emit `{==TEXT==}{>>COMMENT<<}`                        |
| `comment TEXT COMMENT`  | Highlighted text                                  | Add `{>>COMMENT<<}` after the highlight               |
| `comment TEXT COMMENT`  | Target with comment                               | Replace the existing comment payload                  |
| `comment --delete TEXT` | Target with comment                               | Remove only the comment and its delimiters            |
| Any mutation            | Disjoint annotation                               | Leave the annotation unchanged                        |
| Any mutation            | Fully encompassed annotation or Markdown span     | Permit the plan if the result reparses and validates  |
| Any mutation            | Partially intersected annotation or Markdown span | Reject without changing the file                      |
| Any mutation            | Target crosses a Markdown block                   | Reject without changing the file                      |
| `--all` mutation        | Selected source spans overlap                     | Reject the complete command without changing the file |
