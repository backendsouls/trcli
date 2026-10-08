# Contract: `trcli method`

**Spec**: [Conventions, Methodologies, and the Implicit Side of Research](../spec.md) — User Story 2; FR-011 to FR-017

**Conventions of the CLI itself**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `mth` — a version is written `mth-…@<n>`; without `@<n>` the latest is meant

`trcli methodology` is accepted for `trcli method`.

## Synopsis

```text
trcli method add --name <text> --purpose <text> [--description <text>] [--for <kind-of-work>]
                 [--origin <origin>] [--source <text>] [--reference <ref>]...
                 [--use-when <text>] [--not-when <text>] [--needs <text>] [--produces <text>]
                 [--pitfall <text>]... [--limitation <text>]...
trcli method step add <ref> <text> [--check <text>] [--notes <text>] [--at <position>]
trcli method step edit <ref> <n> [fields] | move <ref> <n> --to <position> | rm <ref> <n>
trcli method version add <ref> --note <text>
trcli method version list <ref>
trcli method diff <ref>@<n> <ref>@<n>
trcli method adapt <ref> --name <text> [--origin <origin>]
trcli method differs <ref> <text> --because <text> [--step <n>]
trcli method lineage <ref>
trcli method uses <ref>
trcli method retire <ref> --because <text> [--replaced-by <ref>]
trcli method to-pipeline <ref> <experiment-ref>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `method add` | Records a methodology at version 1 | 2 on a missing name or purpose, a name already used, or an unknown origin |
| `method step add` | Adds a step to the procedure, with an optional check that it was done right | 2 on empty text |
| `method version add` | Records the methodology as it now stands as the next version. Work that used an earlier version keeps showing that version | 2 without `--note`; 0 with a notice when nothing changed |
| `method diff` | Two versions compared step by step | 3 when a version is not found |
| `method adapt` | Creates a new methodology copied from this one and recorded as adapted from it | 2 when the new name is in use |
| `method differs` | Records one difference of an adapted methodology from its original, with the reason | 2 when the methodology was not adapted from another; 2 without `--because` |
| `method lineage` | The chain back to the original, with the differences introduced at each adaptation | — |
| `method uses` | Experiments, literature reviews, models, and manuscripts that rely on it, by version | — |
| `method retire` | Retires a methodology; it stays viewable and linked from past work | 2 without `--because` |
| `method to-pipeline` | Creates one pipeline step per methodology step in an experiment that has none, each to be marked automated or manual | 2 when the experiment already has steps |
| `method rm` | Shared verb | 5 `blocked_by_dependents` listing what uses it — retire it instead |
| (linking) | `trcli experiment edit --method`, `trcli review edit --method`, and `trcli model edit --method` record the version in effect; each run records it too | — |

## Values

| Value | Rule |
|-------|------|
| `--name` | 1–200 characters, unique in the workspace |
| `--for` | the kind of work: `experiment`, `analysis`, `data-collection`, `review`, `simulation`, `writing`, or other text |
| `<origin>` | `own` (default), `lab`, `institution`, `community`, `venue`, `funder`, `literature`; `literature` expects at least one `--reference` and warns without |
| `--pitfall`, `--limitation` | repeatable; each up to 2,000 characters |
| adaptation | must not form a loop; an adapted methodology with no recorded difference is listed by `trcli convention incomplete` |
| list filters | `--origin`, `--for`, `--status <current\|retired>`, `--adapted`, `--unused`, `--search` |

## Example

```console
$ trcli method lineage mth-8h4w
mth-8h4w  Stratified 5-fold cross-validation (lab)            version 2
  adapted from  mth-2c6k  k-fold cross-validation (literature · ref-5p8c)
    differs  folds are stratified by speaker            because recordings of one speaker are not independent
    differs  step 4: test set held out before folding   because the lab reports a single final test score

$ trcli method uses mth-8h4w
@1  exp-2b6r Baseline (runs 1–2)
@2  exp-2b6r Baseline (run 3) · exp-7m2c Queue model validation · ms-5t1q Our Paper
```
