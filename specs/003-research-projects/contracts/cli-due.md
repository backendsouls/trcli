# Contract: `trcli due, progress`

**Spec**: [Research Projects, Milestones, and Tasks](../spec.md) — User Story 5; FR-040 to FR-043

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

## Synopsis

```text
trcli due [--within <span>] [--project <ref> | --all-projects] [--include-inactive]
trcli progress [<project-ref>]
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `due` | Tasks and milestones due in the span, in date order, overdue first, each naming its project | 2 on an invalid span |
| `due` (nothing due) | Says so and shows the next item after the span | — |
| `progress` | Milestones reached, tasks done, share of time elapsed, next milestone and days remaining; shows `behind` when time elapsed exceeds milestones reached and one is overdue | 0 with a message when there is nothing to measure |

## Values

| Value | Rule |
|-------|------|
| `--within` | `<n>d`, `<n>w`, `<n>m`; default `14d` |
| note | the progress report for a supervisor is `trcli report` (see `specs/006-reports`) |

## Example

```console
$ trcli due --within 7d --all-projects
OVERDUE  2026-10-05  task       tsk-3r8j  Send abstract            Doctorate
         2026-10-10  task       tsk-5w0h  Rerun baseline           Doctorate
         2026-10-12  milestone  mil-4t6y  Draft to supervisor      Capstone co-supervision
```
