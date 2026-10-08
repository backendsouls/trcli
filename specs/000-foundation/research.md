# Phase 0 Research: TRCLI Foundation and Architecture

**Date**: 2026-10-08 | **Plan**: [plan.md](./plan.md) | **Spec**: [spec.md](./spec.md)

The decisions behind the plan. The direction was fixed by the user across several planning
inputs: Rust; clap; SeaORM with SQLite in a `.trcli` directory; macOS, Linux, and Windows;
local now, remote later; configurable; coloured output; DDD, TDD, BDD (testing the CLI
itself); async where necessary; as few libraries as possible; fully commented, readable
code; SOLID applied actively. Crate versions are the latest on crates.io on 2026-10-08,
checked with `cargo search`; the installed toolchain is Rust 1.96.0.

This document consolidates and replaces the research written earlier under
`specs/001-research-workspace`. Where it differs from that document, this one holds; the
differences are listed in §16.

There are no open NEEDS CLARIFICATION items. Two points marked **Spike** are to be
confirmed in code in Phase 1 of the tasks.

---

## 1. Language and toolchain

- **Decision**: Rust, edition 2024, pinned to 1.96 in `rust-toolchain.toml`.
- **Rationale**: Chosen by the user. One static binary per platform and nothing to install
  (FR-062). `async fn` in traits and async closures are in the language, so ports need no
  macro crate.
- **Alternatives considered**: None; fixed by the user.

## 2. Five crates, one per layer or outside concern

- **Decision**: `trcli-domain` → `trcli-application` → `trcli-infra-sqlite` and
  `trcli-infra-system` → `trcli-cli`.

  | Crate | Contains | May depend on |
  |-------|----------|---------------|
  | `trcli-domain` | Entities, value objects, rules | `thiserror`, `time`, `uuid`, `unicode-normalization` |
  | `trcli-application` | Use cases, ports, the kind and settings registries, fakes | domain; `serde` for view models |
  | `trcli-infra-sqlite` | SeaORM entities, migrations, stores | application, domain; `sea-orm`, `sha2` |
  | `trcli-infra-system` | Paths, workspace discovery, settings files, clock, ids, actor | application, domain; `toml` |
  | `trcli-cli` | clap, rendering, prompts, composition | all of the above; `clap`, `tokio`, `serde_json`, … |

- **Rationale**: The constitution (principle I) and FR-070 require that the rules of the
  research depend on nothing about storage, display, or invocation, **and that this is
  checked automatically**. A crate that does not list SeaORM cannot import it. The check is
  therefore the compiler, reinforced by `tests/layering.rs`, which reads `cargo metadata`
  and fails if a forbidden dependency is ever added to a manifest.
- **Alternatives considered**:
  - *One crate with modules* — fewest files, but layering becomes a convention, and every
    domain test build compiles SeaORM.
  - *Seven crates* as in the earlier plan — the two extra (bibliography, catalogue) belong
    to features and are not part of the foundation.
  - *One crate per bounded context* — sixteen specifications times three layers; contexts
    are modules inside each layer crate instead.

## 3. Libraries: the minimum, each with its reason

- **Decision**: fifteen direct runtime dependencies.

  | Crate | Why it is necessary |
  |-------|--------------------|
  | `clap` 4.6 | Parsing, help, and "did you mean" for what will be several hundred subcommands; FR-024, FR-055, FR-056 |
  | `clap_complete` 4.6 | Shell completion scripts (FR-059); generated from the same definitions |
  | `sea-orm` 2.0 | Chosen by the user |
  | `sea-orm-migration` 2.0 | Versioned, embedded schema changes (FR-007, FR-075) |
  | `tokio` 1.53 — `rt`, `macros`, `signal` | The runtime SeaORM needs; Ctrl-C (FR-036). No scheduler threads, timers, network, or process features |
  | `time` 0.3 | Calendar dates and UTC instants; one date library, supported by SeaORM |
  | `uuid` 1.27 — `v7` | Identifiers that never collide across copies of a workspace |
  | `serde`, `serde_json` | The structured form (FR-028); already required by SeaORM |
  | `toml` 1.1 | Reading and writing settings files (FR-039) |
  | `sha2` 0.11 | The audit chain (FR-048); a hash function must not be hand-written |
  | `thiserror` 2.0 | Error types that read clearly, without boilerplate |
  | `unicode-normalization` 0.1 | Search and sort that ignore accents in any script (FR-067); cannot be done correctly by hand |
  | `unicode-width` 0.2 | Column alignment with accented and wide characters (FR-029) |
  | `terminal_size` 0.4 | Width to lay out and shorten to (FR-029) |

  `anstream` and `anstyle` (colour that turns itself off when piped, `NO_COLOR`) are
  **already dependencies of clap**; using them adds nothing.

