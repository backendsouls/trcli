# Implementation Plan: TRCLI Research Projects, Milestones, and Tasks

**Branch**: `003-research-projects` | **Date**: 2026-10-08 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/003-research-projects/spec.md`

**Note**: This template is filled in by the `/speckit-plan` command; its definition describes the execution workflow.

## Summary

Add typed research projects, milestones, tasks, progress, a view of what is due, custom
project types, and a to-do list that shows all of it as boards, sections, and checkboxes
(seven user stories, 72 functional requirements).

Technical approach: Rust, in the layered Cargo workspace already chosen for TRCLI — a pure
domain crate, an application crate of use cases and ports, a SQLite adapter using SeaORM,
and the `trcli` command-line crate. Everything is stored locally in the workspace's SQLite
database. Async is used only where the storage library requires it; the domain, the
parsing of to-do marks, and all rendering are plain synchronous code. Dependencies are kept
to the smallest set that does the job: six runtime crates beyond what the foundation
already needs, and no helper crates where a few lines of our own code are clearer. The work
is driven by Gherkin scenarios run against the compiled binary (BDD), written before the
code (TDD), with every item documented and SOLID applied as concrete, checkable rules.

## Technical Context

**Language/Version**: Rust 1.96, edition 2024

**Primary Dependencies**: clap 4.6 (command line), SeaORM 2.0 + sea-orm-migration (storage), tokio 1.53 with only the `rt` and `macros` features (the async runtime SeaORM needs), time 0.3 (dates), uuid 1.27 (identifiers), serde + serde_json (structured output), thiserror 2.0 (error types), unicode-width 0.2 and terminal_size 0.4 (aligning the to-do list). Colour comes from anstream/anstyle, which clap already brings in. See [research.md](./research.md) §3 for why each is there and what was left out.

**Storage**: Local only — the workspace's SQLite database at `<workspace>/.trcli/trcli.db`, ten new tables shared between copies and two local-only tables

**Testing**: `cargo test` for unit tests in the domain and application crates (in-memory fakes of every port); repository tests against a temporary SQLite file; cucumber 0.23 + assert_cmd running the Gherkin scenarios of the spec against the built `trcli` binary; trycmd executing the examples in the usage guides. All test crates are development dependencies only.

**Target Platform**: Linux, macOS, and Windows, as one self-contained binary

**Project Type**: CLI

**Performance Goals**: to-do list shown in under 1 s with 20 projects and 2,000 open tasks (SC-013); listing a project's tasks or records in under 2 s with 20 projects, 5,000 tasks, and 10,000 papers (SC-005); a simple command starts in under 100 ms

**Constraints**: everything local, nothing sent anywhere; every input validated before any write; no command may wait on a prompt when not attached to a terminal; every state shown in the to-do list distinguishable without colour or special symbols (FR-070); the to-do list stores nothing of its own (FR-054)

**Scale/Scope**: single researcher; up to ~20 projects, ~200 milestones, ~5,000 tasks per workspace; 7 user stories, 72 requirements, 6 CLI command groups, one bounded context (`projects`)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Gate | How this plan meets it | Status |
|-----------|------|------------------------|--------|
| I. Clean Architecture | Layers; dependencies inward; external concerns behind ports | The `projects` context lives in three crates by layer. `trcli-domain` and `trcli-application` do not depend on SeaORM, clap, or tokio, so an outward import does not compile. Storage, clock, identifiers, people, and "what is due" sources are ports. | PASS |
| II. Domain-Driven Design | Ubiquitous language; aggregates; invariants in the domain | Four aggregates named as in the spec — Project, Milestone, Task, ProjectType — with value objects for every validated value; other contexts are referred to by `RecordRef` only. Rules such as "no dependency loop" and "completed projects are read-only" live in the domain. | PASS |
| III. Test-First (non-negotiable) | Tests written and failing before code | Each slice starts from failing Gherkin scenarios, then failing unit tests, then code. Order is enforced in `tasks.md` and in review. | PASS |
| IV. BDD | Given/When/Then for every user-facing feature, automated | The 74 acceptance scenarios of the spec become Gherkin scenarios in `tests/features/projects/`, run against the compiled binary. | PASS |
| V. Input Validation (non-negotiable) | All input validated at the boundary; clear message; non-zero exit; tests per input | clap checks syntax; command constructors validate every field and report all problems together; value objects cannot hold invalid values. The to-do "marks" parser is a validating parser with its own table of tests. Exit code 2. | PASS |
| VI. Full Code Documentation | Doc comment on every module and public item | `missing_docs` denied for public items **and** `clippy::missing_docs_in_private_items` denied for private ones, so undocumented code does not build. Comments explain *why*; see [research.md](./research.md) §9. | PASS |
| VII. Feature Docs and Usage Guides | Usage Markdown per feature, shipped with it | `docs/usage/{project,milestone,task,todo,due,project-type}.md`, each written in the slice that delivers the commands; their examples are run as tests. | PASS |

**Additional rules from the user's planning input**, treated as gates of this plan:

| Rule | Gate |
|------|------|
| Minimum libraries | A crate is added only with a written reason in `research.md` §3; nothing is added for what 30 lines of our own code do clearly |
| Async where necessary | `async` appears only on ports that reach storage and on the handlers that await them; domain, parsing, and rendering are synchronous |
| Readable code | Lints for function length and complexity are on; names are whole words from the spec's language |
| SOLID, actively | Each principle has a concrete rule and a place it is checked — see [SOLID in this feature](#solid-in-this-feature) |

**Prerequisite, not a violation**: this feature needs the foundation of
`specs/000-foundation`: workspace discovery and `trcli init`, the
database connection and migrations, the unit of work with audit recording, handles, the
output renderers and exit codes, settings, and the BDD harness. It also needs a minimal
person register for supervisors and responsible people. Neither exists yet. They are
listed in [research.md](./research.md) §1 and must be delivered first; this plan does not
re-plan them.

**Post-design re-check (after Phase 1)**: PASS. The data model keeps every invariant in the
domain; no design choice needed an outward dependency from the domain or application
crates; the to-do list adds two stored facts (a star and a local number) and no second
store.

## Project Structure

### Documentation (this feature)

```text
specs/003-research-projects/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output (/speckit-plan command)
├── data-model.md        # Phase 1 output (/speckit-plan command)
├── quickstart.md        # Phase 1 output (/speckit-plan command)
├── contracts/           # Phase 1 output (/speckit-plan command)
│   ├── README.md
│   ├── cli-project.md  cli-milestone.md  cli-task.md
│   ├── cli-due.md  cli-project-type.md  cli-todo.md
│   └── json-output.md   # structured output of these commands (added by this plan)
└── tasks.md             # Phase 2 output (/speckit-tasks command - NOT created by /speckit-plan)
```

### Source Code (repository root)

Only what this feature adds is shown; the crates themselves come from the foundation.

```text
crates/
├── trcli-domain/src/projects/           # pure: no I/O, no async, no framework
│   ├── mod.rs                           # what the context is; re-exports
│   ├── project.rs                       # Project aggregate, ProjectStatus, AcademicDetails
│   ├── project_type.rs                  # ProjectType, built-in types, MilestoneTemplate
│   ├── milestone.rs                     # Milestone aggregate, MilestoneStatus, date history
│   ├── task.rs                          # Task aggregate, TaskStatus, Priority, Recurrence
│   ├── task_graph.rs                    # sub-task tree and dependency rules (no loops)
│   ├── membership.rs                    # which records belong to which projects
│   ├── progress.rs                      # progress and "behind" calculation
│   └── values.rs                        # ProjectTitle, TaskTitle, TaskNumber, Period, …
│
├── trcli-application/src/projects/      # use cases and ports
│   ├── ports.rs                         # ProjectRepository, MilestoneRepository, TaskRepository,
│   │                                    # ProjectTypeRepository, MembershipRepository,
│   │                                    # TaskNumbering, CurrentProject, PersonDirectory
│   ├── commands/                        # one file per use case that changes data
│   ├── queries/                         # one file per use case that only reads
│   ├── due.rs                           # DueSource port + sources for tasks and milestones
│   ├── scope.rs                         # ProjectScope: current project, read-only guard
│   ├── todo/
│   │   ├── marks.rs                     # parses "text +milestone due:fri p:high *"
│   │   ├── board.rs                     # builds the board view model
│   │   └── timeline.rs                  # builds the timeline view model
│   └── testing.rs                       # in-memory fakes of the ports (feature "test-support")
│
├── trcli-infra-sqlite/src/projects/     # SeaORM entities, migrations, repositories
│   ├── entities/                        # one file per table
│   ├── migrations/                      # one migration per delivery slice
│   └── repositories/                    # one file per port implemented
│
└── trcli-cli/src/
    ├── args/{project,milestone,task,todo,due,project_type}.rs   # clap definitions
    ├── commands/projects/               # parsed args → use case → view
    └── render/
        ├── todo.rs                      # lays out boards, sections, task lines
        ├── symbols.rs                   # SymbolSet: unicode and ascii implementations
        └── width.rs                     # display width and shortening of titles

