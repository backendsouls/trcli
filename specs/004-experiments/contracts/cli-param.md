# Contract: `trcli param, metric`

**Spec**: [Experiments](../spec.md) — User Story 4; FR-030, FR-031, FR-033

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

## Synopsis

```text
trcli param add <experiment-ref> --name <name> --kind <text|number|integer|yes-no|choice>
                [--default <value>] [--min <n>] [--max <n>] [--choices <a,b,c>]
trcli param list <experiment-ref> | edit <experiment-ref> <name> ... | rm <experiment-ref> <name>
trcli metric add <run-ref> <name>=<value>... [--step <n>]
trcli metric list <run-ref> [--name <name>]
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `param add` | Declares a parameter the experiment accepts | 2 when `--default` is outside what is allowed, or the name is reused |
| `param edit` | Changes allowed values; past runs keep theirs and are marked as outside the current definition | — |
| `metric add` | Records named values for a run; repeated values under a name form an ordered series | 2 when a value is not a finite number |
| (from a step) | An automated step reports metrics by printing lines of the form `::metric name=value` | malformed lines are kept in the log and ignored, with a warning |

## Values

| Value | Rule |
|-------|------|
| `--name` | 1–100 characters of `A-Z a-z 0-9 _ . -` |
| `--min` / `--max` | for `number` and `integer` |

## Example

```console
$ trcli metric add run-6p0z accuracy=0.91 loss=0.23
Recorded 2 metrics for run-6p0z
```
