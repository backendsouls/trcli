# Contract: `trcli ethics, dmp`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 9; FR-042 to FR-046

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `eth`, `dmp`

## Synopsis

```text
trcli ethics add --body <text> --reference <text> --from <date> --to <date> [--conditions <text>]
trcli ethics link <ref> <dataset-ref>... [--remove]
trcli dataset sensitivity <dataset-ref> <none|personal|sensitive> [--consent <text>]
trcli dmp add --title <text> [--section <name>=<text>]...
trcli dmp cover <ref> <dataset-ref>... [--remove]
trcli dmp uncovered
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `ethics add` | Records an approval; its status (pending, valid, expiring soon, expired) follows from its dates | 2 when `--to` is before `--from` |
| `ethics link` | Links an approval to the datasets it covers | — |
| `dataset sensitivity` | Flags a dataset as holding personal or sensitive data, with its consent basis | — |
| `dmp cover` / `uncovered` | Links datasets to a data-management plan; lists datasets no plan covers | — |
| (effect on `run start`) | A run that uses a flagged dataset without a valid linked approval warns and asks for acknowledgement | 5 `confirmation_required` without a terminal and without `--yes`; the acknowledgement is recorded |

## Values

| Value | Rule |
|-------|------|
| `--reference` | the approval's reference number; 1–100 characters |
| list filters | `ethics list --status <status>` |

## Example

```console
$ trcli run start exp-2b6r
warning: dat-3n8x "Interviews" holds sensitive data and its approval eth-5c1z expired on 2026-09-30
Continue and record your acknowledgement? [y/N]
```
