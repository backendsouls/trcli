# Contract: `trcli sim`

**Spec**: [Simulations](../spec.md) — User Stories 3, 4, and 6; FR-018 to FR-037, FR-045 to FR-049

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `bat` (batch, sensitivity analysis, or ensemble). A replication is a run and keeps the `run` prefix and every `trcli run` command of `specs/004-experiments`.

## Synopsis

```text
trcli sim run <scenario>... [--replications <n>] [--more <n>] [--base-seed <n> | --seeds <n,n,...>]
                            [--independent-seeds]
trcli sim resume <batch-ref>
trcli sim retry <batch-ref>
trcli sim rerun <run-ref>
trcli sim batch list [--experiment <ref>] | show <batch-ref>

trcli sim output <run-ref> [--name <output>]
trcli sim summary <scenario> [--output <name>]... [--confidence <percent>] [--precision <output>=<n>]...
                             [--include-earlier] [--include-short]
trcli sim compare <scenario>... [--against <scenario>] [--output <name>]... [--confidence <percent>] [--unpaired]
trcli sim record <scenario> --output <name> [--as <result-name>] [--against <scenario>]
trcli sim export <scenario> --output <name> --to <file> [--per-replication | --mean]

trcli sim sensitivity start <experiment-ref> --input <name>[=<min>..<max>]... [--levels <n>] [--replications <n>]
trcli sim sensitivity show <batch-ref> --output <name>
trcli sim uncertain <model-ref> <input> (--uniform <min>..<max> | --normal <mean>,<spread> | --weighted <v:w,v:w,...>)
trcli sim uncertain <model-ref> --list | --clear <input>
trcli sim ensemble start <experiment-ref> --samples <n> [--sampling-seed <n>] [--replications <n>]
trcli sim ensemble show <batch-ref> --output <name> [--percentiles <p,p,...>]
```

## How a simulator is driven

For each replication the experiment's pipeline is carried out once. Before it starts, the
scenario's input values, horizon, warm-up, time step, seed, and replication number are made
available to its steps, and the steps report outputs and simulated time back. The exact
names and forms are fixed at planning time, are the same on every platform, and are
documented in the usage guide; a simulator is adapted to them once.

| Direction | What is exchanged |
|-----------|-------------------|
| To the simulator | every model input by name; `horizon`, `warm_up`, `time_step`; `seed`; `replication`; `scenario` |
| From the simulator | each declared output: a value, or the location of a series; optionally the simulated time reached, for progress |

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `sim run` | Runs the scenarios' replications as one batch. Each replication is a run recording scenario, values, replication number, seed, model version, and simulator version. Seeds derive from the batch's base seed; the same replication number gets the same seed in every scenario unless `--independent-seeds` | 2 when the experiment has no model or an empty pipeline, a seed is given twice or was already used for the scenario, or the model uses no random numbers and more than one replication is asked; 6 when scenarios await `scenario resolve`; 7 when replications failed (the others are kept); 130 when interrupted |
| `sim run --more` | Adds replications to scenarios that already have some; numbering continues and new seeds are used | as above |
| `sim resume` | Continues an interrupted batch; finished replications are not repeated, the rest keep their seeds | 2 when the batch is complete |
| `sim retry` | Reruns the failed replications of a batch with the same seeds | 0 with a message when none failed |
| `sim rerun` | Runs a past replication again with the same values, model version, and seed, linked to the original | 2 when the run is not a replication |
| `sim output` | A replication's outputs: values, and for a series the number of points, first and last time, minimum, maximum, mean, and final value after the warm-up. Reports declared outputs that are missing | — |
| `sim summary` | Per output across a scenario's replications: count, mean, spread, and an interval for the mean. Leaves out and counts failed, interrupted, short, non-finite, and earlier-definition replications. With `--precision`, says whether the interval is narrow enough and estimates how many more replications are needed | 6 `check_failed` when a stated precision is not met; 0 with spread and interval "not available" for a single replication |
| `sim compare` | Each output per scenario beside the baseline (or `--against`): difference, its interval, and whether it excludes zero. Pairs replications that share seeds unless `--unpaired`, and states the method used | 2 with fewer than two scenarios or no baseline |
| `sim record` | Records a summary, or with `--against` a comparison, as a result: the mean as value, the half-width of the interval as uncertainty, tied to every replication used | 2 with a single replication (no uncertainty can be stated) unless `--yes` |
| `sim export` | Writes a series for a scenario as a table: one column per replication, or mean and interval at each time | 5 when the file exists and `--yes` is not given |
| `sim sensitivity start` | Varies each named input alone across levels from the baseline; shows the number of runs and asks | 5 `confirmation_required`, always above the stated limit; 2 on fewer than two levels or an input with no range |
| `sim sensitivity show` | Per input: the output at the lowest and highest level, the swing, its rank, and whether it is distinguishable from the baseline's interval | 3 when the output is not declared |
| `sim uncertain` | States how uncertain a model input is | 2 on a range outside the model's, a negative spread, or negative weights |
| `sim ensemble start` | Draws the samples with a recorded seed, shows the number of runs, asks, and runs them | 5 `confirmation_required`; 2 on fewer than one sample or no uncertain inputs |
| `sim ensemble show` | Mean, spread, percentiles, minimum, and maximum of an output across samples | — |

## Values

| Value | Rule |
|-------|------|
| `<scenario>` | an `scn-…` handle or `<experiment-ref>/<name>`; `<experiment-ref>/*` means all its scenarios |
| `--base-seed`, `--seeds`, `--sampling-seed` | whole numbers ≥ 0; recorded with the batch |
| `--confidence` | a percentage strictly between 50 and 100; default 95 |
| `--levels` | whole number ≥ 2; default 5 |
| `--percentiles` | numbers between 0 and 100; default `5,25,50,75,95` |
| settings | `simulation.confidence` (default `95`), `simulation.confirm_above_runs` (default `50`), `simulation.pair_by_default` (default `true`) |
| credibility | every summary, comparison, and recorded result shows the model's status at the time of its runs (see [cli-model.md](./cli-model.md)) |

## Example

```console
$ trcli sim run exp-7m2c/baseline exp-7m2c/two-servers --base-seed 20261008
Batch bat-9w4r · 2 scenarios × 30 replications = 60 runs · model mdl-3q8e@2 (verified) · base seed 20261008
  baseline       30/30 ✔
  two-servers    30/30 ✔   (same seeds per replication as baseline)

$ trcli sim compare exp-7m2c/two-servers
Against baseline · 30 paired replications · 95% confidence · model mdl-3q8e@2 (verified)

OUTPUT       BASELINE          TWO-SERVERS       DIFFERENCE             EXCLUDES 0
mean_wait    4.12 ± 0.21 s     0.63 ± 0.04 s     −3.49 [−3.69, −3.29]   yes
utilization  0.80 ± 0.01       0.40 ± 0.01       −0.40 [−0.41, −0.39]   yes

$ trcli sim record exp-7m2c/two-servers --output mean_wait --against exp-7m2c/baseline --as wait_reduction
Recorded res-2y6g wait_reduction = −3.49 ± 0.20 s, from 60 replications of bat-9w4r
```
