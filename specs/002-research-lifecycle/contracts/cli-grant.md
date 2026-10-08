# Contract: `trcli funder, grant`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 10; FR-047 to FR-050

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `fnd`, `grt`

## Synopsis

```text
trcli funder add --name <text>
trcli grant add --title <text> --funder <ref> --amount <number> --currency <code>
                [--reference <text>] [--from <date>] [--to <date>] [--stage <stage>]
trcli grant budget add <ref> --line <text> --planned <number>
trcli grant budget spend <ref> --line <text> --amount <number> [--date <date>] [--note <text>]
trcli grant budget show <ref>
trcli grant deadline add <ref> --title <text> --date <date>
trcli grant link <ref> <record-ref>... [--remove]
trcli grant outputs <ref> --from <date> --to <date>
trcli grant acknowledgement <draft-ref>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `grant add` | Records a grant at stage `idea` unless another is given | 2 on a negative amount, unknown currency, or `--to` before `--from` |
| `grant budget show` | Planned, spent, and remaining per line; overspent lines are flagged | — |
| `grant deadline add` | Adds a reporting deadline, shown in `trcli due` | — |
| `grant outputs` | Linked outputs with activity in the period | — |
| `grant acknowledgement` | Text naming each funder and grant reference that supports a draft | — |

## Values

| Value | Rule |
|-------|------|
| `<stage>` | `idea`, `in_preparation`, `submitted`, `awarded`, `rejected`, `active`, `closed` |
| `--currency` | three-letter currency code; totals are never added across currencies |

## Example

```console
$ trcli grant budget show grt-8k3d
LINE         PLANNED      SPENT    REMAINING
Equipment   12000.00    9400.00     2600.00  BRL
Travel       4000.00    4350.00     -350.00  BRL  overspent
```
