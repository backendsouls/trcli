# Contract: `trcli review`

**Spec**: [Literature, References, and Bibliography](../spec.md) — User Story 6; FR-040 to FR-048

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `rev`

## Synopsis

```text
trcli review add --title <text> --question <text> [--kind <narrative|systematic|scoping|mapping>] [--scope <text>]
trcli review criterion add <ref> (--include | --exclude) --label <text> <text>
trcli review criterion list <ref> | rm <ref> <label>
trcli review search add <ref> --query <text> --source <text> --date <date> --results <n> [--limits <text>]
trcli review search list <ref> | rm <ref> <n>
trcli review candidate add <ref> <reference-ref>... [--from-search <n>]
trcli review candidate list <ref> [--stage <abstract|full-text>] [--decision <unscreened|included|excluded>]
trcli review screen <ref> <reference-ref> --stage <abstract|full-text> (--include | --exclude --reason <label|text>)
trcli review screen <ref> --stage <abstract|full-text> --next
trcli review flow <ref>
trcli review complete <ref> | reopen <ref>
trcli review export <ref> --to <file>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `review add` | Creates a review at status `planning` | 2 without `--question` |
| `review search add` | Records a search as it was carried out | 2 when the date is in the future or `--results` is negative |
| `review candidate add` | Attaches candidates; duplicates within the review are kept once and counted as removed | — |
| `review screen` | Records a decision at a stage | 2 when `--exclude` has no `--reason`; 2 for `full-text` on a candidate not included at `abstract` |
| `review screen --next` | Shows the next unscreened candidate and asks; repeats until none remain or stopped | 5 when it cannot ask |
| `review flow` | Found per source, duplicates removed, screened and excluded per stage by reason, included | 0, stating any inconsistency between counts |
| `review complete` | Warns of unscreened candidates; the review can no longer change | 5 `confirmation_required` when candidates are unscreened |
| (any change to a completed review) | Refused; offers `review reopen` | 5 |

## Values

| Value | Rule |
|-------|------|
| `--label` | 1–50 characters; offered as a reason when screening |
| `--results` | whole number ≥ 0 |

## Example

```console
$ trcli review flow rev-41bc
Found            412   Scopus 250 · Web of Science 162
Duplicates        58   removed
Abstract         354   screened · 301 excluded (off-topic 220, not empirical 81)
Full text         53   screened ·  21 excluded (no control 14, unavailable 7)
Included          32
```
