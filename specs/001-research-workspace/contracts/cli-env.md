# Contract: `trcli env, repro`

**Spec**: [Research Workspace](../spec.md) — User Story 8; FR-043 to FR-047

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `env`

## Synopsis

```text
trcli env capture --name <text>
trcli env list | show <ref> | rm <ref>
trcli env diff <ref> <ref>
trcli repro check <run-ref>
trcli repro package <run-ref> --to <file>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `env capture` | Stores a snapshot of the machine, system, configured tools, and allowed variables | 2 when the name is empty |
| `env diff` | Item-by-item differences: added, removed, changed | — |
| `env rm` | Removes a snapshot | 5 `blocked_by_dependents` when a run refers to it |
| `repro check` | Compares a past run with the present: environment, dataset versions, methodology, pipeline, software | 6 `check_failed` when anything differs; 0 when reproducible |
| `repro package` | Writes one shareable file describing everything needed to repeat the run | 5 when `--to` exists and `--yes` is not given |

## Values

| Value | Rule |
|-------|------|
| `--name` | 1–200 characters |
| settings | `environment.tools` and `environment.variables` decide what is captured; secret-like variable names are refused |

## Example

```console
$ trcli repro check run-6p0z
run-6p0z is not reproducible: 1 difference
  dataset   dat-3n8x "Train set"   recorded version 1, current content differs
$ echo $?
6
```
