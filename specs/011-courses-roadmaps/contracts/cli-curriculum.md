# Contract: `trcli programme`, `trcli curriculum`

**Spec**: [Courses, Curricula, and Roadmaps](../spec.md) — User Stories 2 and 3; FR-011 to FR-029

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `prg` (programme), `cur` (curriculum) — a component is named by its code

`trcli program` is accepted for `trcli programme`.

## Synopsis

```text
trcli programme add --name <text> --institution <text> --level <level> --enrolled <date>
                    [--expected-months <n>] [--max-months <n>] [--project <ref>]
trcli programme status <ref> <enrolled|on_leave|completed|withdrawn>
trcli programme use <ref>

trcli curriculum add <programme-ref> --name <text> --year <year> [--follow]
trcli curriculum follow <ref>
trcli curriculum component add <ref> --code <text> --title <text> --credits <n> [--hours <n>]
                               [--category <category>] [--term <n>] [--area <text>]
                               [--requires <code>]... [--with <code>]... [--offered <term-number>]...
trcli curriculum component edit <ref> <code> [fields] | rm <ref> <code>
trcli curriculum group add <ref> --name <text> (--credits <n> | --count <n>) <code>...
trcli curriculum rule set <ref> [--total <n>] [--category <category>=<n>]... [--area <text>=<n>]...
trcli curriculum matrix [<ref>]
trcli curriculum import <ref> <file> [--map <column>=<detail>]... [--dry-run]
trcli curriculum export <ref> --to <file>
trcli curriculum switch <programme-ref> <curriculum-ref>

trcli curriculum equate <course-ref> <code> [--credits <n>] [--granted-by <text>]
trcli curriculum exempt <code> --because <text>
trcli curriculum count <course-ref> (--free | --not-counted)
trcli curriculum progress [<ref>]
trcli curriculum next [<ref>]
trcli curriculum why <code>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply to `programme` and
`curriculum` and are not repeated. Without `<ref>`, the followed curriculum of the
programme in use is meant.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `programme add` | Records a programme at status `enrolled`; shows the expected and the latest date of completion | 2 when the maximum is shorter than the expected time, or the enrolment date is invalid |
| `programme use` | Sets the programme later commands apply to when several exist | 3 when not found |
| `curriculum add` | Creates a curriculum; with `--follow` it becomes the one followed | 2 when the name exists for the programme |
| `curriculum component add` | Adds a component | 2 when the code is reused, credits are negative, the suggested term is below 1, a required code does not exist, or requirements form a loop (naming the components) |
| `curriculum group add` | Defines components from which a number of credits or of components must be completed | 2 when the number exceeds what the group can give |
| `curriculum rule set` | Sets total credits and minimums per category and area | 2 on negative values |
| `curriculum matrix` | The grid: components in columns by suggested term with code, credits, and category; prerequisites marked; groups as one requirement; totals per term | — |
| `curriculum import` | Brings in components from a table; reports each invalid row with its reason; remembers the column mapping for that source. `--dry-run` stores nothing | 2 when the file cannot be read as a table or every row is invalid; 5 when columns cannot be matched and it cannot ask |
| `curriculum switch` | Moves to another curriculum of the programme; shows which completed components carry over and which no longer count | 5 `confirmation_required` |
| `curriculum equate` | Counts a course for a component, with the credits it is worth and who granted it | 5 when the course already counts for a component, or the component already has a course |
| `curriculum exempt` | Records an exemption; the component counts as completed without a grade | 2 without `--because` |
| `curriculum progress` | Each component as completed, in progress, planned, or to do, with course, term, and grade; credits earned, in progress, and remaining in total and per category, area, and group; each rule met or not; pace against the suggested terms | 0 with transcript totals and a message when no curriculum is recorded |
| `curriculum next` | Components not completed whose prerequisites are all completed, mandatory first | 0 with the blocking chain when nothing is available |
| `curriculum why` | The prerequisites still missing for a component | 3 on unknown code |

## Values

| Value | Rule |
|-------|------|
| `<level>` | `undergraduate`, `specialization`, `masters`, `doctoral`, `postdoctoral`, `other` |
| `<category>` | `mandatory` (default), `elective`, `optional`, `complementary` |
| `--code` | 1–30 characters, unique within the curriculum, compared ignoring case and spaces |
| `--term` | the suggested term, counted from 1 |
| `--offered` | term numbers within a year in which the component is offered (for example `1` for first-term only) |
| `--map` | `<column heading or number>=<detail>`, details being `code`, `title`, `credits`, `hours`, `category`, `term`, `area`, `requires` |
| automatic counting | a course with the same code at the programme's institution counts for the component without `equate` |

## Example

```console
$ trcli curriculum matrix
Master's in Computer Science · curriculum 2024 · 24 credits required

TERM 1                    TERM 2                      TERM 3                 TERM 4
INF5001 Methods     4 M ✔ INF5020 Statistics   4 M ◐  INF5090 Seminar  2 M   INF5099 Dissertation 0 M
INF5012 ML          4 M ✔ INF5033 Writing      2 M ◐  ▸ Electives A (8 cr)
                          INF5041 Deep Learning 4 E     requires INF5012
12 cr                     10 cr                       10 cr                  0 cr

✔ completed · ◐ in progress · M mandatory · E elective

$ trcli curriculum progress
Credits   earned 8 · in progress 6 · remaining 10 of 24
  mandatory   8 / 14      elective   0 / 8  (group "Electives A": 0 / 8)
Rules     total ✘ · mandatory ✘ · electives ✘
Pace      on the suggested pace (term 2 of 4)

$ trcli curriculum why INF5041
INF5041 Deep Learning needs INF5012 Machine Learning ✔ — nothing is missing; you can take it.
```
