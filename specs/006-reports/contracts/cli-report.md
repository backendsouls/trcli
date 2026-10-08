# Contract: `trcli report`

**Spec**: [Activity Reports](../spec.md) — User Stories 1 to 6; FR-001 to FR-044

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `rpt`

## Synopsis

```text
trcli report [<period>] [--previous | --date <date> | --number <n> --year <year>]
trcli report custom (--from <date> [--to <date>] | --last <span>)
trcli report run <definition-name> [any option below]

  options:  [--project <ref>]... | --all-projects     scope
            [--audience <personal|supervisor|funder>]
            [--detail <summary|standard|full>]
            [--section <name>]... [--no-section <name>]...
            [--tag <tag>]... [--responsible <staff-ref>]...
            [--compare] [--highlights <text>] [--blockers <text>] [--next <text>] [--comment <text>]
            [--save] [--to <file>] [--format <markdown|text|json>]

trcli report compare <period> <period>
trcli report series <period> --count <n> [--metric <name>]...
trcli report saved list [--period <period>] [--project <ref>] [--audience <audience>] [--from <date>] [--to <date>]
trcli report saved show <ref> [--revision <n>] | diff <ref> | narrate <ref> [--highlights …] | rm <ref>
trcli report define <name> --period <period> [options above] [--prompt <heading>]...
trcli report definitions | definition show <name> | definition rm <name> | default <name>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `report` | With no period: the default definition, or the weekly report of the current scope at standard detail | — |
| `report <period>` | The current period to now; `--previous` the last complete one; `--date` the one containing a date; `--number`/`--year` a named one | 2 on an unknown period, an impossible date, or a number outside the year |
| `report custom` | From a date to a date (both included), from a date to now, or the last span ending today | 2 when `--to` is before `--from`, the period is entirely in the future, or the span is not positive |
| (period before the workspace existed) | Limited to the workspace's first day, and says so | 0; 0 with a message when entirely before |
| `--audience` | Sets sections and tone; anything but `personal` withholds private and sensitive items and says how many | 2 on unknown audience, listing valid ones |
| `--compare` | Each summary count beside the preceding period's, with the difference; a partial period is compared with the same portion | — |
| `--save` | Keeps the report exactly as produced, with date, author, and whether partial | 0 with a notice when a saved report exists for the same period and scope |
| `--to <file>` | Exports instead of printing; the three formats carry the same content | 5 when the file exists and `--yes` is not given; nothing partial is left on failure |
| `report series` | One column per consecutive period, oldest first; each value equals that period's own report | 2 when `--count` is not positive |
| `report saved show` | Exactly as saved, marked as a saved copy | 3 when not found |
| `report saved narrate` | Changes the narrative; saves a new revision and keeps earlier ones | — |
| `report saved diff` | What differs between a saved report and a fresh one for the same period and scope | 6 when they differ |
| `report define` | Stores a named, reusable definition | 2 when the name is in use or a value is invalid |
| `report run` | Produces by definition name; explicit options override it once | 3 when the definition, or a project it names, no longer exists |
| (activity record fails its check) | The report is produced with a visible warning | 0 |

## Values

| Value | Rule |
|-------|------|
| `<period>` | `daily`, `weekly`, `monthly`, `bimonthly` (two months), `quarterly`, `semiannual` (also `semester`, `half-year`), `annual` |
| `<span>` | `<n>d`, `<n>w`, `<n>m` with n ≥ 1 |
| `--section` | `summary`, `literature`, `writing`, `inquiry`, `experiments`, `data`, `planning`, `other`, `attention`, `upcoming`, `highlights`, `blockers`, `next` |
| default `--detail` | `standard` up to one month; `summary` by month for longer periods |
| settings | `report.week_start` (default `monday`), `report.year_start_month` (default `1`), `report.default_detail`, `report.default_audience` |
| guarantees | producing a report changes nothing and never appears as activity; reports need no network and send nothing |

## Example

```console
$ trcli report weekly --project prj-1d4c
Weekly report · 2026-10-05 (Mon) → 2026-10-08 (Thu), partial · Doctorate · America/Sao_Paulo

Summary     6 references added · 2 read · 9 annotations · 3 runs (2 succeeded, 1 waiting)
            1 result · 4 tasks done · quiet: writing

Literature  + ref-7k3f  Attention Is All You Need                       Oct 6
            ✓ ref-2j5n  read · 5 annotations                            Oct 7
Experiments ✓ run-6p0z  exp-2b6r "Baseline" run 3 succeeded             Oct 8
            + res-8u2m  accuracy = 0.91 ± 0.01                          Oct 8
Planning    ✓ tsk-5w0h  Rerun baseline with new split                   Oct 8

Attention   1 overdue task · 1 run waiting for you (run-4k1a, step "label")
Next week   2 tasks · milestone mil-7c3k "Draft to supervisor" on Oct 12
```