- **Left out on purpose** — each replaced by a small, documented module of our own:

  | Not used | Instead | Size |
  |----------|---------|------|
  | `figment`, `config` | `settings/` layering over `toml` values | ~150 lines |
  | `directories`, `dirs` | `paths.rs`: three rules, one per operating system, from environment variables | ~40 lines |
  | `comfy-table`, `tabled` | `render/` columns over `unicode-width` | ~120 lines |
  | `indicatif` | `progress.rs`: a spinner and a counter on stderr | ~50 lines |
  | `strsim` | `similar.rs`: edit distance for "close matches" of handles (FR-011) | ~25 lines |
  | `dialoguer`, `inquire` | `prompt.rs`: yes/no and a line of text | ~40 lines |
  | `whoami` | `actor.rs`: the user name from the environment | ~15 lines |
  | `async-trait` | `async fn` in traits | language |
  | `anyhow` | Typed errors per layer, mapped once to `Problem` | — |
  | `garde`, `validator` | Value objects and `ValidationReport` | — |
  | `chrono`, `regex`, `itertools`, `once_cell` | `time`, hand-written parsers, the standard library | — |
  | `tracing` | `--verbose` writes plain diagnostic lines to stderr through one small `Diagnostics` type | ~30 lines |

- **Development-only**: `cucumber` 0.23, `assert_cmd` 2.2, `trycmd` 1.2, `tempfile` 3.27.
  Not compiled into `trcli`.
- **The test for adding a crate**: would our own version exceed about 60 lines, or be easy
  to get subtly wrong (Unicode, cryptography, time zones, SQL)? If neither, we write it.
- **A stated fallback**: if `paths.rs` meets an operating-system edge case it handles badly
  (redirected folders on Windows), `directories` is added then, with the reason recorded.
- **Spike 1**: confirm SeaORM 2.0's feature names for SQLite with a bundled library, `time`
  and `uuid` support, and a tokio runtime; confirm it runs on a current-thread runtime.

## 4. Storage: SQLite in a `.trcli` directory

- **Decision**: A workspace is a directory containing `.trcli/`:

  ```text
  <workspace>/.trcli/
  ├── trcli.db        the database (location configurable: storage.path)
  ├── config.toml     workspace settings
  ├── audit.head      sequence and hash of the last audit entry, outside the database
  └── backups/        copy of trcli.db taken before each upgrade
  ```

  Connection settings: write-ahead journal, foreign keys on, a busy timeout of 5 s,
  write transactions begun in immediate mode.
- **Rationale**: Chosen by the user. One file makes a workspace easy to copy; the journal
  mode lets a second `trcli` read while one writes; immediate-mode write transactions with
  a busy timeout give FR-009's "two commands at once never leave it inconsistent" — the
  second waits, or reports the workspace as busy. SQLite's transactions give "wholly present
  or wholly absent" on power loss (SC-008).
- **Nothing inside depends on where the workspace is** (FR-005): the database holds no
  absolute path to itself, and paths recorded by features are relative to the workspace
  root when they lie inside it.
- **Alternatives considered**: Plain files per record (readable, but integrity,
  cross-record queries, and atomic change would be rebuilt by hand); sqlx directly (more
  SQL to write and review for the same result).

## 5. Finding the workspace

- **Decision**: `WorkspaceLocator` resolves, in order: `--workspace <dir>`; the
  `TRCLI_WORKSPACE` environment variable (the "session" of FR-003); the nearest `.trcli/`
  at or above the current directory; `default_workspace` from the user's settings. It
  returns the workspace and *how* it was found, so the tool can say when it used the
  default. Nested workspaces resolve to the nearest.
- **Rationale**: FR-003 word for word; the nearest-ancestor rule is the one researchers
  know from version control.
- **Commands that need no workspace** (`init`, `help`, `completions`, `config --user`,
  `--version`) skip the locator; all others fail with the workspace exit code and an
  explanation (FR-004).

## 6. Record identity and the record index

