# Implementation Plan: TRCLI Foundation and Architecture

**Branch**: `000-foundation` | **Date**: 2026-10-08 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/000-foundation/spec.md`

**Note**: This template is filled in by the `/speckit-plan` command; its definition describes the execution workflow.

## Summary

Build the ground every TRCLI feature stands on: a workspace found from wherever the
researcher stands; records that all behave alike (short names, shared actions, tags, notes,
links, guarded deletion); input checked before anything changes; answers for people and for
programs with meaningful exit codes; settings in layers; an audit trail that detects
tampering; help that is always present and always right; and the rules and gates that keep
later features consistent (eight user stories, 76 requirements).

Technical approach: a Rust Cargo workspace of five crates, one per architectural layer or
outside concern, so that the compiler enforces which code may depend on which. A workspace
is a `.trcli/` directory holding a SQLite database reached through SeaORM, and nothing
leaves the machine. Async is used only where storage and signals require it. Libraries are
kept to the smallest set that does the job. Behaviour is specified as Gherkin scenarios and
tested against the compiled binary; tests are written first; every item of code is
documented or the build fails; SOLID is applied as rules that the structure and the tests
make checkable.

This plan **consolidates and replaces** the architectural content of the plan written
earlier under `specs/001-research-workspace`, trimmed by the later instruction to keep
libraries to a minimum.

## Technical Context

**Language/Version**: Rust 1.96, edition 2024

**Primary Dependencies**: clap 4.6 + clap_complete (command line, help, completion), SeaORM 2.0 + sea-orm-migration (storage), tokio 1.53 with only `rt`, `macros`, `signal` (the runtime SeaORM needs; Ctrl-C), time 0.3 (dates), uuid 1.27 (identifiers), serde + serde_json (structured output), toml 1.1 (settings files), sha2 0.11 (audit chain), thiserror 2.0 (error types), unicode-normalization 0.1 and unicode-width 0.2 (search and alignment in any script), terminal_size 0.4. Colour comes from anstream/anstyle, which clap already brings in. Fifteen direct runtime crates; each is justified in [research.md](./research.md) §3.

**Storage**: Local only — one SQLite database per workspace at `<workspace>/.trcli/trcli.db` (path configurable), with `config.toml`, `audit.head`, and `backups/` beside it; user settings in the platform's configuration directory

**Testing**: `cargo test` for unit tests with in-memory fakes of every port; contract tests run against both the fakes and the SQLite adapter; cucumber 0.23 + assert_cmd running the spec's Gherkin scenarios against the built `trcli` binary; trycmd executing the examples in `docs/usage/*.md`. Test crates are development dependencies only.

**Target Platform**: Linux, macOS, and Windows (x86_64 and aarch64), one self-contained binary with SQLite compiled in

**Project Type**: CLI

**Performance Goals**: a simple command answers in under 100 ms (SC-012); listing or searching 10,000 records of one kind in under 2 s (SC-012); a workspace is created and ready in seconds (SC-001)

**Constraints**: nothing leaves the machine (SC-010); every change is whole or absent, even on power loss (SC-008); no command waits for an answer nobody can give (SC-007); every input validated before any write (SC-004); colour and symbols never the only carrier of meaning; identical behaviour on three operating systems (SC-014)

**Scale/Scope**: single researcher; one workspace up to ~100,000 records and ~1,000,000 audit entries; 8 user stories, 76 requirements; 5 crates; 8 tables; the `init`, `workspace`, `config`, `link`, `tag`, `note`, `audit`, `telemetry`, `completions` commands plus the machinery every later command uses

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Gate | How this plan meets it | Status |
|-----------|------|------------------------|--------|
| I. Clean Architecture | Layers; dependencies inward; external concerns behind ports | Five crates by layer. `trcli-domain` and `trcli-application` have no dependency on clap, SeaORM, tokio, or the file system, so an outward import does not compile; a test reads the dependency graph and fails if that ever changes (FR-070). Storage, files, clock, identifiers, prompts, and paths are ports. | PASS |
| II. Domain-Driven Design | Ubiquitous language; aggregates; invariants in the domain | The foundation's own model — Workspace, Record identity, Tag, Note, Link, Setting, AuditEntry — uses the spec's words; value objects make invalid values unrepresentable; each feature is a bounded context added as a module in each layer, referring to others by `RecordRef` only (FR-072). | PASS |
| III. Test-First (non-negotiable) | Tests written and failing before code | Phase workflow: failing scenarios → failing unit tests → code. The pull-request template and review checklist require evidence of the red step; Phase 2 builds the harness before any feature code. | PASS |
| IV. BDD | Given/When/Then for every user-facing feature, automated | The spec's 95 acceptance scenarios are automated: 89 as Gherkin run against the compiled binary, and six contributor scenarios (User Story 8, 3 to 7 and 9) as structural tests — scenario coverage, invalid-input coverage, documentation and help examples, the three-system CI matrix, layering, and upgrades. Every scenario is tagged with the one it automates, and `tests/scenario_coverage.rs` fails when one is missing. Generic steps mean later features add scenarios, not step code. | PASS |
| V. Input Validation (non-negotiable) | All input validated at the boundary; clear message; non-zero exit; tests per input | One mechanism for all features: clap for syntax, validating command constructors that report every problem together, value objects, and one `Problem` format. Exit code 2. A gate checks that every command has scenarios for invalid input. | PASS |
| VI. Full Code Documentation | Doc comment on every module and public item | `missing_docs` denied for public items and `clippy::missing_docs_in_private_items` for private ones; `cargo doc` with warnings as errors. Undocumented code does not build. | PASS |
| VII. Feature Docs and Usage Guides | Usage Markdown per feature, shipped with it | `docs/usage/<noun>.md` per command group; examples executed by trycmd (FR-058); a test fails if a command has no help example or a command group has no guide. | PASS |

**Rules from the user's planning inputs across this project**, treated as gates:

| Rule | Gate |
|------|------|
| Rust, clap, SeaORM, SQLite in a `.trcli` directory | As in Technical Context |
| Works on macOS, Linux, Windows | All platform differences live in one crate; CI runs every suite on all three |
| Local now, remote later | Storage and files are behind ports; record identifiers never collide across copies; nothing remote is built |
| Configurable | Layered settings with one registry every feature adds to |
| Colours in the terminal | Through the crates clap already uses; off when piped; a theme the user can change |
| Async where necessary | Only storage ports, the handlers that await them, and signal handling |
| Minimum libraries | A crate is added only with a written reason; [research.md](./research.md) §3 lists what was left out and what replaces it |
| DDD, TDD, BDD | Rows II, III, IV above |
| Code commented for a human; readable | Row VI, plus lints on function length and complexity, and naming rules |
| SOLID, actively | One rule and one check per principle — see [SOLID as rules](#solid-as-rules) |

**Constitution follow-up**: the constitution carries `TODO(TECH_STACK)`. This plan settles
it. Amending the constitution (a MINOR change, to 1.1.0) is done with
`/speckit-constitution`, not by this command.

**Post-design re-check (after Phase 1)**: PASS. The data model keeps every rule in the
domain; the feature contract gives later features one way to plug in; no design choice
needed an outward dependency from the domain or application crates.

## Project Structure

### Documentation (this feature)

```text
specs/000-foundation/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output (/speckit-plan command)
├── data-model.md        # Phase 1 output (/speckit-plan command)
├── quickstart.md        # Phase 1 output (/speckit-plan command)
├── contracts/           # Phase 1 output (/speckit-plan command)
│   ├── README.md
│   ├── cli-conventions.md  output-and-exit-codes.md  configuration.md   # rules for every command
│   ├── cli-workspace.md  cli-link.md  cli-audit.md                      # commands owned here
│   └── feature-contract.md   # what a feature receives and must provide (added by this plan)
└── tasks.md             # Phase 2 output (/speckit-tasks command - NOT created by /speckit-plan)
```

### Source Code (repository root)

```text
Cargo.toml                      # workspace: members, shared dependency versions, shared lints
rust-toolchain.toml             # pins Rust 1.96
clippy.toml                     # limits for function length, complexity, arguments
.github/workflows/ci.yml        # fmt, clippy, doc, tests on linux / macos / windows
.github/workflows/release.yml   # one self-contained binary per system and architecture
CONTRIBUTING.md                 # build, check, add a command (FR-076)

crates/
├── trcli-domain/               # pure rules: no I/O, no async, no framework
│   └── src/
│       ├── lib.rs
│       ├── shared/             # used by every context
│       │   ├── record.rs       # RecordId, RecordKind, RecordRef, Handle
│       │   ├── text.rs         # Title, Name, LongText, TagName, SearchKey (case/accent-free)
│       │   ├── tag.rs  note.rs  link.rs
│       │   └── problem.rs      # FieldProblem, ValidationReport
│       ├── workspace/          # Workspace, FormatVersion
│       ├── settings/           # SettingKey, SettingValue, SettingDefinition, Scope
│       └── governance/         # AuditEntry, AuditAction, chain hashing rule, TelemetryRecord
│
├── trcli-application/          # use cases and ports
│   └── src/
│       ├── ports/              # Storage, UnitOfWork, RecordIndex, TagStore, NoteStore, LinkStore,
│       │                       # AuditLog, TelemetryLog, WorkspaceLocator, SettingsSource,
│       │                       # Clock, IdGenerator, ActorProvider, Prompter, Progress
│       ├── kinds.rs            # RecordKindDescriptor + registry: how a feature plugs in (FR-068)
│       ├── records/            # generic use cases: resolve a reference, tag, note, link, delete
│       ├── workspace/          # init, show, edit, upgrade, check
│       ├── settings/           # layering, validation, get / set / unset / list
│       ├── governance/         # record an entry, query, verify, export; telemetry
│       ├── outcome.rs          # Outcome and Problem: the fixed set of ways a command ends
│       ├── validation.rs       # helpers and the pattern every command constructor follows
│       ├── sample/             # two sample record kinds (feature "sample-kind"; never released)
│       └── testing/            # in-memory fakes of every port and the contract test suites
│                               # (feature "test-support")
│
├── trcli-infra-sqlite/         # SeaORM entities, migrations, port implementations
│   ├── src/{connection.rs, unit_of_work.rs, entities/, migrations/, stores/}
│   └── tests/                  # contract tests: the same suite as the fakes
│
├── trcli-infra-system/         # everything that differs between operating systems
│   ├── src/{paths.rs, locator.rs, settings_files.rs, clock.rs, ids.rs, actor.rs,
│   │        audit_head.rs, backup_copy.rs}
│   └── tests/
│
└── trcli-cli/                  # the `trcli` binary: composition root and presentation
    └── src/
        ├── main.rs             # runtime, signal handling, exit code
        ├── compose.rs          # the only place that names concrete adapters
        ├── args/               # clap definitions, one module per command group
        ├── shared_verbs.rs     # builds list / show / rm / tag / note for any record kind
        ├── commands/           # parsed arguments → use case → view model
        ├── render/             # human.rs, json.rs, theme.rs, symbols.rs, width.rs, problem.rs
        ├── prompt.rs           # terminal, assume-yes, and refuse-to-ask Prompter
        └── progress.rs

tests/                          # black-box tests of the binary
├── features/foundation/        # Gherkin, one file per user story (8)
├── bdd/                        # cucumber world and generic steps, reused by every feature
├── usage.rs                    # trycmd over docs/usage/*.md
├── help_examples.rs            # every command has help with an example; every group has a guide
├── layering.rs                 # the dependency graph obeys the layers (FR-070)
├── scenario_coverage.rs        # every acceptance scenario of the spec has an automated check
├── invalid_input_gate.rs       # every input of every command has a rejection scenario
├── sample_kind_is_external.rs  # no foundation file names the sample kinds (FR-068)
├── forms_agree.rs              # the two output forms carry the same content (SC-005)
├── performance.rs  any_script.rs  offline.rs  no_half_written_files.rs  read_only_workspace.rs
└── fixtures/formats/           # a workspace of every earlier format, for upgrade tests (FR-075)

docs/
├── usage/                      # workspace.md, config.md, records.md, audit.md
└── contributing/               # adding-a-command.md, adding-a-record-kind.md
```

**Structure Decision**: A Cargo workspace of five crates. Dependency direction is
`trcli-cli` → `trcli-infra-*` → `trcli-application` → `trcli-domain`; the two adapter
crates do not depend on each other. Each later feature adds a module of the same name to
the domain, application, and SQLite crates and to the CLI's `args`, `commands`, and
`render`; it adds a crate only when it brings a new outside concern (for example the
network), which the foundation does not have.

## Architecture Notes

### The path of one command

```text
argv ─▶ clap ─────────────▶ syntax checked; help and "did you mean" come from here
     ─▶ locate workspace ─▶ flag › session variable › nearest .trcli › default        (FR-003)
     ─▶ load settings ────▶ default ‹ user ‹ workspace ‹ session ‹ command            (FR-040)
     ─▶ command constructor ▶ every value validated; all problems returned together   (FR-023)
     ─▶ handler (async) ──▶ loads through ports ─▶ domain (sync) decides
                          ─▶ saves through ports ┐
                          ─▶ audit entry ────────┴─▶ unit of work: both, or neither   (FR-047)
     ◀─ view model
     ─▶ renderer ─────────▶ stdout: for people, or one JSON document                  (FR-028)
                            stderr: problems, warnings, progress, questions           (FR-031)
     ─▶ Outcome ──────────▶ exit code                                                 (FR-032)
```

### Where async is, and is not

| Part | Async? | Why |
|------|--------|-----|
| `trcli-domain` | No | Rules on values in memory |
| Validation, settings layering, rendering | No | Pure functions |
| Ports that reach the database | Yes | SeaORM is async |
| Use-case handlers | Yes | They await those ports |
| Ctrl-C handling | Yes | Awaiting a signal while work proceeds |
| `main` | One single-threaded tokio runtime | Nothing in the foundation runs in parallel |

Ports use `async fn` in traits (in the language since Rust 1.75) with generic handlers; no
macro crate is needed.

### SOLID as rules

| Principle | Rule in this codebase | How it is checked |
|-----------|-----------------------|-------------------|
| **S**ingle responsibility | One use case per file; one reason to change per module; a function either decides or performs I/O, not both | Lints on length and complexity; review |
| **O**pen/closed | A feature is added by registering — a record kind, settings, problems, sources — never by editing a central `match` | `kinds.rs` and the settings registry have no list of features; the sample kinds are added without any foundation file naming them, which a test checks |
| **L**iskov substitution | Every implementation of a port passes the port's contract tests | One generic test suite per port, run against the fake and the real adapter |
| **I**nterface segregation | Ports are small and named for what the caller needs (`RecordResolver`, not `Database`) | A handler's type parameters list exactly the ports it uses |
| **D**ependency inversion | Use cases depend on ports defined beside them; adapters depend on the application crate | `tests/layering.rs` reads `cargo metadata` and fails on a forbidden edge; `compose.rs` is the only file naming adapters |

A caution written down on purpose: a trait is introduced where there is a second
implementation (at least a test fake) or a second caller — not "for flexibility".

### Local now, remote later

Nothing remote is built. Three decisions keep the door open at no extra cost today, because
each is needed anyway:

| Decision | Needed now for | Lets a later feature |
|----------|----------------|----------------------|
| Storage behind ports with a `UnitOfWork` | Testing with fakes; atomic change + audit entry | Add another store or a sync engine as a new adapter crate |
| Record ids are UUID v7; short names are separate | Short typeable names (FR-010) | Merge copies of a workspace without identifier clashes |
| The audit trail is an ordered, hash-chained journal | Tamper detection (FR-048) | Replay changes between copies (`specs/010-integrations`) |

A port for files that records refer to (datasets, figures) is **not** defined here: no
foundation feature reads such files. It arrives with the first feature that does.

## Delivery

Delivery follows the phases of [tasks.md](./tasks.md), one stacked pull request per phase.
What several user stories share — the unit of work with audit recording, settings sources,
validation types, the renderers — is built in its Phase 2, so that the stories can then
follow the spec's priority order.

FR-062 to FR-067 (qualities) are not a phase of their own: they are met by every phase,
checked by CI on three systems, and measured in the final phase.

Phase 1 carries the two spikes named in [research.md](./research.md).

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

There are no constitution violations. Four choices add structure beyond the minimum and are
recorded for reviewers:

| Choice | Why Needed | Simpler Alternative Rejected Because |
|--------|------------|-------------------------------------|
| Five crates instead of one | The constitution and FR-070 require the layering to be checked automatically; crate boundaries make the compiler do it, and keep SeaORM out of domain test builds | One crate with modules relies on review, or on an extra lint tool, to catch a forbidden import |
| A record index table beside each feature's own tables | Short names must be unique across all kinds and never reused (FR-010); tags, notes, and links must be able to point at any record with real integrity (FR-017); counts per kind must be cheap (FR-006) | Without it, every generic mechanism would need a case per feature table, and "nothing left dangling" would be a convention instead of a constraint |
| Two identifiers per record (UUID + short name) | Typeable names now; safe merging of copies later | A counter is short but clashes on merge; a UUID alone cannot be typed |
| Our own small code instead of crates for platform paths, settings layering, tables, spinner, and similarity | The instruction to keep libraries to a minimum; each is under ~60 lines and specific to our needs | Five more dependencies, each doing far more than we use |
