# Quickstart: Validating Projects, Milestones, Tasks, and the To-do List

**Plan**: [plan.md](./plan.md) | **Contracts**: [contracts/README.md](./contracts/README.md) | **Data model**: [data-model.md](./data-model.md)

How to build `trcli`, run the tests of this feature, and walk through each delivery slice
by hand. Handles and task numbers shown (`prj-…`, `12`) are examples; use the ones your
commands print.

## Prerequisites

- Rust 1.96 or newer
- The foundation in place (`specs/000-foundation`): `trcli init`,
  settings, audit, renderers, the BDD harness
- A minimal person register: `trcli staff add`

## Build and test

```sh
cargo build --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings      # includes the documentation and readability lints
cargo doc --workspace --no-deps --document-private-items  # fails on any undocumented item

cargo test -p trcli-domain projects                        # value objects, aggregates, task graph, progress
cargo test -p trcli-application projects                   # handlers with fakes, marks parser, board builder
cargo test -p trcli-infra-sqlite projects                  # contract tests: fake and SQLite behave the same
cargo test --test bdd -- --input tests/features/projects   # Gherkin scenarios against the binary
cargo test --test usage                                    # examples in docs/usage/*.md
```

**Expected**: every command exits 0. The BDD build enables the `test-clock` feature so that
"today" is fixed and scenarios about due dates are repeatable.

## Manual walk-through

```sh
mkdir /tmp/trcli-projects && cd /tmp/trcli-projects
alias trcli="$OLDPWD/target/debug/trcli"
trcli init --name "Demo"
trcli staff add --name "Lima, Carla" --role "supervisor"
```

### Slice 1 — projects (Story 1)

```sh
trcli project add --title "Doctorate" --type doctoral --from 2026-03-01 --to 2030-02-28 \
      --institution "Example University" --supervisor stf-… --milestones none
trcli project add --title "Side study" --type independent --goal "Try the idea"
trcli project add --title "Bad" --type nosuchtype --from 2026-05-01 --to 2026-01-01
                                        # exit 2: both problems reported together
trcli project list --type doctoral
trcli project use prj-…                 # the doctorate
trcli project current
trcli project status prj-… completed    # exit 2: a closing note is required
trcli project status prj-… on_hold && trcli project status prj-… active
trcli project show prj-… --output json  # shape in contracts/json-output.md
```

Existing workspace check: in a workspace created before this feature, with records and no
project, run any command.

**Expected**: one project of type Independent Research, named after the workspace, holding
every record; nothing lost.

### Slice 2 — records in projects (Story 2)

Needs at least one other record type; use whichever exists (a reference, a dataset).

```sh
trcli ref add --title "A paper"                      # belongs to the current project
trcli project assign prj-…side ref-…                 # now in both; still one record
trcli ref list                                       # current project only
trcli ref list --all-projects
trcli project assign prj-…doctorate ref-… --remove
trcli project assign prj-…side ref-… --remove        # warns: would belong to no project
trcli project unassigned
trcli project summary prj-…
```

### Slice 3 — milestones (Story 3)

```sh
trcli milestone propose                              # shows the six doctoral milestones
trcli milestone propose --accept all                 # dates suggested, marked unconfirmed
trcli milestone propose --accept all                 # adds nothing: already present
trcli milestone add --title "Paper submitted" --date 2027-06-30
trcli milestone add --title "Too late" --date 2031-01-01   # warns: outside the project's period
trcli milestone move mil-… --date 2027-09-30               # exit 2: a reason is required
trcli milestone move mil-… --date 2027-09-30 --reason "Venue moved its deadline"
trcli milestone reach mil-… --date 2099-01-01              # exit 2: in the future
trcli milestone timeline
```

### Slice 4 — tasks (Story 4)

```sh
trcli task add "Collect reference papers" --milestone mil-…
trcli task add "Write related work" --milestone mil-… --priority urgent --due 2026-10-09
trcli task add "Summarize the survey" --parent tsk-…
trcli task add "Book the committee" --after tsk-…          # blocked until that one is done
trcli task add "Loop" --after tsk-…self                    # exit 2: names the loop
trcli task add "Weekly meeting" --repeat weekly --due 2026-10-12
trcli task done tsk-…weekly                                # next occurrence appears, due 2026-10-19
trcli task done tsk-…related                               # warns: has an unfinished sub-task
trcli task list --blocked
trcli task move tsk-… --project prj-…side                  # its milestone is cleared
```

### Slice 5 — to-do list (Story 7)

```sh
trcli todo                                   # the board of the current project
trcli todo --all-projects
trcli todo "Send the abstract to Carla +paper due:fri p:high *"
trcli todo "Mail carla@example.org about p:values"     # no mark recognized: all of it is the title
trcli todo "Something +nosuchmilestone"                # exit 2: the mark names nothing
trcli todo done 12 15 99                     # two change, 99 is reported as not found; exit 0
trcli todo undo 15                           # back to the state it had before
trcli todo star 16 && trcli todo --starred
trcli todo --timeline
trcli todo legend
```

Check the three forms of the same list:

```sh
trcli todo --all-projects                    # symbols and colour
trcli todo --all-projects --plain            # [ ] [~] [x], no colour
trcli todo --all-projects | cat              # piped: no colour codes
trcli todo --all-projects --output json | python3 -m json.tool
COLUMNS=50 trcli todo --all-projects         # titles shortened, columns still aligned
```

**Expected**: all three show the same tasks in the same states; in the plain form every
state and mark is still distinguishable; `trcli task list` agrees with them.

Tick the last open task of a milestone.

**Expected**: the tool offers to mark the milestone reached and does so only if accepted.

### Slice 6 — progress and what is due (Story 5)

```sh
trcli progress                               # reached/total, done/total, time elapsed, next milestone
trcli due --within 14d
trcli due --within 14d --all-projects        # each item names its project
trcli project status prj-…side on_hold
trcli due --within 14d --all-projects        # the on-hold project's items are gone
trcli due --within 1d                        # nothing due: says so, shows the next item
```

Make a milestone overdue (move "today" forward with the test clock, or use past dates).

**Expected**: `trcli progress` shows the project as behind and names the milestone.

### Slice 7 — custom types (Story 6)

```sh
trcli project-type add specialization --description "Specialization course" --degree \
      --detail "Course coordinator"
trcli project-type milestones specialization --set "Project approved@20" --set "Monograph submitted@95"
trcli project add --title "Specialization" --type specialization --milestones all
trcli project-type milestones masters --set "Coursework@40" --set "Defended@100"
trcli project-type milestones masters --restore
trcli project-type rm specialization         # exit 5: a project uses it
trcli project retype prj-… masters           # shows the effect, keeps milestones, asks
```

## Cross-cutting checks

| Check | How | Expected |
|-------|-----|----------|
| Nothing hangs without a terminal | `trcli milestone reach mil-… < /dev/null` (with open tasks) | Exit 5, immediately |
| Completed projects are read-only | complete a project, then `trcli task add "x"` in it | Exit 5, offers `project reopen` |
| Everything is audited | `trcli audit list --kind task` | Every creation, status change, and deletion is there |
| The to-do list is a view | tick with `trcli todo done n`, look with `trcli task show` | Same state |
| Scale (SC-013, SC-005) | load 20 projects and 2,000 open tasks from a script; time `trcli todo --all-projects` | Under 1 second |
| Platforms | CI matrix | All suites green on Linux, macOS, Windows |
| Documentation | `cargo doc --document-private-items` | No warning; every item has a comment |

## Clean up

```sh
cd / && rm -rf /tmp/trcli-projects
```
