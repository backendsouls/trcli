# Phase 0 Research: TRCLI Research Projects, Milestones, and Tasks

**Date**: 2026-10-08 | **Plan**: [plan.md](./plan.md) | **Spec**: [spec.md](./spec.md)

The user fixed the direction: Rust, SeaORM, local storage, async only where necessary, as
few libraries as possible, DDD/TDD/BDD, fully commented and readable code, SOLID applied
actively. This document records the decisions that follow. Crate versions are the latest
on crates.io on 2026-10-08, checked with `cargo search`; the installed toolchain is Rust
1.96.0.

There are no open NEEDS CLARIFICATION items. Points marked **Spike** are small things to
confirm in code at the start of implementation because they rest on library behaviour that
was not exercised during planning.

---

## 1. What this feature needs from the foundation

- **Decision**: This feature is built on `specs/000-foundation` and does
  not re-plan it. It needs exactly these pieces to exist:

  | Needed | Used here for |
  |--------|---------------|
  | Workspace discovery, `trcli init`, settings layers | Finding the database; `todo.*`, `output.symbols` settings |
  | Database connection, embedded migrations | Twelve new tables |
  | `UnitOfWork` with audit recording | Every change and its audit entry commit together (FR-050) |
  | `Handle`, `RecordRef`, tags, notes, links | Identity of projects, milestones, tasks; links to other records |
  | Renderers, theme, exit codes, `--output json` | All commands; the to-do renderer plugs into them |
  | Confirmation prompts honouring `--yes` / `--no-input` | Deleting, reaching a milestone with open tasks |
  | `Clock`, `IdGenerator`, `ActorProvider` ports | Dates, identifiers, audit actor |
  | BDD harness with generic steps | The scenarios of this spec |
  | A minimal person register (`staff add`, `staff list`) | Supervisors, responsible people (FR-005, FR-029) |

- **Rationale**: None of these belongs to projects, and all are needed by every other
  specification. Building them here would duplicate the first plan.
- **Consequence**: The first plan's dependency list was written before the "minimum
  libraries" rule. When the foundation is implemented it should be trimmed the same way as §3
  below: no table crate, no configuration framework, no bibliography or HTTP crates until
  the features that need them are built.
- **Alternatives considered**: Planning a private mini-foundation inside this feature —
  rejected; it would be thrown away or become a second, competing foundation.

## 2. Where the code goes

- **Decision**: No new crate. One bounded context, `projects`, as a module of the same name
  in `trcli-domain`, `trcli-application`, and `trcli-infra-sqlite`, plus `args`, `commands`,
  and `render` modules in `trcli-cli`.
- **Rationale**: The crate-per-layer layout already makes the compiler enforce that the
  domain and application do not import SeaORM, clap, or tokio. A context is a vertical
  slice through those layers, not a new layer.
- **Alternatives considered**: A `trcli-projects` crate holding all layers of this context —
  it would put domain rules and SeaORM entities in one crate, losing the compile-time
  boundary the constitution asks for.

## 3. Libraries: the minimum, each with its reason

- **Decision**: These runtime crates are used by this feature.

  | Crate | Why it is necessary | What we do without |
  |-------|--------------------|--------------------|
  | `clap` 4.6 | Parsing, help text, and shell completion for ~40 subcommands; writing this by hand would be a project of its own | — |
  | `sea-orm` 2.0, `sea-orm-migration` | Chosen by the user | — |
  | `tokio` 1.53, features `rt`, `macros` only | SeaORM needs an async runtime | The multi-threaded scheduler, timers, networking, process and signal features: unused here |
  | `time` 0.3 | Calendar dates, weekdays, adding months correctly | `chrono` — one date library is enough, and SeaORM supports `time` |
  | `uuid` 1.27 (v7) | Identifiers that never collide across copies of a workspace | — |
  | `serde`, `serde_json` | `--output json` (FR-051, FR-071); already required by SeaORM | — |
  | `thiserror` 2.0 | Error types that read clearly; removes ~10 lines of boilerplate per error | Writing `Display` and `Error` by hand, which is noise, not clarity |
  | `unicode-width` 0.2 | Aligning columns when titles contain accents or wide characters (FR-069); cannot be done correctly by counting bytes or characters | — |
  | `terminal_size` 0.4 | Knowing the width to shorten lines to (FR-070) | — |
  | `anstream`, `anstyle` | Colour that turns itself off when piped; **already brought in by clap**, so no new dependency | `owo-colors`, `colored` |

