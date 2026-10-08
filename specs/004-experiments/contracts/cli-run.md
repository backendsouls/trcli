# Contract: `trcli run, sweep`

**Spec**: [Experiments](../spec.md) — User Stories 3 and 4; FR-017 to FR-037

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `run`, `swp`

## Synopsis

```text
trcli run start <experiment-ref> [--param <name>=<value>]... [--condition <name>]
trcli run confirm <run-ref> [--notes <text>]
trcli run fail <run-ref> --reason <text>
trcli run resume <run-ref>
trcli run cancel <run-ref>
trcli run list [--experiment <ref>] [--status <status>] [--condition <name>] [--sweep <ref>] [--from <date>] [--to <date>]
trcli run show <run-ref> [--log <step-key>]
trcli run compare <run-ref> <run-ref>... [--sort <column>]
trcli run best <experiment-ref|sweep-ref> --metric <name> (--max | --min) [--mark [--note <text>]]
trcli run summary <experiment-ref> [--metric <name>]...
trcli run rm <run-ref>
trcli sweep start <experiment-ref> --param <name>=<v1>,<v2>,... ...
trcli sweep show <sweep-ref>
trcli sweep retry <sweep-ref>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `run start` | Carries out automated steps in order, streaming their output. On a manual step it saves the run as `paused`, prints the instructions and the command to continue, and returns | 2 on an empty pipeline, undeclared parameter, or value not allowed; 7 `step_failed`; 5 on a completed or abandoned experiment; 130 when interrupted |
| `run confirm` | Marks the waiting manual step done and continues the run | 2 when the run is not paused, stating its actual state |
| `run fail` | Marks the waiting manual step failed; the run stops as `failed` | 2 without `--reason` |
| `run resume` | Continues a `failed` or `interrupted` run from the first step that did not succeed | 2 when the run is `succeeded`, `cancelled`, or `paused` |
| `run cancel` | Cancels a `paused`, `failed`, or `interrupted` run for good | 2 in any other state |
| `run show` | Every step's status, times, outputs, notes; what was in effect at start | — |
| `run compare` | Parameters, metrics, status, condition, and software versions side by side; differing parameters are highlighted | 2 with fewer than two runs |
| `run best` | Names the best run by a metric; `--mark` records it; later says when a newer run beats the mark | 3 when no run has the metric |
| `run summary` | Per condition: number of replicates, and mean and spread per metric | — |
| `run rm` | Removes a run and its step records | 5 `blocked_by_dependents` when a result, figure, or table depends on it |
| `sweep start` | Shows how many runs will be created, asks, then creates one run per combination | 5 `confirmation_required`; always asked above 50 runs |
| `sweep retry` | Reruns only the failed runs of a sweep | 0 with a message when none failed |

## Values

| Value | Rule |
|-------|------|
| `<status>` | `running`, `paused`, `succeeded`, `failed`, `interrupted`, `cancelled` |
| `--param` | must be declared with `trcli param add`; defaults are recorded for those not given |
| `--condition` | a condition of the experiment's design; the run is numbered as its next replicate |
| exit code of a paused run | 0 — pausing at a manual step is success |

## Example

```console
$ trcli run start exp-2b6r --param lr=0.01
run-6p0z  exp-2b6r "Baseline"  run 3
✔ prepare   0.4s
⏸ label     manual — waiting for you
    Label 50 samples in data/unlabelled and save them to data/labelled.
  When done: trcli run confirm run-6p0z --notes "..."
$ echo $?
0
```
