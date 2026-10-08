# Contract: `trcli prereg`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 7; FR-033 to FR-037

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `pre`

## Synopsis

```text
trcli prereg add <hypothesis-ref> <experiment-ref> --analysis <text> [--sample-size <n>]
                 [--stopping <text>] [--outcomes <text>]
trcli prereg freeze <ref>
trcli prereg amend <ref> --reason <text> --change <text>
trcli prereg compare <ref>
trcli prereg verify <ref>
trcli prereg export <ref> --to <file>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `prereg add` | Writes an editable pre-registration | 3 when the hypothesis or experiment is not found |
| `prereg freeze` | Seals the content and the time; it can no longer be edited | 2 when a required section is empty |
| `prereg edit` | Shared verb; allowed only before freezing | 2 after freezing, pointing to `prereg amend` |
| `prereg amend` | Adds a dated amendment with a reason; the original stays visible | 2 when not frozen |
| `prereg compare` | Lists differences between the plan and what runs recorded; flags runs that started before the freeze | — |
| `prereg verify` | Checks the frozen content was not altered outside the tool | 6 `check_failed` |

## Values

| Value | Rule |
|-------|------|
| `--analysis` | required; 1–20,000 characters |
| `--sample-size` | positive whole number |

## Example

```console
$ trcli prereg freeze pre-0d5m
Frozen pre-0d5m at 2026-10-08 15:30 UTC. It can now only be amended.
```