- **Left out on purpose**:

  | Not used | Instead |
  |----------|---------|
  | A table crate (`comfy-table`, `tabled`) | ~150 lines in `render/todo.rs` and `render/width.rs`; the to-do list is a tree, not a grid |
  | `async-trait` | `async fn` in traits, in the language since Rust 1.75 |
  | `anyhow` | Typed errors per layer; the CLI maps them to exit codes |
  | A validation framework (`garde`, `validator`) | Value objects with checking constructors, and a small `ValidationReport` |
  | `chrono`, `chrono-english`, date-parsing crates | A 60-line parser for `today`, `tomorrow`, weekdays, `+3d`, and ISO dates |
  | `regex` | The marks parser is a hand-written tokenizer: simpler to read and to test |
  | `indicatif` | Nothing here runs long enough to need a progress bar |
  | `itertools`, `once_cell`, `lazy_static` | The standard library |

- **Development-only crates**: `cucumber` 0.23 and `assert_cmd` 2.2 (BDD against the
  binary), `trycmd` 1.2 (usage-guide examples as tests), `tempfile` 3.27. They are not
  compiled into `trcli`. `rstest` and `proptest`, listed in the first plan, are dropped:
  table-driven tests are written as plain loops over arrays of cases.
- **Rationale**: "Use minimum Rust libs, but add if necessary." The test for adding a crate
  is: would our own version be more than ~30 lines, or easy to get subtly wrong? Display
  width and terminal size pass that test; table layout for this particular shape, relative
  dates, and mark parsing do not.
- **Spike**: confirm SeaORM 2.0's feature names for SQLite with `time` and `uuid` support and
  a tokio runtime, and that a current-thread runtime is sufficient for it.

## 4. Async only where necessary

- **Decision**: One single-threaded tokio runtime started in `main`. `async` appears on the
  repository ports and on the use-case handlers that await them, and nowhere else. The
  domain crate, the marks parser, the board and timeline builders, and all renderers are
  synchronous.
- **Rationale**: SeaORM is async-only, so storage calls must be awaited. Nothing in this
  feature waits on anything else: no network, no child processes, no parallel work. Keeping
  the rest synchronous makes it trivially testable and easier to read.
- **How the boundary is kept visible**: a handler first awaits what it needs to load, then
  calls synchronous domain methods, then awaits the save. Domain code never receives a
  port.
- **Alternatives considered**: A multi-threaded runtime (unused cost at start-up); hiding
  async behind a blocking wrapper around every repository call (more code, and it would
  have to be undone when a feature such as experiment runs genuinely needs concurrency).

## 5. Aggregates and their boundaries (DDD)

- **Decision**: Four aggregates.

  | Aggregate | Owns | Refers to by id |
  |-----------|------|-----------------|
  | `Project` | title, type, goal, academic details, period, status history | people, project type |
  | `Milestone` | title, target date and its history, status, deliverable | its project |
  | `Task` | title, status history, priority, due date, star, recurrence | its project, milestone, parent task, tasks it depends on, responsible person, records it concerns |
  | `ProjectType` | name, details asked for, milestone template | — |

  Membership of records in projects is its own small model (`Membership`), not part of
  `Project`, because it is changed from every other context.
- **Rationale**: A project with 5,000 tasks cannot be one aggregate — loading it to tick one
  task would be absurd. Tasks and milestones are aggregates of their own and name their
  project. Rules that span several tasks (no dependency loop, no task under itself) are a
  domain service, `task_graph`, given just the tasks of one project.
