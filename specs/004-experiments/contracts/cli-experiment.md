# Contract: `trcli experiment`

**Spec**: [Experiments](../spec.md) — User Stories 1 and 7; FR-001 to FR-011, FR-048 to FR-055

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `exp`

## Synopsis

```text
trcli experiment add --name <text> [--objective <text>] [--kind <kind>] [--hypothesis <ref>]...
                     [--method <ref>] [--dataset <ref>[@<version>]]... [--software <ref>]...
                     [--responsible <staff-ref>]... [--sample-size <n>] [--replicates <n>]
trcli experiment variable add <ref> --name <text> --role <independent|dependent|controlled> [--unit <text>] [--description <text>]
trcli experiment variable rm <ref> <name>
trcli experiment condition add <ref> --name <text> [--set <variable>=<value>]... [--control]
trcli experiment condition rm <ref> <name>
trcli experiment design <ref>
trcli experiment conclude <ref> --outcome <hypothesis-ref>=<supported|refuted|inconclusive>...
                          --conclusion <text> [--result <ref>]... [--limitations <text>] [--next <text>]
trcli experiment abandon <ref> --reason <text>
trcli experiment reopen <ref>
trcli experiment replicate <ref> [--name <text>]
trcli experiment compare <ref> <ref>...
trcli experiment report <ref> [--to <file>]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `experiment add` | Creates an experiment at status `planned` | 3 when a linked record is not found |
| `experiment variable add` | Adds a variable to the design | 2 when the name is already used in the experiment |
| `experiment condition add` | Adds a condition; at most one is the control | 2 when `--set` names a variable that is not independent, or a second `--control` |
| `experiment design` | Variables, conditions, planned sample size and replicates | — |
| (design change with runs) | Warns that past runs used the earlier design; they keep what they recorded | 5 `confirmation_required` |
| `experiment conclude` | Records the conclusion and sets status `completed`; offers to update each hypothesis's status | 2 when there is no succeeded run; 5 when a run is paused or in progress |
| `experiment abandon` | Sets status `abandoned`; the reason is required | 2 without `--reason` |
| `experiment replicate` | Creates a new experiment copying design, pipeline, parameters, and links; each shows the other | — |
| `experiment compare` | Results of the same name side by side, whether they agree, and design differences | — |
| `experiment rm` | Shared verb | 5 `blocked_by_dependents` when it has runs |

## Values

| Value | Rule |
|-------|------|
| `--kind` | `computational`, `laboratory`, `field`, `survey`, `simulation`, `other` |
| `--sample-size`, `--replicates` | positive whole numbers |
| list filters | `--status`, `--kind`, `--hypothesis <ref>`, `--responsible <ref>` |

## Example

```console
$ trcli experiment condition add exp-2b6r --name treated --set dose=5
error: 1 value is invalid
  --set dose=5   "dose" is not an independent variable of exp-2b6r (independent: temperature)
Nothing was changed.
```
