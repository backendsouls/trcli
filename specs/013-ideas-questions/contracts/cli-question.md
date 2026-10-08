# Contract: `trcli question, hypothesis`

**Spec**: [Ideas, Topics, Questions, and Hypotheses](../spec.md) — User Story 3; FR-026 to FR-034

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `rq`, `hyp`

## Synopsis

```text
trcli question add --statement <text> [--motivation <text>] [--parent <ref>]
                   [--importance <1-5>] [--feasibility <1-5>] [--topic <topic>]...
trcli question overview <ref>
trcli question close <ref> --as <answered|abandoned> --note <text>
trcli question reopen <ref>
trcli question link <ref> <record-ref>... [--remove]
trcli question unlinked [--kind <kind>]
trcli question quiet [--for <span>]
trcli question history <ref>
trcli hypothesis add <question-ref> --statement <text> [--supports-if <text>] [--refutes-if <text>]
trcli hypothesis status <ref> <untested|supported|refuted|inconclusive>
trcli hypothesis link <ref> <record-ref>... [--remove]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `question add` | Adds a question, or a sub-question with `--parent` | 2 when `--parent` would create a loop |
| `question overview` | Sub-questions, hypotheses with status, linked records grouped by kind | — |
| `question close` | Marks answered or abandoned; the note is required | 2 without `--note` |
| `question reopen` | Sets the status back to `open`; history is kept | — |
| `question link` | Links records that address the question | 3 when a record is not found |
| `question unlinked` | Records linked to no question, by kind | — |
| `question quiet` | Open questions with no activity for longer than the span, with the date of their last activity | 6 `check_failed` when any |
| `question history` | Status changes and earlier wordings of the statement | — |
| (a result behind a hypothesis is replaced or removed) | The hypothesis is flagged for another look; its status does not change by itself | — |
| (an experiment is concluded with an outcome for a hypothesis) | The tool offers to update the hypothesis; it does so only on acceptance | — |
| `hypothesis add` | Adds a hypothesis under a question | — |
| `hypothesis status` | Changes status; anything but `untested` requires linked evidence | 2 `validation_failed` when no result is linked as evidence |
| `question rm` | Shared verb | 5 `blocked_by_dependents` when it has sub-questions |

## Values

| Value | Rule |
|-------|------|
| `--statement` | 1–500 characters, required |
| `--motivation`, `--supports-if`, `--refutes-if`, `--note` | up to 20,000 characters |
| list filters | `question list --status <open\|answered\|abandoned>`; `hypothesis list --question <ref> --status <status>` |

## Example

```console
$ trcli hypothesis status hyp-2c9d supported
error: 1 value is invalid
  status supported   needs at least one result linked as evidence
                     (link one with: trcli result evidence <result> hyp-2c9d --supports)
Nothing was changed.
```