- **Decision**:
  - Every record has `id` (UUID v7) and a **handle**: a prefix of 2–4 letters for its kind,
    a hyphen, and a code of 4 Crockford base-32 characters taken from the random part of
    the id, lengthened on collision (`ref-7k3f`).
  - A table `record` (the *record index*) holds one row per record of any kind: id, kind,
    handle, a display name, a normalized search key, created, changed, and deleted-at. A
    feature's own table has the same id as its primary key.
  - Deleting a record removes the feature's row and marks the index row deleted; the handle
    is thereby never given out again (FR-010).
  - References typed by the user are resolved against the index: exact handle, else unique
    prefix among the kinds the command accepts, case-insensitively; several matches list
    them; none suggests the closest by edit distance (FR-011).
- **Rationale**: Four requirements are met by this one table: handles unique across all
  kinds and never reused (FR-010); tags, notes, and links able to point at any record with
  database-enforced integrity (FR-015 to FR-017); cheap counts per kind (FR-006); and a
  feature referring to another's record by kind and identity alone (FR-019, FR-072).
- **Alternatives considered**: Polymorphic `(kind, id)` columns with no index table — no
  foreign keys, so "nothing left dangling" would rest on every feature remembering to clean
  up. Sequential numbers as names — clash when copies of a workspace are merged.

## 7. How a feature plugs in (FR-068)

- **Decision**: A feature registers a `RecordKindDescriptor` for each kind it owns: the
  kind's name, handle prefix, how to produce a display name and search key, and a
  `DeletionPolicy` (what else refers to a record, and what blocks deleting it). The
  registry is filled in the composition root. From a descriptor alone the foundation
  provides: handle assignment, reference resolution, `tag`, `note`, `link`, guarded `rm`,
  listing options (filter by tag, search, sort, limit), both output forms for the common
  fields, counts in `workspace show`, and audit entries.
- **What a feature still writes**: its aggregate and rules, its own fields for `add`,
  `edit`, `show`, and `list`, and its repository.
- **Proof**: two *sample kinds* (`specimen` and `sample-note`) live in the application crate
  under `src/sample/`, compiled only with the `sample-kind` cargo feature, which no release
  build enables; the BDD suite builds with it, since it tests the compiled binary. Their
  tables come from a separate migration set applied after the foundation's only under that
  feature and not counted in the workspace format version. They are exercised by the
  scenarios of user stories 2 and 8. If it
  needs a change to foundation code to work, FR-068 is not met.
- **Rationale**: Open/closed made concrete: adding a kind is adding a registration.
- **Alternatives considered**: A derive macro generating commands per kind — less code per
  feature, but a proc-macro crate to maintain and behaviour hidden from the reader, against
  the readability goal. A base trait with default methods — pulls storage concerns into the
  domain.

## 8. Validation in three places, one report

- **Decision**:
  1. **Syntax** — clap value parsers and its own messages for unknown commands and options.
  2. **Boundary** — every use case takes a *command* built from raw input by a constructor
     that checks every field and returns a `ValidationReport` holding **all** problems.
     Checks that need storage (does that record exist? is that name taken?) are made by the
     handler through ports and added to the same report before anything is written.
  3. **Invariants** — value objects can only be built through a checking constructor;
     aggregates enforce rules across fields.
  A `FieldProblem` carries the field as the user typed it, the value, what is wrong, what
  is expected, and an example or the list of choices (FR-022). Warnings are a separate
  list: shown, or turned into a question when the owning specification says so (FR-025).
- **Rationale**: Constitution principle V and FR-020 to FR-027. One report type means every
  feature's problems look alike in text and in JSON.
- **Alternatives considered**: A validation derive crate (validates after construction, so
  invalid values exist first, and adds a domain dependency); validating in clap only
  (cannot cover files, imports, or rules between values).

## 9. Output, problems, and exit codes

- **Decision**:
  - A handler returns a **view model**: a plain value that implements `Serialize` (for the
    structured form) and `Render` (for people). Both forms come from the same value, so
    their content cannot diverge (SC-005).
  - `--output json` prints one envelope — `ok`, `data`, `warnings`, or `ok: false`,
    `error` — defined in [contracts/output-and-exit-codes.md](./contracts/output-and-exit-codes.md).
  - `Outcome` is an enum with exactly the meanings of FR-032, mapped once to exit codes
    0, 1, 2, 3, 4, 5, 6, 7, and 130.
  - `Problem` has a stable `code`, a message, and details. Each layer's error type maps to
    a `Problem` in one place; features add problem codes by registration.
  - Results go to stdout; everything else to stderr (FR-031).
  - Colour: `auto` (only on a terminal, and not when `NO_COLOR` is set or `CLICOLOR=0`),
    `always`, `never`. A `Theme` maps meanings (success, warning, error, handle, heading,
    muted) to styles and is configurable. A `SymbolSet` has a unicode and an ascii
    implementation. Meaning is always also in words.
