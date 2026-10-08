# Contract: `trcli part`

**Spec**: [Manuscripts](../spec.md) — User Story 6; FR-037 to FR-041

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Referred to by**: its number in the outline (`2`, `2.3`) or any unique start of its title

## Synopsis

```text
trcli part add <manuscript-ref> <title> [--under <part>] [--at <position>] [--status <status>]
               [--target-length <n>] [--deadline <date>] [--writer <staff-ref>] [--file <path>]
trcli part add <manuscript-ref> --manuscript <manuscript-ref> [--under <part>] [--at <position>]
trcli part list <manuscript-ref>
trcli part edit <manuscript-ref> <part> [--title <text>] [--status <status>] [--target-length <n>]
                [--length <n>] [--deadline <date>] [--writer <staff-ref>] [--file <path>]
trcli part move <manuscript-ref> <part> [--under <part> | --top] [--at <position>]
trcli part rm <manuscript-ref> <part> [--keep-children]
trcli part progress <manuscript-ref>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `part add <title>` | Adds a part to the outline, at the end or at `--at`, optionally beneath another part | 2 on an empty title or a target length that is not positive |
| `part add --manuscript` | Makes another manuscript a part; its status follows that manuscript's stage | 2 when it would make a manuscript a part of itself, directly or through others |
| `part list` | The outline, nested and numbered as it will appear, with status, length against target, deadline, and writer | — |
| `part edit` | Changes only what is given; `--length` enters a length by hand when the file cannot be read as text | 3 when the part is not found or the start of the title matches several |
| `part move` | Moves a part to another position or beneath another part; numbering follows | 2 when moved beneath itself |
| `part rm` | Removes a part; with parts beneath it, asks whether to remove them or keep them one level up (`--keep-children`) | 5 `confirmation_required` |
| `part progress` | Number of parts at each status; current length against target per part and overall; parts that are late | 0 with a message when the manuscript has no parts |
| (warning) | A part's deadline later than the manuscript's is accepted with a warning | 0 |

## Values

| Value | Rule |
|-------|------|
| `<status>` | `not_started` (default), `outlined`, `drafting`, `drafted`, `revised`, `final` |
| `--target-length`, `--length` | words; positive whole numbers |
| `--file` | the file the part is written in; must exist; used to count its length |
| a part that is a manuscript | its status is derived: `idea` → `not_started`, `outlining` → `outlined`, `drafting` → `drafting`, `revising` → `drafted`, `submitted`/`under_review` → `revised`, `accepted`/`published` → `final` |
| from a template | when a document is created for a manuscript with `trcli doc new`, the template's sections become its parts unless it already has parts |

## Example

```console
$ trcli part progress ms-0h7y
ms-0h7y "Doctoral thesis" · 7 parts · 31,400 of 60,000 words (52%)

  1    Introduction             drafted        4,100 /  5,000
  2    Background               final          9,800 / 10,000
  3    Method                   drafting       6,200 /  8,000
  4    Study A  (ms-5t1q)       final          8,900 /  9,000   published
  5    Study B  (ms-3e8k)       drafting       2,400 /  9,000   late: due 2026-09-30
  6    Discussion               not started        0 / 12,000
  7    Conclusion               not started        0 /  7,000

final 2 · drafted 1 · drafting 2 · not started 2 · late 1
```