tests/
├── features/projects/                   # Gherkin, one file per user story (7)
└── bdd/                                 # generic steps (from the foundation)

docs/usage/
└── project.md  milestone.md  task.md  todo.md  due.md  project-type.md
```

**Structure Decision**: No new crate. The feature is one bounded context, `projects`,
added as a module with the same name in each existing layer crate, plus argument, command,
and rendering modules in the CLI crate. Dependency direction is unchanged: CLI → adapters →
application → domain.

## Architecture Notes

### One command, end to end

```text
argv ─▶ clap (syntax) ─▶ command constructor (validates every field, reports all problems)
     ─▶ handler (async: awaits ports) ─▶ aggregate method (sync: enforces invariants)
     ─▶ repository port ┐
     ─▶ audit entry ─────┴─▶ unit of work commits both, or neither
     ◀─ view model ◀─ query
     ─▶ renderer (sync) ─▶ stdout            problems ─▶ stderr + exit code
```

### Where async is, and is not

| Layer | Async? | Why |
|-------|--------|-----|
| Domain (`trcli-domain`) | No | Pure rules on values in memory |
| To-do marks parser, board and timeline builders | No | Pure functions from records to view models |
| Renderers | No | Formatting text |
| Ports that reach storage | Yes | SeaORM is async |
| Use-case handlers | Yes | They await those ports |
| `main` | One single-threaded tokio runtime | Nothing here runs in parallel; a multi-threaded runtime would be unused weight |

Ports use `async fn` in traits, a language feature since Rust 1.75, with generic handlers;
no macro crate is needed for it.

### SOLID in this feature

| Principle | Rule in this codebase | Where it shows |
|-----------|-----------------------|----------------|
| **S**ingle responsibility | One use case per file; an aggregate changes only for a change in its own rules | `commands/complete_task.rs` does one thing; `Task` knows nothing of storage or display |
| **O**pen/closed | New behaviour is added by a new implementation of a port, not by editing a `match` | `DueSource`: tasks and milestones are two sources; grants, calls, and requirements are added later by other specs without touching `due`. `SymbolSet`: unicode and ascii |
| **L**iskov substitution | Every implementation of a port passes the same contract tests | One shared test suite per repository port, run against the in-memory fake and the SQLite adapter |
| **I**nterface segregation | Ports are small and named for what the caller needs | A handler that only reads tasks takes `TaskReader`, not the whole repository; other contexts see `ProjectScope` only |
| **D**ependency inversion | Use cases depend on ports defined beside them; adapters depend on the application crate | `trcli-application` has no dependency on SeaORM; wiring happens once, in the CLI's `compose.rs` |

Contract tests for Liskov and the lint configuration for readability are part of slice 1.

### How the to-do list stays a view (FR-054)

`board.rs` is a pure function: given projects, milestones, and tasks, it returns a tree of
boards, sections, and lines. `trcli todo done 12` resolves the number to a task and calls
the same `CompleteTask` use case as `trcli task done`. There is no to-do table.

## Delivery Slices

Each slice is one stacked pull request, opens with failing Gherkin scenarios, and ends with
its usage guide. Slice 5 is taken out of the spec's priority order on purpose: the to-do
list is the command used most, and needs only slices 3 and 4.

| Slice | Content | Spec |
|-------|---------|------|
| — | **Prerequisite**: foundation (`specs/000-foundation`) and a minimal person register | — |
| 1 | Projects: types, academic details, status and history, current project, read-only guard, default project for an existing workspace; lint and contract-test setup | Story 1 · FR-001–011, 018 |
| 2 | Records in projects: membership, `--project` / `--all-projects`, unassigned records, summary | Story 2 · FR-012–017 |
| 3 | Milestones: proposals per type, suggested dates, date history, reach, overdue, timeline | Story 3 · FR-019–027 |
| 4 | Tasks: sub-tasks, dependencies, repetition, filters, move | Story 4 · FR-028–039 |
| 5 | To-do list: board, marks, acting by number, timeline, plain form, structured form | Story 7 · FR-053–072 |
| 6 | Progress and what is due, with the `DueSource` port | Story 5 · FR-040–043 |
| 7 | Custom project types and milestone templates; change of a project's type | Story 6 · FR-045–048 |

FR-044 (progress report export) is superseded by `specs/006-reports` and is not built here.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

There are no constitution violations. Three choices are recorded for reviewers because they
trade a little structure for a stated goal:

| Choice | Why Needed | Simpler Alternative Rejected Because |
|--------|------------|-------------------------------------|
| Task numbers are kept per copy of the workspace, in a local-only table | FR-058 needs short numbers that never change; once workspaces sync between machines (`specs/010-integrations`), a shared counter would hand out the same number twice | A column on the task is simpler but breaks on the first sync; handles remain the identity shared between machines |
| Our own layout code for the to-do list instead of a table library | The list is a nested tree with marks, not a grid; a table crate would not produce it, and "minimum libraries" asks us not to add one that only half fits | A table library still needs most of the same code around it |
| Built-in project types are constants in the domain; only changes to them are stored | Restoring a built-in type's milestones (FR-046) is then "delete the stored change", and a new version of the tool can improve built-ins without a migration | Seeding built-ins as rows makes restoring and upgrading them harder |
