# Contract: `trcli model`

**Spec**: [Simulations](../spec.md) — User Stories 1 and 5; FR-001 to FR-008, FR-038 to FR-044

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `mdl` — a version is written `mdl-…@<n>`; without `@<n>` the latest is meant

## Synopsis

```text
trcli model add --name <text> --purpose <text> [--kind <kind>] [--description <text>]
                [--time-unit <text>] [--stochastic | --deterministic] [--simulator <software-ref>]
trcli model input add <ref> --name <name> --kind <number|integer|yes-no|choice|text> [--unit <text>]
                      [--default <value>] [--min <n>] [--max <n>] [--choices <a,b,c>] [--description <text>]
trcli model input edit <ref> <name> [fields] | rm <ref> <name>
trcli model output add <ref> --name <name> (--value | --series) [--unit <text>] [--description <text>]
trcli model output edit <ref> <name> [fields] | rm <ref> <name>
trcli model assume <ref> <statement> [--because <text>] [--reference <ref>]...
trcli model assumption list <ref> | rm <ref> <n>
trcli model version add <ref> --note <text>
trcli model version list <ref>
trcli model diff <ref>@<n> <ref>@<n>
trcli experiment edit <experiment-ref> --model <ref>[@<n>]

trcli model check repeat <scenario-ref> [--seed <n>]
trcli model case add <ref> --name <text> --set <input>=<value>... --output <name> --expect <value>
                     --tolerance <n> --source <text>
trcli model case list <ref> | rm <ref> <name>
trcli model verify <ref> [--case <name>]...
trcli model validate <ref> --dataset <ref>[@<version>] --scenario <ref> --output <name>
                     --measure <text> --value <n> --threshold <text> (--acceptable | --not-acceptable) [--note <text>]
trcli model credibility <ref>[@<n>]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `model add` | Records a model at version 1 with the status `unverified` | 2 without `--purpose`, or on any invalid value |
| `model input add` | Declares an input | 2 when the name is reused, the default is outside what is allowed, or `choice` has no `--choices` |
| `model output add` | Declares an output as a single value or a series over simulated time | 2 when the name is reused |
| `model assume` | Records an assumption with its justification and supporting references | 3 when a reference is not found |
| `model version add` | Records the model as it now stands as the next version; its status starts `unverified` | 2 without `--note`; 0 with a notice when nothing changed since the last version |
| `model diff` | Inputs, outputs, and assumptions added, removed, and changed between two versions | 3 when a version is not found |
| `experiment edit --model` | Sets the model an experiment simulates. Offers to change the experiment's kind to `simulation` when it is another | 5 `confirmation_required` for the change of kind; 5 listing scenarios to resolve when moving to a version that removed an input they set |
| `model check repeat` | Runs a scenario twice with the same seed and compares every output | 6 `check_failed` naming the outputs that differ and any difference in simulator version |
| `model case add` | Defines a verification case: inputs with a known answer | 2 on a negative tolerance, an unknown input or output, or an expected value of the wrong kind |
| `model verify` | Runs the verification cases; each is reported passed or failed with obtained value, expected value, and difference | 6 `check_failed` when any case fails; 7 when a case could not be run |
| `model validate` | Records a comparison with observed data and the researcher's judgement | 2 without a judgement; 3 when the dataset or scenario is not found |
| `model credibility` | Status of a version, its assumptions, every check with outcome and date, and what is missing for the next status | — |
| `model rm` | Shared verb | 5 `blocked_by_dependents` when runs refer to it |

## Values

| Value | Rule |
|-------|------|
| `<kind>` | `discrete-event`, `agent-based`, `system-dynamics`, `equation-based`, `monte-carlo`, `other` (default) |
| `--name` (input, output) | 1–100 characters of `A-Z a-z 0-9 _ . -`, unique within the model |
| `--time-unit` | the unit of simulated time, for example `s`, `day`, `step`; default `step` |
| `--stochastic` / `--deterministic` | whether the model uses random numbers; default `--stochastic` |
| `--tolerance` | a number ≥ 0; a case passes when the obtained value is within it of the expected value |
| status | `unverified` → `verified` (repeatability check and all cases pass) → `validated` (verified and at least one validation judged acceptable); per version; lowered when a check fails |

## Example

```console
$ trcli model verify mdl-3q8e
mdl-3q8e@2 "Single-server queue": 2 of 3 cases passed
  ✔ low-load     mean_wait = 0.111   expected 0.111 ± 0.005
  ✔ no-arrivals  served    = 0       expected 0 ± 0
  ✘ high-load    mean_wait = 8.42    expected 9.000 ± 0.250   (off by 0.58)
Status of mdl-3q8e@2: unverified (was verified) — case "high-load" failed
$ echo $?
6
```
