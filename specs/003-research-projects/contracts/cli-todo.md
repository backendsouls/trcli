# Contract: `trcli todo`

**Spec**: [Research Projects, Milestones, and Tasks](../spec.md) — User Story 7; FR-053 to FR-072

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

The to-do list is a view of the projects, milestones, and tasks of this specification. It
stores nothing of its own: `trcli todo done 12` and `trcli task done tsk-5w0h` are the same
change to the same record (see [cli-task.md](./cli-task.md), [cli-milestone.md](./cli-milestone.md)).

`trcli t` is accepted for `trcli todo`.

## Synopsis

```text
trcli todo [--all-projects | --project <ref>] [--timeline] [filters]
trcli todo "<text with marks>"
trcli todo add "<text with marks>"...
trcli todo done <n>...          trcli todo undo <n>...
trcli todo start <n>...         trcli todo cancel <n>...
trcli todo star <n>...          trcli todo unstar <n>...
trcli todo priority <n>... <low|normal|high|urgent>
trcli todo due <n>... <when> | --none
trcli todo move <n>... (--to <milestone> | --no-milestone | --project <ref> | --under <n> | --top)
trcli todo edit <n> "<new title>"
trcli todo rm <n>...
trcli todo show <n>
trcli todo find <words>...
trcli todo legend
```

Filters: `--starred`, `--pending`, `--doing`, `--blocked`, `--overdue`, `--milestone <ref>`,
`--mine`, `--person <ref>`, `--priority <level>`, `--tag <tag>`, `--done`, `--everything`.

## The list

```console
$ trcli todo --all-projects

  Doctorate  [5/11]

    ▸ Qualifying exam · 2028-02-29 · in 144 days · required  [2/5]
       12 ✔ Collect the three reference papers
       14 ✔ Outline the exam document
       15 … Rerun baseline with new split                        due in 2 days  ★  @ana
       16 ☐ Write the related-work chapter                      !! due tomorrow
            17 ☐ Summarize the 2024 survey
            18 ✔ List what is missing from chapter 2
       19 ☐ Book the committee                                   ⊘ blocked by 16

    ▸ Paper submitted · 2027-06-30 · overdue 4 days  [1/3]
       21 ✔ Choose the venue
       22 ☐ Final read-through                                   ! overdue 6 days  ★
       23 ☐ Upload the camera-ready version

    ▸ No milestone  [2/3]
       25 ☐ Renew the library card                               ↻ monthly
       + 2 done this week

  Capstone co-supervision  [3/3]  ✔ all done

    ▸ Draft to supervisor · 2026-10-12  ✔ reached 2026-10-09

  57% of all tasks complete
  8 done · 1 in progress · 4 pending · 1 blocked            (3 older done tasks hidden)
```

What each part means:

| Element | Shows |
|---------|-------|
| Board | a project: its title and tasks done out of total |
| Section `▸` | a milestone: title, target date, time remaining or overdue, `required`, and its own count; `No milestone` gathers the rest |
| Number | the task's short number: stable while the task is open, never reused |
| Checkbox | `☐` to do · `…` in progress · `✔` done · `✖` cancelled |
| Indentation | sub-tasks under their task, to any depth |
| `★` | starred |
| `!` `!!` | priority high, urgent (normal and low show nothing) |
| `due …` / `overdue …` | the due date as time from now |
| `⊘ blocked by n` | waits for another task |
| `↻` | repeats |
| `@name` | the person responsible, when it is not the researcher |
| `✎` | has notes |
| Summary | percentage of listed tasks done; counts done, in progress, pending, blocked; how many are hidden |

Color adds to these and never replaces them: one color each for done, in progress, overdue,
and starred; finished work is dimmed.

## Plain form

Where the terminal cannot show the symbols or color, or `--plain` is given, or
`output.symbols` is `ascii`:

```text
  Doctorate  [5/11]

    > Qualifying exam - 2028-02-29 - in 144 days - required  [2/5]
       12 [x] Collect the three reference papers
       15 [~] Rerun baseline with new split                      due in 2 days  (*)  @ana
       16 [ ] Write the related-work chapter                    !! due tomorrow
            17 [ ] Summarize the 2024 survey
       19 [ ] Book the committee                                 (blocked by 16)
```

