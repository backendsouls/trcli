# Contract: `trcli step`

**Spec**: [Experiments](../spec.md) — User Story 2; FR-012 to FR-016

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

## Synopsis

```text
trcli step add <experiment-ref> --key <key> --name <text> --run <program> [--arg <text>]...
               [--shell] [--cwd <dir>] [--after <key>]... [--output <path>]...
trcli step add <experiment-ref> --key <key> --name <text> --manual --instructions <text>
               [--after <key>]... [--performer <staff-ref>] [--output <path>]...
trcli step list <experiment-ref>
trcli step edit <experiment-ref> <key> [fields]
trcli step rm <experiment-ref> <key>
trcli step copy <from-experiment-ref> <to-experiment-ref>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `step add --run` | Adds an automated step: a program and its arguments | 2 when the program is empty |
| `step add --manual` | Adds a manual step with instructions for a person | 2 without `--instructions` |
| `step list` | Steps in the order they will be carried out, with kind and dependencies | — |
| (any add or edit) | The pipeline is checked as a whole | 2 naming the steps when dependencies form a loop or name an unknown step, or when a key is reused |
| `step rm` | Removes a step | 5 `blocked_by_dependents` listing steps that depend on it |
| `step copy` | Copies a pipeline into an experiment that has none | 2 when the target already has steps |
| (completed or abandoned experiment) | Pipeline changes are refused | 5 |

## Values

| Value | Rule |
|-------|------|
| `--key` | 1–50 characters of `a-z 0-9 - _`, unique in the pipeline |
| `--shell` | runs through the platform's shell; without it the program is run directly, the same on every system |
| `--output` | a file the step must produce; missing at the end makes the step fail |

## Example

```console
$ trcli step add exp-2b6r --key train --name "Train" --run python --arg train.py --after label --after train
error: 1 value is invalid
  --after train   step "train" cannot depend on itself (loop: train → train)
Nothing was changed.
```
