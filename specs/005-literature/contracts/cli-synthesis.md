# Contract: `trcli extract, theme`

**Spec**: [Literature, References, and Bibliography](../spec.md) — User Story 7; FR-049 to FR-052

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

## Synopsis

```text
trcli extract item add <review-ref> --name <text> --kind <text|number|yes-no|choice> [--choices <a,b,c>] [--required]
trcli extract item list <review-ref> | edit <review-ref> <name> ... | rm <review-ref> <name>
trcli extract set <review-ref> <reference-ref> <item>=<value>... [--page <page>] [--from <annotation-ref>]
trcli extract missing <review-ref>
trcli extract matrix <review-ref> [--to <file>] [--format <csv|markdown>]
trcli theme add <review-ref> <name>
trcli theme assign <review-ref> <name> <reference-ref>... [--remove]
trcli theme coverage <review-ref>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `extract item add` | Defines something to record from every included reference | 2 when `choice` has no `--choices` |
| `extract item edit` / `rm` | Shows how many values are affected first | 5 `confirmation_required` |
| `extract set` | Records values for a reference; `--from` copies the text of an annotation and keeps the link | 2 on a value of the wrong kind; 3 when the reference is not included in the review |
| `extract missing` | Included references with required items not filled | 6 `check_failed` when any |
| `extract matrix` | References as rows, items as columns, empty cells visible; `--to` writes a table with citations | — |
| `theme coverage` | Number of references per theme; themes with none are shown as gaps | — |

## Values

| Value | Rule |
|-------|------|
| `<item>` | the name of an item defined for the review |

## Example

```console
$ trcli theme coverage rev-41bc
Efficiency        14
Interpretability   9
Low-resource       0   gap
```
