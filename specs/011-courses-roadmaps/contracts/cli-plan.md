# Contract: `trcli plan`

**Spec**: [Courses, Curricula, and Roadmaps](../spec.md) — User Story 4; FR-030 to FR-037

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `pln` — a plan is usually named instead: `main`, `fast-track`

## Synopsis

```text
trcli plan add <name> [--curriculum <ref>] [--from <plan>]
trcli plan list
trcli plan show [<plan>]
trcli plan place <plan> <term> <code>...
trcli plan unplace <plan> <code>...
trcli plan load [--min <n>] [--max <n>]
trcli plan check [<plan>]
trcli plan propose [<plan>] [--save]
trcli plan compare <plan> <plan>
trcli plan follow <plan>
trcli plan enrol [<term>]
trcli plan rm <plan>
```

Without `<plan>`, the followed plan is meant.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `plan add` | Creates an empty plan for the followed curriculum, or a copy of another plan | 2 when the name is in use |
| `plan show` | Components term by term with the credits of each term, the term in which the curriculum would be completed, and whether that is within the expected and the maximum time | — |
| `plan place` | Places components in a future term | 2 for a completed component, a past term, or an unknown code |
| `plan load` | Sets the minimum and maximum credit load of a term | 2 when the minimum exceeds the maximum |
| `plan check` | Reports components placed before a prerequisite, corequisites in different terms, components placed where they are not offered, terms outside the credit load, mandatory components placed nowhere, and curriculum rules left unmet. Also shows components unplaced because a course was failed or dropped, with what they delay | 6 `check_failed` when anything is reported |
| `plan propose` | Spreads the remaining components over terms: the earliest term that respects prerequisites, offering, suggested terms, and load. Shown as a proposal; stored only with `--save` or on acceptance | 6 when no plan fits the maximum time — the shortest is shown |
| `plan compare` | Differences term by term and the two completion terms | 3 when a plan is not found |
| `plan follow` | Makes this the plan progress and `trcli due` use | — |
| `plan enrol` | Creates the courses of a term (the current one by default) from the followed plan, at status `enrolled` | 2 when the term has no placed components; 0 with a notice for components that already have a course |
| `plan rm` | Removes a plan | 5 `confirmation_required` for the followed plan |

## Values

| Value | Rule |
|-------|------|
| `<name>` | 1–40 characters of `a-z 0-9 -` |
| `<term>` | as in [cli-course.md](./cli-course.md); must not be in the past |
| settings | `plan.min_load`, `plan.max_load` (credits per term) |
| a proposal | is simple and explainable: it does not balance workload, avoid timetable clashes, or choose electives for the researcher beyond the cheapest way to meet a group |

## Example

```console
$ trcli plan check main
main: 2 problems
  2027.1  INF5041 Deep Learning is offered only in second terms
  2027.2  18 credits is above your maximum load of 12
Would complete in 2027.2 — within the expected time (24 months) ✔
$ echo $?
6

$ trcli plan propose main
Proposed (not saved):
  2027.1  INF5090 Seminar (2) · INF5055 Optimization (4) · INF5061 NLP (4)         10 cr
  2027.2  INF5041 Deep Learning (4) · INF5099 Dissertation (0)                      4 cr
Completes in 2027.2 · meets every rule
Save this as "main"? [y/N]
```
