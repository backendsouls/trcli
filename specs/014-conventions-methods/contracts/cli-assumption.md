# Contract: `trcli assumption`, `trcli decision`, `trcli lesson`

**Spec**: [Conventions, Methodologies, and the Implicit Side of Research](../spec.md) — User Story 4; FR-026 to FR-032

**Conventions of the CLI itself**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `asm` (assumption), `dec` (decision), `lsn` (lesson)

## Synopsis

```text
trcli assumption add <statement> [--under <work-ref>]... [--why <text>] [--if-false <text>]
                     [--check-by <text>] [--impact <low|medium|high>] [--origin <origin>]
trcli assumption under <ref> <work-ref>... [--remove]
trcli assumption check <ref> (--holds | --fails | --cannot-check) --evidence <text|record-ref>
trcli assumption affected <ref>
trcli assumption for <manuscript-ref | project-ref | experiment-ref>

trcli decision add <question> --chose <text> --because <text> [--option <text>]...
                   [--about <record-ref>]... [--made <date>] [--by <person>]... [--reopen-if <text>]
trcli decision replace <ref> --chose <text> --because <text> [--made <date>]
trcli decision revisit <ref> --on <date>
trcli decision for <record-ref>

trcli lesson add <what-happened> --learned <text> [--next-time <text>] [--about <record-ref>]...
trcli lesson to-convention <ref>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply to all three and are
not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `assumption add` | Records an assumption with the status `held`, under the work it concerns | 2 on an empty statement or unknown impact |
| `assumption under` | Links one assumption to further work; it remains one assumption | 3 when a record is not found |
| `assumption check` | Records that it was checked and holds, checked and does not hold, or cannot be checked, with the evidence and the date. With `--fails`, lists everything that rests on it | 2 without `--evidence` |
| `assumption affected` | The experiments, models, results, and manuscripts that rest on an assumption | — |
| `assumption for` | All assumptions under a manuscript, project, or experiment — including those of its experiments, models, and results — unchecked ones of highest impact first | 6 `check_failed` when any assumption is recorded as not holding |
| `decision add` | Records a choice: the question, the options considered, the one chosen, the reason, when, and by whom | 2 without `--chose` or `--because` |
| `decision replace` | Records a later decision that replaces this one; both are kept and linked | 2 without `--because` |
| `decision revisit` | Gives a decision a date to be looked at again; it appears in `trcli due` | 2 on a date in the past |
| `decision for` | The decisions that concern a record, in order of date | — |
| `lesson add` | Records what happened, what was learned, and what to do next time | 2 without `--learned` |
| `lesson to-convention` | Turns a lesson into a proposed convention, keeping the link | — |

## Values

| Value | Rule |
|-------|------|
| assumption status | `held` (default), `holds`, `fails`, `cannot-check`; every change is kept with its evidence |
| `--impact` | how much would change if it were false; default `medium`; used to order lists |
| `--made` | the date the decision was made, when recorded later; the recording date is kept too |
| `--option` | an option that was considered and not chosen; repeatable |
| model assumptions | assumptions of a simulation model (`specs/009-simulations`) appear in `assumption for` lists |
| list filters | `assumption list --status --impact --under <ref> --unchecked`; `decision list --about <ref> --from <date> --to <date> --replaced`; `lesson list --about <ref>` |

## Example

```console
$ trcli decision add "Which sessions do we exclude?" \
    --option "none" --option "shorter than 10 s" --chose "shorter than 30 s" \
    --because "Below 30 s the transcript has too few turns for the agreement measure to be defined" \
    --about exp-2b6r --about dat-3n8x --reopen-if "we change the agreement measure"
Recorded dec-6b3h for exp-2b6r, dat-3n8x

$ trcli assumption check asm-9k1d --fails --evidence res-4c7j
asm-9k1d "Annotators were independent" → checked, does not hold (2026-10-08)
Resting on it — look at these again:
  experiment  exp-2b6r  Baseline
  result      res-8u2m  accuracy = 0.91 ± 0.01
  manuscript  ms-5t1q   Our Paper (reports res-8u2m)
```