- **Alternatives considered**: `Project` owning milestones and tasks (too large; every
  change would rewrite the project); one aggregate per table (loses the rule that a
  milestone's date history belongs with the milestone).

## 6. Storage shape

- **Decision**: Ten tables shared between copies of a workspace — `project`,
  `project_status_change`, `project_type`, `milestone_template_item`, `milestone`,
  `milestone_date_change`, `task`, `task_dependency`, `task_concern`, and
  `project_membership` — and two **local-only** tables, `task_number` and `local_state`. Details are in [data-model.md](./data-model.md).
  - Sub-tasks: a `parent_id` on `task`. The tree of one project is loaded in one query and
    assembled in memory; 5,000 rows is well within budget.
  - Membership: `(project_id, record_kind, record_id)`. No other table of any other
    specification gains a column.
  - History (status changes, milestone date changes) is kept as rows, never overwritten.
  - Indexes on `task(project_id, status)`, `task(milestone_id)`, `task(due_on)`,
    `milestone(project_id, target_on)`, `project_membership(record_kind, record_id)`.
- **Rationale**: Local storage in the workspace's SQLite file, as the user asked. Loading a
  project's tasks in one query keeps the to-do list inside one second (SC-013) without
  recursive SQL.
- **Alternatives considered**: Recursive queries for the sub-task tree (harder to read and
  to port; unnecessary at this size); a JSON column for history (cannot be queried for
  reports).

## 7. Short task numbers (FR-058)

- **Decision**: A task's short number is assigned when the task is first seen by a copy of
  the workspace, from a counter in the local-only table `task_number`. It never changes in
  that copy and is never reused. The handle (`tsk-5w0h`) remains the identity that is the
  same everywhere.
- **Rationale**: The spec requires numbers that do not change and are not reused. A number
  stored on the task itself would satisfy that only until the workspace is synchronized
  between two machines (`specs/010-integrations`), when both could assign `26` to different
  tasks. Keeping numbers per copy makes the requirement hold on every machine, at the cost
  that task 26 on the laptop may be task 31 on the workstation — acceptable, because the
  number is something typed while looking at the list, not something written down.
- **Alternatives considered**: A shared counter reconciled at sync (complex, and numbers
  would change — exactly what FR-058 forbids); numbering by position in the last list shown
  (acting on a stale list would tick the wrong task).
- **Spec note**: FR-058 holds per copy of the workspace. This should be stated in the spec's
  assumptions when it is next clarified.

## 8. The to-do list as a pure view (FR-053, FR-054)

- **Decision**: Three small, synchronous parts.
  1. `marks.rs` — a tokenizer that turns `"Send the abstract +paper due:fri p:high *"` into
     a title and a set of validated marks, or a list of problems. A token is a mark only if
     it matches a mark's form exactly; anything else is title text (so `carla@example.org`
     and `p:value` survive).
  2. `board.rs` / `timeline.rs` — functions from loaded projects, milestones, and tasks to
     a view model: boards → sections → lines with their marks and counts, or days → lines.
     No storage, no formatting.
  3. `render/todo.rs` — turns a view model into text, given a `SymbolSet` (unicode or
     ascii), a `Theme`, and a width. `--output json` serializes the same view model.
- **Rationale**: Each part has one reason to change and can be tested with plain values.
  The same view model feeding text and JSON guarantees they agree (SC-015).
- **Acting by number**: `todo done 12 15` resolves each number through `TaskNumbering` and
  calls the existing use cases one by one, collecting a per-number outcome (changed, not
  found, already so), which is how FR-061's "act on the others" is met.

## 9. Readable, commented code — as checks, not intentions

- **Decision**:
  - **Documentation**: `#![deny(missing_docs)]` and
    `#![deny(clippy::missing_docs_in_private_items)]` in every crate, so every module,
    type, function, field, and constant — public or private — has a doc comment or the
    build fails. `cargo doc` runs in CI with warnings as errors.
  - **What a comment says**: the doc comment states what the item is *for*, in the words of
    the spec; inline comments explain *why* a non-obvious line is there (a rule from the
    spec, with its FR number; an edge case; a trade-off). Comments do not restate the code.
  - **Readability lints** (denied): `clippy::too_many_lines` (limit 50),
    `clippy::cognitive_complexity` (limit 10), `clippy::too_many_arguments` (limit 5),
    `clippy::wildcard_imports`, `clippy::enum_glob_use`.
  - **Naming**: whole words from the spec's vocabulary (`milestone`, not `ms`;
    `responsible_person`, not `resp`). No abbreviations except `id`.
  - **Shape**: early returns over nested conditionals; one level of abstraction per
    function; no function both decides and performs I/O.
  - **Each module file starts** with a short paragraph: what it contains, which requirement
    group it implements, and what it deliberately does not do.
- **Rationale**: "All the code should be commented for a human to understand" is only kept
  over time if a machine refuses code that does not comply. The lints make the standard a
  property of the build.
- **Alternatives considered**: A written style guide alone (decays); documenting only public
  items (leaves the private rules, where most of the logic lives, unexplained).

## 10. SOLID, applied

