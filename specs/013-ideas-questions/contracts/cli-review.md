# Contract: `trcli review-ideas`

**Spec**: [Ideas, Topics, Questions, and Hypotheses](../spec.md) — User Story 5; FR-043 to FR-050

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

The command is named `review-ideas` because `trcli review` already means a literature
review (`specs/005-literature`). `trcli revisit` is accepted as a shorter name.

## Synopsis

```text
trcli review-ideas [--kind <thought|idea|question|hypothesis>]... [--topic <topic>] [--limit <n>]
trcli review-ideas start [--kind <kind>]... [--topic <topic>] [--limit <n>]
trcli review-ideas ok <ref>...
trcli review-ideas later <ref>... (--until <date> | --for <span>)
trcli review-ideas let-go <ref>... --because <text>
trcli review-ideas remind <ref> --on <date>
trcli review-ideas periods [--ideas <span>] [--questions <span>] [--hypotheses <span>]
```

## What is due for review

| Kind | Due when |
|------|----------|
| Parked thought | its date has come (it is also back in the inbox) |
| Idea in state `raw`, `developing`, or `ready` | not looked at for longer than the period for ideas |
| Open research question | no activity for longer than the period for questions |
| Untested hypothesis | older than the period for hypotheses |
| Anything | the date given with `remind` has come |

"Looked at" means viewed with `show`, changed, linked, or answered in a review. Activity on
a question also includes a linked record being added and a hypothesis changing.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `review-ideas` | Lists what is due, most important and oldest first, with when each was last looked at and how often it has been put off | 6 `check_failed` when anything is due; 0 with the date the next item will be due when nothing is |
| `review-ideas start` | Presents due items one at a time with what has happened around each since; for each: still relevant, later, act, let go, skip, or stop. Can be stopped at any time with the rest still due | 5 when it cannot ask a person — use the direct commands |
| `review-ideas ok` | Marks items still relevant: they are not due again until their period has passed | 3 when not found |
| `review-ideas later` | Puts items off until a date; counted and shown at the next review | 2 on a date in the past |
| `review-ideas let-go` | Drops ideas and abandons questions, with a reason; they are kept among those let go | 2 without `--because`; 2 for a hypothesis (change its question or status instead) |
| `review-ideas remind` | Shows an item again on a chosen date whatever the periods; it appears in `trcli due` | 2 on a date in the past |
| `review-ideas periods` | Shows or sets the periods | 2 on a negative span |

## Values

| Value | Rule |
|-------|------|
| periods | defaults: ideas `3m`, questions `2m`, hypotheses `6m`; `0` means never due by time alone |
| settings | `review.ideas`, `review.questions`, `review.hypotheses` |
| acting on an item | any command that changes or links it counts as having reviewed it |
| a hypothesis whose evidence was replaced | is due at once, flagged "evidence changed" |
| in `trcli status` and activity reports | "Review: 6 due (2 ideas, 3 questions, 1 hypothesis)" |
| both parked and reminded | the earlier date wins |

## Example

```console
$ trcli review-ideas
6 due for review
  rq-6t1b   question    How much label noise can pre-training absorb?   quiet for 71 days
  hyp-8d4f  hypothesis  Effect disappears with more data                 untested for 7 months
  ida-2f7h  idea        Use disagreement as a signal, not as error      not seen for 4 months · put off twice
  tht-5p1a  thought     "compare with the 2019 baseline"                 parked until 2026-10-01
  … 2 more
$ echo $?
6

$ trcli review-ideas ok rq-6t1b
rq-6t1b marked still relevant · next review after 2026-12-08

$ trcli review-ideas let-go ida-2f7h --because "covered by Lima's project"
Dropped ida-2f7h (kept; revive with: trcli idea revive ida-2f7h)
```
