# Contract: `trcli instrument, sample, material`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 14; FR-055, FR-056

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `ins`, `smp`, `mat`

## Synopsis

```text
trcli instrument add --name <text> [--identifier <text>] [--calibrated <date>] [--next-calibration <date>]
trcli instrument calibrate <ref> --date <date> [--next <date>] [--note <text>]
trcli sample add --name <text> [--origin <text>] [--collected <date>] [--stored <text>] [--parent <ref>]
trcli material add --name <text> [--supplier <text>] [--lot <text>] [--quantity <text>] [--expires <date>]
trcli run use <run-ref> <instrument|sample|material-ref>...
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `instrument calibrate` | Adds to the calibration history; the next date is shown in `trcli due` | — |
| `sample add --parent` | Records a sample derived from another | 2 when the parent would create a loop |
| `run use` | Records what a run used | 5 `confirmation_required` when an instrument is past calibration or a material past expiry; the acknowledgement is recorded |

## Values

| Value | Rule |
|-------|------|
| list filters | `instrument list --due`; `material list --expired`; `sample list --origin` |

## Example

```console
$ trcli run use run-6p0z ins-3k9v
warning: ins-3k9v "Spectrometer" was due for calibration on 2026-09-01
Record its use anyway? [y/N]
```