`[ ]` to do · `[~]` in progress · `[x]` done · `[-]` cancelled · `(*)` starred.

## Adding with marks

```console
$ trcli todo "Send the abstract to Carla +paper-submitted due:fri p:high *"
Added 26 ☐ Send the abstract to Carla  (Doctorate ▸ Paper submitted · due Friday · high · ★)
```

| Mark | Sets | Example |
|------|------|---------|
| `@<project>` | the project (otherwise the current one) | `@doctorate` |
| `+<milestone>` | the milestone, by a unique start of its title, with hyphens for spaces | `+qualifying` |
| `^<n>` | the parent task | `^16` |
| `p:<level>` | priority: `low`, `normal`, `high`, `urgent` or `1`–`4` | `p:high` |
| `due:<when>` | due date: a date, `today`, `tomorrow`, a weekday, or `+<n>d` / `+<n>w` | `due:+3d` |
| `*` (alone) | a star | `*` |
| `~<person>` | the person responsible | `~ana` |
| `#<tag>` | a tag | `#writing` |

Marks may appear anywhere and are removed from the title. Something that looks like a mark
but is not one — `carla@example.org`, `p:` followed by another word, a `#` inside a word —
stays in the title. A mark that names nothing (`+nosuchmilestone`) fails with exit code 2
and adds nothing.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `todo` | The board view: the current project, or all active projects when there is none or `--all-projects` is given | 0 with a friendly line and how to add a task when there is nothing to show |
| `todo --timeline` | The same tasks and milestones grouped by date: overdue, today, tomorrow, this week, later, no date; each naming its project | — |
| `todo "<text>"` / `todo add` | Adds a task at status `to do` and confirms in one line with its number | 2 on empty text or a mark that names nothing; 2 naming `@<project>` when there is no current project, several exist, and it cannot ask |
| `todo done` | Ticks tasks. Asks first for a task with unticked sub-tasks. Offers to tick a parent whose last sub-task was ticked, and to mark a milestone reached when its last open task was ticked — each only on acceptance. A repeating task gets its next occurrence, with a new number | 5 when it would have to ask and cannot (use `--yes`) |
| `todo undo` | Unticks; the task returns to the state it had before, including `in progress` | — |
| `todo start` / `cancel` | Sets `in progress` / `cancelled` | — |
| `todo star` / `unstar` | Sets or clears the star, independently of priority | — |
| `todo priority` / `due` | Sets priority or due date for several tasks at once | 2 on an unknown level or malformed date |
| `todo move` | Moves tasks to a milestone, to no milestone, to another project (the milestone is cleared), under another task, or to the top level | 2 when it would put a task under itself |
| `todo edit` | Changes a task's title | 2 on empty text |
| `todo rm` | Deletes tasks | 5 `confirmation_required` |
| `todo show` | One task in full: description, dates, history, notes, links, sub-tasks | 3 on unknown number |
| `todo find` | The list narrowed to tasks whose text contains the words | 0 with a message when none |
| `todo legend` | Explains the symbols in use | — |
| (several numbers) | Each is acted on in turn; a number that matches nothing, or already has the state asked for, is reported and the others are still done | 3 only when no number matched |

## Values

| Value | Rule |
|-------|------|
| `<n>` | a task's short number; ranges (`12-15`) and several numbers are accepted; a `tsk-…` handle works too |
| default scope | the current project; all active projects when none is current; `--project` and `--all-projects` override |
| what is hidden by default | tasks done more than `todo.keep_done` ago, cancelled tasks, and completed, on-hold, and abandoned projects; `--done` and `--everything` show them; the summary says how many are hidden |
| long lists | a milestone with more tasks than `todo.section_limit` shows the first and "+ n more"; `--milestone <ref>` opens it in full |
| narrow terminals | titles are shortened with `…`; columns stay aligned |
| `--output json` | the same content nested as projects → milestones → tasks → sub-tasks, with states and marks as fields and no decoration |
| settings | `todo.default_view` (`board` or `timeline`), `todo.keep_done` (default `7d`), `todo.section_limit` (default `15`), `output.symbols` (`unicode` or `ascii`), and the theme colors of [configuration.md](../../000-foundation/contracts/configuration.md) |
| what is not here | notes that are not tasks go to `trcli idea`; other deadlines are in `trcli due` |
