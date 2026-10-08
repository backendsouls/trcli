# Contract: `trcli course`, `trcli term`

**Spec**: [Courses, Curricula, and Roadmaps](../spec.md) — User Story 1; FR-001 to FR-010

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `crs`

## Synopsis

```text
trcli course add --title <text> (--taking | --teaching) [--code <text>] [--institution <text>]
                 [--term <term> | --from <date> --to <date>] [--credits <n>] [--hours <n>]
                 [--instructor <text|staff-ref>] [--schedule <text>] [--programme <ref>]
                 [--role <lecturer|assistant|intern>] [--students <n>] [--notes <text>]
trcli course status <ref> <status>
trcli course grade <ref> <grade>
trcli course assessment add <ref> --title <text> --date <date> [--weight <n>] [--result <text>]
trcli course assessment list <ref> | edit <ref> <n> [fields] | rm <ref> <n>
trcli course link <ref> <record-ref>... [--remove]
trcli course transcript [--programme <ref>] [--term <term>] [--to <file>]
trcli course teaching [--from <term>] [--to-term <term>]

trcli term define <institution> --per-year <n> [--names <a,b,...>]
trcli term set <institution> <term> --from <date> --to <date>
trcli term list [<institution>]
trcli grading set <institution> (--numeric <min>..<max> --pass <n> | --ordered <A,B,C,D,F> --pass <C> | --pass-fail)
trcli grading show [<institution>]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `course add` | Records a course at status `planned`. With the code of a component of the followed curriculum, it counts for that component | 2 on negative credits or hours, `--to` before `--from`, an unknown term, or neither `--taking` nor `--teaching` — all reported together |
| `course status` | Changes status; dated and kept | 2 on unknown status |
| `course grade` | Records the final grade and sets the status to `completed` or `failed` according to the grading scheme | 2 when the grade is outside the institution's scheme, or no scheme is set |
| `course assessment add` | Records an exam or assignment; a future date appears in `trcli due` | 2 on a weight outside 0–100 |
| `course transcript` | Courses taken, by term, with credits and grades; credits attempted and earned and the average per term and overall | 0 with a message when no course has been taken |
| `course teaching` | Courses taught, by term, with role, hours, and students | — |
| `term define` / `term set` | Defines an institution's terms: how many per year, their names, their dates | 2 when `--per-year` is not between 1 and 6, or dates overlap |
| `grading set` | Sets how an institution grades and what passes | 2 when the passing grade is not in the scheme |

## Values

| Value | Rule |
|-------|------|
| `<status>` | `planned`, `enrolled`, `in_progress`, `completed`, `failed`, `dropped`, `exempted` |
| `<term>` | `<year>.<n>` (for example `2026.2`), or a name defined with `term define` |
| `--credits`, `--hours` | numbers ≥ 0 |
| averages | weighted by credits; leave out pass/fail, exempted, dropped, and in-progress courses; never combine grading schemes |
| repeated course | every attempt stays in the transcript; only a passed one earns credits |
| list filters | `--term`, `--status`, `--institution`, `--programme`, `--taking` / `--teaching`, `--search` |

## Example

```console
$ trcli course transcript
2026.1                                       CREDITS  GRADE
  INF5001  Research Methods                       4     9.0
  INF5012  Machine Learning                       4     7.5
  INF5020  Statistics for Experiments             4     4.0   failed
                              attempted 12 · earned 8 · average 6.8
2026.2
  INF5020  Statistics for Experiments             4     —     in progress
  INF5033  Scientific Writing                     2     —     in progress

Overall     attempted 12 · earned 8 · in progress 6 · average 6.8
```
