# Contract: `trcli milestone`

**Spec**: [Research Projects, Milestones, and Tasks](../spec.md) — User Story 3; FR-019 to FR-027

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `mil`

## Synopsis

```text
trcli milestone add --title <text> --date <date> [--description <text>] [--required] [--deliverable <text>]
trcli milestone propose [--accept <all|n,n,...>]
trcli milestone move <ref> --date <date> --reason <text>
trcli milestone reach <ref> [--date <date>]
trcli milestone reopen <ref>
trcli milestone timeline
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `milestone add` | Adds a milestone to the current project at status `upcoming` | 5 when the date is outside the project's period and `--yes` is not given |
| `milestone propose` | Shows the project type's typical milestones and adds those accepted; already present ones are skipped | — |
| `milestone move` | Changes the target date; the old date and the reason are kept | 2 without `--reason` |
| `milestone reach` | Marks reached and shows any delay | 2 when the date is in the future; 5 when tasks are unfinished and `--yes` is not given |
| `milestone timeline` | Milestones in date order with status; the next is highlighted, overdue ones show days late | — |
| `milestone rm` | Attached tasks are kept and shown as attached to no milestone | 5 `confirmation_required` |

## Values

| Value | Rule |
|-------|------|
| `--date` | `YYYY-MM-DD` |
| list filters | `--status <upcoming\|reached\|missed\|cancelled>`, `--required`, `--overdue` |

## Example

```console
$ trcli milestone timeline
  2026-12-15  reached   mil-2a8f  Coursework completed          (3 days late)
▶ 2027-06-30  upcoming  mil-7c3k  Qualifying exam               required · in 265 days
  2028-03-01  upcoming  mil-9e1p  Proposal defended             suggested date, not confirmed
```