- **Decision**: Each principle is turned into a rule that review and tests can check; the
  table is in [plan.md](./plan.md#solid-in-this-feature). The mechanisms:
  - **Small ports** (interface segregation): `TaskReader`, `TaskWriter`, `TaskNumbering`
    rather than one `TaskRepository` with fifteen methods; a handler lists exactly the ports
    it uses in its type parameters, which doubles as documentation.
  - **Extension points** (open/closed): `DueSource` is a trait with one method,
    `due_between(period, scope)`. This feature registers two sources (tasks, milestones).
    Grants, calls, graduate requirements, and reviews register theirs later; the `due` use
    case and the to-do list are not edited. `SymbolSet` and `Theme` work the same way for
    rendering.
  - **Contract tests** (Liskov): for each repository port, one generic test function is run
    against the in-memory fake and the SQLite adapter. If the fake drifts from the real
    adapter, a test fails.
  - **Composition root** (dependency inversion): `trcli-cli/src/compose.rs` is the only
    place that names concrete adapters.
- **Rationale**: "Apply SOLID actively" — stated as habits it is unverifiable; stated as
  these four mechanisms it shows up in the code structure and in CI.
- **A caution recorded on purpose**: SOLID is applied where there is a real second
  implementation or a real second caller. A trait with one implementation and no test fake
  is not added "for flexibility".

## 11. Validation

- **Decision**: As in the first plan — clap for syntax; a validating constructor per command
  that returns every problem at once; value objects that cannot hold invalid values.
  Specific to this feature:
  - Cross-record checks (the milestone belongs to the same project; the person exists; a
    dependency does not form a loop) are done by the handler through ports, before any
    write, and are reported in the same `ValidationReport`.
  - Warnings that need confirmation (a milestone outside the project's period, a task due
    after its milestone) are a separate list from errors; `--yes` accepts them, and without
    a terminal they fail with exit code 5.
  - The marks parser is validated input like any other: an unknown milestone in `+name` is
    an error naming the mark.
- **Rationale**: Constitution principle V; FR-049.

## 12. Tests: TDD and BDD

- **Decision**:

  | Level | What | Where |
  |-------|------|-------|
  | Unit | Value objects, aggregates, `task_graph`, progress, marks parser, board and timeline builders, handlers with fakes | in `trcli-domain`, `trcli-application` |
  | Contract | Each repository port against fake and SQLite | `trcli-infra-sqlite/tests` |
  | Behaviour | Every acceptance scenario of the spec as Gherkin, run against the built binary in a temporary workspace | `tests/features/projects/*.feature` |
  | Rendering | The to-do list in unicode, ascii, narrow, and JSON forms, compared with expected text | unit tests of `render/todo.rs` with fixed view models |
  | Documentation | Examples in `docs/usage/*.md` | `tests/usage.rs` |

  - **Order within a slice**: scenarios first (red), then inside-out — value object test,
    value object; aggregate test, aggregate; handler test with fakes, handler; contract
    test, repository; argument parsing and rendering — until the scenarios are green.
  - **Time**: every test uses a fixed `Clock`; the binary reads `TRCLI_TEST_NOW` only when
    built with the `test-clock` feature, which the BDD build enables, so scenarios about "overdue" and "in 3 days" are deterministic.
- **Rationale**: The user asked for TDD and BDD; the first plan established testing the
  binary itself.

## 13. Dates and "what is due"

- **Decision**: Dates in this feature are calendar dates without time of day, as the spec
  assumes. "Today" comes from the `Clock` port in the researcher's local time zone.
  Relative phrases (`today`, `tomorrow`, weekday names meaning the next such day, `+3d`,
  `+2w`) are resolved to a date at the moment of input and stored as that date.
  Repetition stores an interval (`daily`, `weekly`, `monthly`, `Nd`, `Nw`, `Nm`); the next
  occurrence is computed from the due date when one exists, otherwise from the completion
  date. Adding months clamps to the last day (31 January + 1 month = 28 or 29 February).
- **Rationale**: Storing resolved dates keeps the data unambiguous; the edge cases are rules
  the domain owns and tests.

## 14. Risks

| Risk | Mitigation |
|------|------------|
| The foundation does not exist yet | Declared as a prerequisite; slice 1 does not start before it |
| Person register is specified in `specs/012-people`, not yet planned | Only `PersonDirectory` (look up by handle, display name) is needed; a minimal adapter satisfies it |
| To-do list alignment across terminals and fonts | `unicode-width`; an ascii symbol set; rendering tests with fixed widths; symbols limited to ones with unambiguous width |
| Per-copy task numbers surprise users who sync | Stated in the usage guide and shown by `trcli todo legend`; handles work everywhere |
| Lints for private docs slow early work | Turned on from the first commit so the cost is never a backlog |