- **Rationale**: FR-028 to FR-034, and the user's request for coloured output.
- **Alternatives considered**: Rendering inside handlers (mixes deciding with presenting,
  and makes the two forms drift).

## 10. Never waiting, always interruptible

- **Decision**:
  - `Prompter` is a port with three implementations chosen at start-up: *terminal* (asks),
    *assume-yes* (`--yes`), and *refuse* (not a terminal, or `--no-input`), which returns
    the confirmation-required outcome at once (FR-035). No code path reads standard input
    directly.
  - `Progress` is a port; the terminal implementation draws on stderr only when stderr is a
    terminal, after ~200 ms, and erases itself.
  - Ctrl-C is awaited alongside the running handler. On interruption the handler's unit of
    work is dropped, which rolls back; the outcome is "interrupted" (130) with what was and
    was not done (FR-036).
- **Rationale**: SC-007 and SC-008 are properties of the structure, not of each command.

## 11. Settings

- **Decision**: A **settings registry** in the application crate: each feature registers
  its `SettingDefinition`s (key, kind of value, allowed values, default, scope, one line of
  meaning). `SettingsSource` adapters read the user file, the workspace file, and
  `TRCLI_*` variables; flags are applied last. Unknown keys and invalid values in any
  source are errors naming the source and the key (FR-043); a key in the wrong scope is
  ignored with a warning (FR-044). `config list` shows each value and its source (FR-041).
  User file: `$XDG_CONFIG_HOME/trcli/config.toml` or `~/.config/trcli/config.toml` on
  Linux; `~/Library/Application Support/trcli/config.toml` on macOS;
  `%APPDATA%\trcli\config.toml` on Windows.
- **Rationale**: "Configurable", with one place and one rule. The registry is again
  open/closed: features add settings without the foundation knowing them.
- **Alternatives considered**: A configuration framework (two crates and their
  dependencies to do what ~150 lines do); settings in the database (cannot be read before
  the database is located).

## 12. Audit trail and telemetry

- **Decision**:
  - `AuditLog::record` is called by handlers inside the unit of work, so a change and its
    entry commit together or not at all (FR-047). Handlers cannot commit without passing
    through `UnitOfWork`, which is where this is enforced.
  - Entries have a strictly increasing `sequence`; order never comes from the clock
    (FR-051). Each stores `hash = SHA-256(previous hash ‖ canonical form of the entry)`.
  - `audit.head`, a small file beside the database, holds the sequence and hash of the last
    entry and is rewritten after each commit. `audit verify` recomputes the chain and
    compares the end with the head file, so an edit in the middle **and** a removal at the
    end are both reported (FR-048 "altered or shortened").
  - There is no update or delete operation for audit entries in any layer.
  - The actor is `researcher.name` from settings, else the operating system's user name,
    else the placeholder `unknown` with a hint to set a name (FR-050).
  - Telemetry is written by one decorator around command dispatch — command name,
    duration, outcome — to its own table, only when `telemetry.enabled` (FR-052). No code
    path sends it anywhere.
  - Values of settings and fields marked secret are never written (FR-054); the foundation
    itself has none, and the marking exists for the features that will.
- **Limit, stated honestly**: someone who rewrites the database's chain *and* the head file
  consistently is not detected. That needs an outside reference and is out of scope, as the
  spec's assumptions say.
