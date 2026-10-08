# Contract: `trcli roadmap` (learning roadmap)

**Spec**: [Courses, Curricula, and Roadmaps](../spec.md) — User Story 6; FR-050 to FR-057

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `rmp` — a stage is named by its number, an item by `<stage>.<item>` (`2.3`)

A learning roadmap is the researcher's own plan toward a goal. The requirements of a
programme are a different thing: see [cli-requirement.md](./cli-requirement.md).

## Synopsis

```text
trcli roadmap add --title <text> --goal <text>
trcli roadmap status <ref> <active|paused|completed|abandoned> [--note <text>]
trcli roadmap stage add <ref> <title> [--target <date>] [--at <position>]
trcli roadmap stage edit <ref> <stage> [--title <text>] [--target <date>] | move <ref> <stage> --to <position> | rm <ref> <stage>
trcli roadmap item add <ref> <stage> (--course <ref> | --reference <ref> | --task <ref>
                                      | --skill <text> | --free <text> [--url <address>])
                                      [--effort <text>] [--after <item>]... [--notes <text>]
trcli roadmap item edit <ref> <item> [fields] | move <ref> <item> --to <stage>[.<position>] | rm <ref> <item>
trcli roadmap item start <ref> <item> | done <ref> <item> [--took <text>] [--learned <text>] | skip <ref> <item>
trcli roadmap item level <ref> <item> --before <level> --after <level>
trcli roadmap view <ref>
trcli roadmap next [<ref>]
trcli roadmap export <ref> --to <file>
trcli roadmap import <file>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `roadmap add` | Creates a roadmap at status `active` | 2 without `--goal` |
| `roadmap stage add` | Adds a stage in order, with an optional target date | 2 on an empty title or an invalid date |
| `roadmap item add` | Adds an item of one kind. An item linked to a course, reference, or task takes its status from that record | 3 when the linked record is not found; 2 when `--after` forms a loop or names an unknown item |
| `roadmap item done` / `start` / `skip` | Sets an item's status, with the time it took and what was learned | 2 for an item whose status follows a linked record (change the record instead) |
| `roadmap item level` | Records the researcher's level in a skill before and after | 2 for an item that is not a skill |
| `roadmap view` | Each stage with its items, status, and how many are done; blocked items name what blocks them; stages past their target are marked; overall progress | — |
| `roadmap next` | Items that can be worked on now, across the roadmap or all active roadmaps | 0 with a message when everything is done |
| (all items done or skipped) | Offers to mark the roadmap `completed` with a closing note | — |
| `roadmap export` | Writes stages, items, and dependencies, without personal progress | 5 when the file exists and `--yes` is not given |
| `roadmap import` | Creates a new roadmap, not started, saying where it came from; lists references it names that are not in the library and offers to add them | 2 when the file is damaged or not a roadmap; nothing is created |
| (a linked record is deleted) | The item remains as a free item with the title it had | — |

## Values

| Value | Rule |
|-------|------|
| item status | `to_do`, `in_progress`, `done`, `skipped`; `blocked` is shown while an item it depends on is neither done nor skipped |
| linked items | course → done when `completed`; reference → done when `read`; task → done when `done` |
| `<level>` | `none`, `basic`, `working`, `proficient` (self-assessed) |
| `--effort`, `--took` | free text such as `10h` or `3 weeks`; recorded, not computed with |
| list filters | `roadmap list --status <status>`; each roadmap is shown with its goal, progress, and next item |

## Example

```console
$ trcli roadmap view rmp-6k2n
Bayesian analysis for my experiments · active · 5 of 9 items done (56%)
Goal: run and interpret Bayesian models for the thesis studies

1  Foundations                         target 2026-11-30      3 / 3 ✔
   1.1 ✔ course     INF5020 Statistics for Experiments
   1.2 ✔ reference  ref-3h7q Statistical Rethinking, ch. 1–4
   1.3 ✔ skill      Probability basics                 none → working

2  Modelling                           target 2027-01-31      2 / 4
   2.1 ✔ reference  ref-3h7q Statistical Rethinking, ch. 5–9
   2.2 ✔ free       Online course: hierarchical models
   2.3 ◐ skill      Writing models in a modelling language
   2.4 ○ task       tsk-8m1c Reanalyse Study A          blocked by 2.3

3  Practice                            target 2027-03-31      0 / 2
   3.1 ○ task       tsk-9n4d Preregister the analysis   blocked by 2.4
   3.2 ○ free       Present at the group seminar

Next: 2.3 Writing models in a modelling language
```
