# Contract: `trcli result, figure, table`

**Spec**: [Experiments](../spec.md) — User Story 5; FR-038 to FR-044

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `res`, `fig`, `tbl`

## Synopsis

```text
trcli result add <run-ref> --name <name> --value <value> [--unit <text>] [--uncertainty <n>] [--description <text>]
trcli result promote <run-ref> <metric-name> [--unit <text>]
trcli result evidence <ref> <hypothesis-ref> (--supports | --contradicts | --neutral) [--remove]
trcli figure add <run-ref> --title <text> --file <path> [--caption <text>]
trcli table add <run-ref> --title <text> --file <path> [--caption <text>]
trcli result|figure|table origin <ref>
trcli figure|table verify <ref>
trcli draft report <draft-ref> <result|figure|table-ref>... [--remove]
trcli draft evidence <draft-ref>
trcli draft refresh <draft-ref> <result-ref> (--keep | --switch)
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `result add` | Records a named finding, permanently tied to its run | 2 when the name is already used in the run, offering to replace; 5 when the run did not succeed and `--yes` is not given |
| `result promote` | Creates a result from a recorded metric | 3 when the run has no such metric |
| `result evidence` | Links a result to a hypothesis with a stance | — |
| `figure add` / `table add` | Records the file's location and fingerprint; no copy is taken | 2 when the file does not exist |
| `… origin` | Run, parameters and condition, pipeline as run, dataset versions, methodology, software versions, environment | — |
| `… verify` | Checks the file against its fingerprint | 6 `check_failed` when changed or missing |
| `draft evidence` | What the draft reports, each with its run; flags results a newer run has replaced | 6 when any is flagged |
| `draft refresh` | Keeps the reported result or switches to the newer one | — |

## Values

| Value | Rule |
|-------|------|
| `--name` | 1–100 characters of `A-Z a-z 0-9 _ . -`; the same name across runs identifies "the same result" |
| `--uncertainty` | a number ≥ 0; only with a numeric value |
| list filters | `--run`, `--experiment`, `--name`, `--hypothesis` |

## Example

```console
$ trcli result origin res-8u2m
res-8u2m  accuracy = 0.91 ± 0.01
  run          run-6p0z (run 3 of exp-2b6r "Baseline"), succeeded 2026-10-08
  parameters   lr=0.01 · condition: treated, replicate 2
  dataset      dat-3n8x "Train set" version 2
  method       mth-8h4w "5-fold cross-validation"
  software     sw-1k5j "trainer" @ 3f9c2ab
  environment  env-0q7t (linux, x86_64)
```