- **Alternatives considered**: Database triggers writing the audit rows (hides behaviour
  from the reader and cannot describe a change in the domain's words); signing entries with
  a key (the key would sit beside the data it protects).

## 13. Format versions and upgrades

- **Decision**: The format version is a number in the `workspace` row that increases with
  each **released** change to how a workspace is stored; migrations added between releases
  belong to the next format. On opening: newer than the binary knows →
  refuse with the workspace outcome; older → every command that would write says an upgrade
  is needed and exits; reading commands still work. `trcli workspace upgrade` copies
  `trcli.db` to `backups/`, applies migrations in one transaction, and on failure restores
  the copy (FR-007). `tests/fixtures/formats/` keeps a small workspace of every released
  format, added at each release; a test upgrades each and compares record counts and audit
  verification (FR-075). Before the first release there is no fixture, and the test covers
  only rollback on failure and refusal of a newer format.
- **Damaged workspaces** (FR-008): if the database cannot be opened, fails SQLite's
  integrity check, or lacks expected tables, the tool reports which and where, suggests
  restoring from `backups/`, and writes nothing.
- **Rationale**: Researchers will not trust years of work to a tool that upgrades without
  asking or without a way back.

## 14. Tests: TDD, and BDD against the binary

- **Decision**:

  | Level | What | Where |
  |-------|------|-------|
  | Unit | Value objects, rules, settings layering, validation, rendering, handlers with fakes | in each crate |
  | Contract | Each port against its fake and its real adapter, same suite | `trcli-infra-*/tests` |
  | Behaviour | Every acceptance scenario of the spec as Gherkin, against the built binary in a temporary directory | `tests/features/**`, `tests/bdd` |
  | Documentation | Examples in `docs/usage/*.md` | `tests/usage.rs` |
  | Structure | Layering; every command has help with an example; every command group has a guide | `tests/layering.rs`, `tests/help_examples.rs` |
  | Upgrade | Every earlier format upgrades without loss | `tests/fixtures/formats` |

  - **Generic steps**: "Given a workspace", "When I run `…`", "Then the exit code is N",
    "Then stdout contains / is JSON with …", "Then stderr mentions …", "Then nothing was
    changed". Later features write scenarios and almost no step code.
  - **Determinism**: a `test-clock` cargo feature, enabled only for the BDD build, lets the
    binary take "now" and identifier seeds from the environment.
  - **Order in a phase**: scenarios first (red); then inside-out, a failing test before each
    piece; scenarios green last.
- **Rationale**: The user asked for TDD and for BDD "by testing the CLI itself".
- **Spike 2**: confirm cucumber 0.23 runs with a harness-less test target on Windows, and
  measure the suite's time; if it is too slow, scenarios are sharded per feature directory.

## 15. Code a human can read

- **Decision** (enforced by the build, not by goodwill):
  - `#![deny(missing_docs)]` and `#![deny(clippy::missing_docs_in_private_items)]` in every
    crate; `cargo doc --document-private-items` with warnings as errors.
  - Readability lints denied: function length (limit 50 lines), cognitive complexity (10),
    argument count (5), wildcard imports.
  - Each module file opens with a paragraph: what it contains, which requirements it
    implements, what it deliberately does not do.
  - Doc comments say what an item is *for*, in the spec's words; inline comments say *why*
    a line is there, citing the FR number when a rule comes from the spec. Comments do not
    restate code.
  - Names are whole words from the spec's vocabulary; no abbreviations but `id`.
  - Early returns over nesting; a function either decides or performs I/O.
- **Rationale**: "All the code should be commented for a human to understand, use readable
  code." A standard that a machine does not check decays.

## 16. What changed from the earlier plan

| Earlier (`specs/001-research-workspace/research.md`) | Now | Why |
|------------------------------------------------------|-----|-----|
| Seven crates | Five | Bibliography and catalogue crates belong to their features |
| Multi-thread tokio runtime | Current-thread; `rt`, `macros`, `signal` only | Async only where necessary |
| `figment` + `directories` | `toml` + our own layering and paths | Minimum libraries |
| `comfy-table`, `indicatif` | Our own rendering and spinner | Minimum libraries |
| `tracing` | A small `Diagnostics` writer | Minimum libraries |
| `rstest`, `proptest` | Plain table-driven tests | Minimum libraries |
| `blake3`, `biblatex`, `hayagriva`, `reqwest` | Not in the foundation | They arrive with datasets, literature, and lookup |
| Polymorphic `RecordRef` columns for tags, notes, links | A record index with real foreign keys | FR-017 as a constraint, FR-010 across kinds |
| Audit chain only | Chain plus a head file | FR-048 also asks to detect a shortened trail |
| Auto-migrate after copying | Upgrade only when asked | FR-007 |
| An `ArtifactStore` port for referenced files | Deferred to the first feature that reads files | No foundation feature needs it |
| Private items undocumented | Private items documented by lint | "All the code commented" |

## 17. Risks

| Risk | Mitigation |
|------|------------|
| SeaORM 2.0 feature flags or current-thread behaviour differ from expectation | Spike 1 in Phase 1, before anything is built on it |
| Our own platform paths miss a Windows edge case | Covered by CI on Windows; stated fallback to `directories` |
| Start-up over 100 ms because of opening the database | Open lazily; commands that need no workspace never touch it; measure in CI |
| BDD suite becomes slow as features add scenarios | Spike 2; shard by feature directory; unit tests carry the detail |
| The documentation lints slow the first phases | On from the first commit, so there is never a backlog |
| A later feature needs something the registry cannot express | The sample kind proves the common case; the feature contract is versioned and extended deliberately |
