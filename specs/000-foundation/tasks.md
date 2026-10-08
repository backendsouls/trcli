---

description: "Task list for the TRCLI foundation"
---

# Tasks: TRCLI Foundation and Architecture

**Input**: Design documents from `/specs/000-foundation/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: Included. The constitution makes test-first development non-negotiable, and the plan asks for TDD and for BDD against the compiled binary. In every phase the test tasks come first and MUST be seen to fail before the implementation tasks that follow them.

**Organization**: Tasks are grouped by user story. What several stories need (the unit of work with audit recording, settings sources and files, validation types, renderers) is in Phase 2, so that the stories can then follow the spec's priority order. These phases are the delivery plan referred to by `plan.md`.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1 … US8)
- Every task names the file or files it creates or changes

## Path Conventions

- Rust Cargo workspace: `crates/trcli-domain`, `crates/trcli-application`, `crates/trcli-infra-sqlite`, `crates/trcli-infra-system`, `crates/trcli-cli`
- Black-box tests of the binary at the repository root: `tests/features/foundation/*.feature`, `tests/bdd/`, `tests/*.rs`
- Usage guides: `docs/usage/`; contributor guides: `docs/contributing/`

## Rules that apply to every task

- **Document everything**: every module, type, function, field, and constant gets a doc comment saying what it is for; the build fails otherwise. Each module file opens with what it contains, which requirements it implements, and what it deliberately does not do. Inline comments explain why, citing the FR number when a rule comes from the spec.
- **Stay in the layer**: rules in `trcli-domain` (no I/O, no async); use cases and ports in `trcli-application`; SeaORM only in `trcli-infra-sqlite`; operating-system differences only in `trcli-infra-system`; clap and rendering only in `trcli-cli`; concrete adapters named only in `crates/trcli-cli/src/compose.rs`.
- **Async only where necessary**: on ports that reach storage, the handlers that await them, and signal handling.
- **No new library** without a written reason added to `specs/000-foundation/research.md` §3.
- **Tag every scenario**: each Gherkin scenario carries a tag naming the user story and acceptance scenario it automates (`@US3-07`); `tests/scenario_coverage.rs` fails when a scenario of the spec has no tagged counterpart.
- **One pull request per phase**, stacked on the previous one; never commit to `main` directly.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: The Cargo workspace, the lints that make documentation and readability part of the build, and CI on three systems.

- [X] T001 Create the Cargo workspace manifest `Cargo.toml` with members `crates/trcli-domain`, `crates/trcli-application`, `crates/trcli-infra-sqlite`, `crates/trcli-infra-system`, `crates/trcli-cli`; a `[workspace.dependencies]` table pinning clap 4.6, clap_complete 4.6, sea-orm 2.0, sea-orm-migration 2.0, tokio 1.53 (features `rt`, `macros`, `signal` only), time 0.3, uuid 1.27 (feature `v7`), serde 1, serde_json 1, toml 1.1, sha2 0.11, thiserror 2.0, unicode-normalization 0.1, unicode-width 0.2, terminal_size 0.4, and dev-dependencies cucumber 0.23, assert_cmd 2.2, trycmd 1.2, tempfile 3.27; and `rust-toolchain.toml` pinning channel 1.96
- [X] T002 Create the five crates with edition 2024 and an empty documented `lib.rs` (or `main.rs` for the CLI, binary name `trcli`): `crates/trcli-domain/Cargo.toml` (deps: thiserror, time, uuid, unicode-normalization only), `crates/trcli-application/Cargo.toml` (deps: trcli-domain, serde; features `test-support`, `sample-kind`), `crates/trcli-infra-sqlite/Cargo.toml` (deps: trcli-application, trcli-domain, sea-orm, sea-orm-migration, sha2), `crates/trcli-infra-system/Cargo.toml` (deps: trcli-application, trcli-domain, toml), `crates/trcli-cli/Cargo.toml` (deps: all four crates, clap, clap_complete, tokio, serde_json, unicode-width, terminal_size; features `test-clock`, `sample-kind`)
- [X] T003 [P] Add shared lints in `Cargo.toml` under `[workspace.lints]` and inherit them in every crate manifest: deny `missing_docs`, `clippy::missing_docs_in_private_items`, `clippy::too_many_lines`, `clippy::cognitive_complexity`, `clippy::too_many_arguments`, `clippy::wildcard_imports`, `clippy::enum_glob_use`; create `clippy.toml` with `too-many-lines-threshold = 50`, `cognitive-complexity-threshold = 10`, `too-many-arguments-threshold = 5`
- [X] T004 [P] Add `rustfmt.toml` (edition 2024, max width 100) and `.gitignore` (`target/`, `*.db`, `.trcli/` under `tests/tmp`)
- [X] T005 [P] Create `.github/workflows/ci.yml`: a matrix over `ubuntu-latest`, `macos-latest`, `windows-latest` running `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo doc --workspace --no-deps --document-private-items` with `RUSTDOCFLAGS=-D warnings`, `cargo test --workspace`, `cargo test --test bdd --features test-clock,sample-kind`, and a step asserting that a default build of `trcli` has no `specimen` or `sample-note` subcommand
- [X] T006 [P] Create `.github/pull_request_template.md` with the acceptance gates of `specs/000-foundation/contracts/feature-contract.md`: link to the failing test run before the change, scenarios added, invalid-input scenarios, usage guide, help example, three systems green
- [X] T007 Spike 1 (research.md §3): in `crates/trcli-infra-sqlite/tests/spike_seaorm.rs`, open a bundled SQLite database through SeaORM 2.0 on a tokio current-thread runtime with `time` and `uuid` column support, create a table, insert and read a row; record the exact feature flags that work in `specs/000-foundation/research.md` §3 and fix `Cargo.toml` accordingly
- [ ] T008 Spike 2 (research.md §14): create `tests/bdd/main.rs` as a harness-less cucumber test target with one trivial feature file `tests/features/foundation/00_smoke.feature` ("When I run `trcli --version` Then the exit code is 0"); confirm it runs on all three CI systems and record the measured time in `specs/000-foundation/research.md` §14

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: What every user story needs before it can be written: the BDD steps, the fixed vocabulary of outcomes and problems, the two output forms, validation, the ports with their fakes, the database with a unit of work that records audit entries, and read-only settings.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete. Within it, tests are written first and must fail before the code that satisfies them.

### BDD harness

- [X] T009 Implement the cucumber `World` in `tests/bdd/world.rs`: a temporary directory per scenario (tempfile), the path to the built `trcli` binary (assert_cmd), the last command's stdout, stderr, and exit code, and environment overrides including `TRCLI_TEST_NOW` and `TRCLI_TEST_ID_SEED`
- [X] T010 Implement generic steps in `tests/bdd/steps.rs`: `Given a workspace named {string}`, `Given I am in the directory {string}`, `Given the environment variable {word} is {string}`, `Given standard input is not a terminal`, `When I run {string}`, `Then the exit code is {int}`, `Then stdout contains {string}`, `Then stdout does not contain {string}`, `Then stderr contains {string}`, `Then stdout is empty`, `Then stdout is JSON where {string} is {string}`, `Then the output has no colour codes`, `Then nothing was changed` (compares, before and after the command, the number of rows in `audit_entry`, a hash of the contents of every shared table read directly from `.trcli/trcli.db` (local tables such as `telemetry_record` are left out, since every command adds to them), and the bytes of `.trcli/config.toml`)

### Tests first (must fail)

- [X] T011 [P] Unit tests for the shared text value objects in `crates/trcli-domain/src/shared/text.rs` (test module): `Title` is Text(1..500), `Name` is Text(1..200), `LongText` is Text(0..20000); surrounding whitespace is trimmed; control characters other than line break and tab are rejected; text is otherwise kept exactly as written, in any script
- [X] T012 Unit tests for `SearchKey` in `crates/trcli-domain/src/shared/text.rs` (test module): lower case, accents and other combining marks removed, punctuation collapsed to single spaces; "Ação" and "acao" give the same key; "東京" is preserved
- [X] T013 [P] Unit tests for `ValidationReport` and `FieldProblem` in `crates/trcli-domain/src/shared/problem.rs` (test module): several problems accumulate in order; warnings are a separate list; a report with no errors is valid; `FieldProblem` carries `field`, `value`, `problem`, `expected`, and `example` or `choices`
- [X] T014 [P] Unit tests for `Outcome` in `crates/trcli-application/src/outcome.rs` (test module): exit codes are exactly `Success` 0, `Failure` 1, `InvalidInput` 2, `NotFound` 3, `WorkspaceProblem` 4, `Refused` 5, `CheckFailed` 6, `OperationFailed` 7, `Interrupted` 130
- [X] T015 [P] Unit tests for the JSON envelope in `crates/trcli-cli/src/render/json.rs` (test module): success is `{"ok":true,"data":…,"warnings":[…]}`; failure is `{"ok":false,"error":{"code":…,"message":…,"details":…}}`; absent optional values are `null`, never omitted; output ends with one newline
- [X] T016 [P] Unit tests for width and shortening in `crates/trcli-cli/src/render/width.rs` (test module): display width counts accented and wide characters correctly; shortening to N columns ends with `…` (or `...` with the ascii symbol set) and never splits a character
- [X] T017 [P] Unit tests for colour selection in `crates/trcli-cli/src/render/theme.rs` (test module): `auto` gives colour only when the stream is a terminal and `NO_COLOR` is unset and `CLICOLOR` is not `0`; `always` forces it; `never` removes it
- [X] T018 [P] Contract test suite for `UnitOfWork` and `AuditLog` in `crates/trcli-application/src/testing/contract_uow.rs`: a change and its audit entry are both present after commit; neither is present when the unit is dropped without commit; sequences start at 1 and increase by 1 with no gap; each entry's `previous_hash` equals the hash before it
- [X] T019 [P] Unit tests for the audit hashing rule in `crates/trcli-domain/src/governance/audit.rs` (test module): `hash` is SHA-256 over `previous_hash` and the canonical form (fields in the documented order, each as UTF-8 with a length prefix); the first entry's `previous_hash` is zeros; changing any field changes the hash; two different entries never share a canonical form
- [X] T020 [P] Unit tests for settings layering in `crates/trcli-application/src/settings/layers.rs` (test module): precedence is default ‹ user file ‹ workspace file ‹ `TRCLI_*` variables ‹ command-line option; each effective value reports its source; an unknown key or an invalid value in any source is an error naming the source and the key; a key in a scope it does not have is ignored with a warning
- [X] T021 [P] Unit tests for the system adapters in `crates/trcli-infra-system/src/{actor.rs,settings_files.rs}` (test modules): the actor falls back from `researcher.name` to `USER`/`USERNAME` to `unknown`; a missing or empty settings file yields no values; an unreadable one is an error naming the file; writing one key preserves every other key and never leaves a half-written file
- [X] T022 [P] Test for the connection in `crates/trcli-infra-sqlite/tests/connection.rs`: foreign keys are enforced; the journal mode is write-ahead; a second writer waits up to `storage.busy_timeout_ms` and then fails with a busy error
- [X] T023 Write `tests/features/foundation/00_skeleton.feature`: `trcli` with no arguments prints short help and exits 0; `trcli --version` prints the version and the workspace format; `trcli nosuchcommand` exits 2 and suggests the nearest command; a command that needs a workspace run outside one exits 4; the same with `--output json` prints one JSON document with `ok` false and still exits 4

### Domain kernel

- [X] T024 [P] Implement `Title`, `Name`, `LongText`, `TagName`, `Relation`, and `SearchKey` in `crates/trcli-domain/src/shared/text.rs` with checking constructors; `TagName` is 1–50 characters of `a-z 0-9 - _` after removing surrounding whitespace and lowering upper case, and rejects anything else with the rule; `Relation` is Text(1..50) with default `related`
- [X] T025 [P] Implement `FieldProblem`, `ValidationReport`, and the `Warning` list in `crates/trcli-domain/src/shared/problem.rs`
- [X] T026 [P] Implement `RecordId` (UUID v7), `RecordKind` (name of 2–40 characters of `a-z -`, handle prefix of 2–4 letters), `RecordRef`, and `Handle` (`<prefix>-<code>`, code of 4–12 Crockford base-32 characters, lower case, compared ignoring case) in `crates/trcli-domain/src/shared/record.rs`, with unit tests in the same file written first
- [X] T027 [P] Implement `AuditEntry`, `AuditAction` (`create`, `update`, `delete`, `status`, `link`, `unlink`, `tag`, `untag`, `note`, `import`, `export`, `run`, `confirm`, `setting`, `upgrade`), the canonical form, and the hashing rule in `crates/trcli-domain/src/governance/audit.rs`; `sha2` is used from the SQLite crate, so the domain exposes the canonical bytes and takes the digest as an argument
- [X] T028 [P] Implement `SettingKey` (dotted segments of `a-z 0-9 _`), `SettingValue`, `SettingDefinition` (key, value kind, default, scope `user`/`workspace`/`both`, summary; registering a definition marked secret is an error), and `Scope` in `crates/trcli-domain/src/settings/mod.rs`

### Application: vocabulary and ports

- [X] T029 Implement `Outcome` and `Problem` (`code`, `message`, `details`, `changed`, `next_step`) and a registry of problem codes in `crates/trcli-application/src/outcome.rs`; register the foundation's codes listed in `specs/000-foundation/contracts/output-and-exit-codes.md`
- [X] T030 Define the ports in `crates/trcli-application/src/ports/` as small traits with `async fn` where they reach storage: `Storage` and `UnitOfWork` (`unit_of_work.rs`), `AuditLog` (`audit.rs`), `Clock`, `IdGenerator`, `ActorProvider` (`environment.rs`), `SettingsSource` (`settings.rs`), `Prompter` and `Progress` (`interaction.rs`); each trait documented with what the caller needs it for
- [X] T031 Implement in-memory fakes of every port defined so far in `crates/trcli-application/src/testing/` behind the `test-support` feature, including a fixed `Clock` and a seeded `IdGenerator`; make the contract suite of the unit of work pass against the fakes
- [X] T032 Implement the settings registry and layering in `crates/trcli-application/src/settings/{registry.rs,layers.rs}` and register the foundation's definitions: `default_workspace`, `storage.path` (default `.trcli/trcli.db`), `storage.busy_timeout_ms` (0–60000, default 5000), `output.format` (`human`/`json`), `output.color` (`auto`/`always`/`never`), `output.symbols` (`unicode`/`ascii`), `output.page_size` (1–1000, default 50), `output.date_format` (`iso` only, default `iso`), the `theme.*` styles, `researcher.name` (text 1–200), `telemetry.enabled` (default true)

### Adapters

- [X] T033 Implement the connection in `crates/trcli-infra-sqlite/src/connection.rs`: open the database at a path with write-ahead journal, foreign keys on, a busy timeout from `storage.busy_timeout_ms`, and write transactions begun in immediate mode; open lazily so commands that need no workspace never touch it
- [X] T034 Create the migration runner and the first migration in `crates/trcli-infra-sqlite/src/migrations/m0001_foundation.rs` with tables `workspace` (id, name, description, format_version, created_at) and `audit_entry` (sequence primary key, at, actor, action, kind, record_id, handle, display_name, changes as JSON text, previous_hash, hash) and indexes `audit_entry(record_id)`, `audit_entry(at)`
- [X] T035 Implement `UnitOfWork` and `AuditLog::record` in `crates/trcli-infra-sqlite/src/{unit_of_work.rs,stores/audit.rs}` so that an entry is appended inside the same transaction as the change, with `sequence` = last + 1 and the hash chain computed with `sha2`; run the contract suite from the application crate against it in `crates/trcli-infra-sqlite/tests/contract_uow.rs`
- [X] T036 [P] Implement platform paths in `crates/trcli-infra-system/src/paths.rs`: the user settings file is `$XDG_CONFIG_HOME/trcli/config.toml` or `~/.config/trcli/config.toml` on Linux, `~/Library/Application Support/trcli/config.toml` on macOS, `%APPDATA%\trcli\config.toml` on Windows; unit tests set the environment per platform
- [X] T037 [P] Implement `Clock`, `IdGenerator`, and `ActorProvider` in `crates/trcli-infra-system/src/{clock.rs,ids.rs,actor.rs}`: the actor is `researcher.name`, else the system user name from `USER` or `USERNAME`, else the placeholder `unknown`; under the `test-clock` feature the clock reads `TRCLI_TEST_NOW` and ids are seeded from `TRCLI_TEST_ID_SEED`
- [X] T038 [P] Implement `SettingsSource` adapters in `crates/trcli-infra-system/src/settings_files.rs` for the user file, the workspace file `.trcli/config.toml`, and `TRCLI_*` variables (`TRCLI_OUTPUT_COLOR` → `output.color`); a missing or empty file yields no values; an unreadable file is an error naming the file
- [X] T039 Implement writing of settings files in `crates/trcli-infra-system/src/settings_files.rs`: update one key in a TOML file preserving the rest, written to a temporary name and renamed

### CLI skeleton

- [X] T040 Implement the renderers in `crates/trcli-cli/src/render/`: the `Render` trait and human layout helpers (`human.rs`), the JSON envelope (`json.rs`), `Theme` mapping success, warning, error, handle, heading, muted to styles through anstyle (`theme.rs`), `SymbolSet` with unicode and ascii implementations (`symbols.rs`), display width and shortening (`width.rs`), and problem rendering that names each value, what is wrong, what is expected, and ends with "Nothing was changed." when so (`problem.rs`)
- [X] T041 Implement the global options in `crates/trcli-cli/src/args/global.rs` exactly as in `specs/000-foundation/contracts/cli-conventions.md`: `--workspace`, `--output human|json`, `--color auto|always|never`, `-y/--yes`, `--no-input`, `-q/--quiet`, `-v/--verbose` (`--project` and `--all-projects`, also listed in that file, are added by `specs/003-research-projects` and are not built here); and `--version` printing the version and the workspace format it reads and writes
- [X] T042 Implement `crates/trcli-cli/src/main.rs` and `crates/trcli-cli/src/compose.rs`: start one current-thread tokio runtime, parse arguments, build adapters in `compose.rs` only, dispatch, render the view model or the problem to the right stream, and exit with the `Outcome`'s code; `trcli` with no arguments prints the short help and exits 0; a small `Diagnostics` writer prints `--verbose` lines to stderr
- [X] T043 Make `tests/features/foundation/00_skeleton.feature` pass

**Checkpoint**: `trcli` builds, answers in both forms, ends with the right exit code, and a change committed through the unit of work has its audit entry. User stories can now begin.

---

## Phase 3: User Story 1 - Create and find a workspace (Priority: P1) 🎯 MVP

**Goal**: A researcher creates a workspace, and from anywhere beneath it the tool finds it; outside one, the tool explains what to do; old, too-new, and damaged workspaces are handled safely.

**Independent Test**: Create a workspace, run a command from a subdirectory three levels down and confirm it is found; run a command from outside and confirm the explanation; try to create a second workspace in the same place and confirm it is refused.

### Tests for User Story 1 ⚠️ write first, must fail

> **Write these tests FIRST and see them FAIL before implementing.**

- [X] T044 [P] [US1] Write `tests/features/foundation/01_workspace.feature` with one scenario per acceptance scenario 1–14 of User Story 1 in `specs/000-foundation/spec.md` (create; found from beneath; outside explains and changes nothing; refuse where one exists; show; edit; two workspaces separate; named for a command and for a session; default workspace; nearest of nested; too new refuses; older needs upgrade and keeps a copy; damaged is reported; empty name or unwritable directory rejected), plus one for FR-005: after the workspace directory is renamed, commands run inside it still work
- [X] T045 [P] [US1] Unit tests for the locator in `crates/trcli-application/src/workspace/locate.rs` (test module): order is `--workspace`, then `TRCLI_WORKSPACE`, then the nearest `.trcli/` at or above the current directory, then `default_workspace`; the result says how the workspace was found; nested workspaces resolve to the nearest
- [X] T046 [P] [US1] Unit tests for workspace states in `crates/trcli-domain/src/workspace/mod.rs` (test module): format equal to known is usable; lower needs upgrade (reading allowed, writing refused); higher is too new (nothing may change); `name` is Name (Text 1..200) and required
- [X] T047 [P] [US1] Contract test suite for `WorkspaceStore` in `crates/trcli-application/src/testing/contract_workspace.rs`: creating where a workspace exists is refused and leaves it untouched; the workspace row round-trips name and description; a missing database, a failed integrity check, and missing tables are each reported as damage with what and where
- [X] T048 [P] [US1] Unit tests with fakes for the use cases in `crates/trcli-application/src/workspace/{init.rs,show.rs,edit.rs}` (test modules): `init` rejects an empty name and an unwritable directory and reports both together; `show` returns name, description, location, and format version (counts per kind are added in User Story 2); `edit` changes only what was named; each change records one audit entry
- [X] T049 [P] [US1] Tests in `crates/trcli-infra-sqlite/tests/upgrade.rs`: a failing migration leaves the database byte-identical to the copy taken before it; a workspace whose format is higher than the binary's is refused; a loop over every directory under `tests/fixtures/formats/` upgrades a copy and asserts record counts are unchanged and the audit trail verifies (it passes vacuously until the first release adds a fixture)

### Implementation for User Story 1

- [X] T050 [P] [US1] Implement `Workspace` and `FormatVersion` with the four opening states in `crates/trcli-domain/src/workspace/mod.rs`
- [X] T051 [P] [US1] Define the `WorkspaceLocator` and `WorkspaceStore` ports in `crates/trcli-application/src/ports/workspace.rs` and their fakes in `crates/trcli-application/src/testing/workspace.rs`
- [X] T052 [US1] Implement the use cases in `crates/trcli-application/src/workspace/`: `init.rs` (validated command: name required, directory writable, refuse where a workspace exists), `show.rs` (name, description, location, format version; counts per kind are added in User Story 2), `edit.rs` (name, description, and `researcher.name` written to the workspace settings), each recording an audit entry
- [X] T053 [US1] Implement the locator in `crates/trcli-infra-system/src/locator.rs` walking up from the current directory, and the guard in `crates/trcli-application/src/workspace/open.rs` that turns the opening state into `no_workspace`, `workspace_too_new`, `workspace_needs_upgrade`, or `workspace_damaged` problems (exit code 4) before any handler runs
- [X] T054 [US1] Implement `WorkspaceStore` in `crates/trcli-infra-sqlite/src/stores/workspace.rs`: create `.trcli/` with `trcli.db`, `config.toml`, and `backups/`; read and update the workspace row; report damage when the database cannot be opened, fails SQLite's integrity check, or lacks expected tables; run the contract suite against it in `crates/trcli-infra-sqlite/tests/contract_workspace.rs`
- [X] T055 [US1] Implement `upgrade.rs` and `check.rs` in `crates/trcli-application/src/workspace/` and the copy-before-upgrade adapter in `crates/trcli-infra-system/src/backup_copy.rs`: copy `trcli.db` to `.trcli/backups/` (written to a temporary name and renamed, never half-written), apply pending migrations in one transaction, restore the copy on failure, record an `upgrade` audit entry
- [X] T056 [US1] Implement the commands `init`, `workspace show`, `workspace edit`, `workspace upgrade [--check]`, `workspace check` in `crates/trcli-cli/src/args/workspace.rs` and `crates/trcli-cli/src/commands/workspace.rs` per `specs/000-foundation/contracts/cli-workspace.md`, each with help text and an example; the tool says which workspace it used when that was the default
- [X] T057 [US1] Create `tests/fixtures/formats/README.md` explaining that the format version increases with each released change to storage and that a fixture workspace of the outgoing format is added at each release
- [X] T058 [US1] Write `docs/usage/workspace.md` (purpose, commands, worked examples with their output, failures and how each ends) and make `tests/features/foundation/01_workspace.feature` pass

**Checkpoint**: A workspace can be created, found, shown, edited, upgraded, and checked. This is the MVP.

---

## Phase 4: User Story 2 - Work with any record the same way (Priority: P2)

**Goal**: Every kind of record gets a short name, the same actions, tags, notes, links, and deletion that leaves nothing dangling — by registering a descriptor.

**Independent Test**: With two kinds of record (the sample kinds `specimen` and `sample-note`), add one of each, refer to each by the start of its short name, tag one, note the other, link them, list each with a filter and a search, and delete one — confirming the link is listed first and gone afterwards.

### Tests for User Story 2 ⚠️ write first, must fail

> **Write these tests FIRST and see them FAIL before implementing.**

- [X] T059 [P] [US2] Write `tests/features/foundation/02_records.feature` with one scenario per acceptance scenario 1–14 of User Story 2, using the `specimen` and `sample-note` commands (build feature `sample-kind`); scenario 13 runs the same verbs on both kinds and compares their `--help` option lists
- [X] T060 [P] [US2] Unit tests for reference resolution in `crates/trcli-application/src/records/resolve.rs` (test module): exact handle wins; otherwise a unique prefix among non-deleted records of the accepted kinds, ignoring case; several matches give `ambiguous_reference` listing them; none gives `not_found` with up to three handles closest by edit distance
- [X] T061 [P] [US2] Unit tests for handle assignment in `crates/trcli-application/src/records/handles.rs` (test module): prefix of the kind, then 4 characters from the random part of the id; on collision 5 characters, and so on; a deleted record's handle is never assigned again
- [X] T062 [P] [US2] Unit tests for `Link` rules in `crates/trcli-domain/src/shared/link.rs` (test module): `from ≠ to`; the triple `(from, to, relation)` is unique and so is its reverse; a link is listed from both ends
- [X] T063 [P] [US2] Contract test suites in `crates/trcli-application/src/testing/contract_records.rs` for `RecordIndex`, `TagStore`, `NoteStore`, `LinkStore`: counts per kind exclude deleted rows; a tag, note, or link cannot be created for a record that does not exist; after a record is deleted no tagging, note, or link refers to it
- [X] T064 [P] [US2] Unit tests for the generic deletion use case in `crates/trcli-application/src/records/delete.rs` (test module) following the flow in `specs/000-foundation/data-model.md` §Deletion: a blocking policy refuses with the alternative; dependents are listed; without confirmation nothing changes; with it, the kind's rows, links, taggings, and notes are removed, the index row is marked deleted, and one audit entry is written — all in one unit of work
- [X] T065 [P] [US2] Unit tests for the kind registry in `crates/trcli-application/src/kinds.rs` (test module): a registered kind is found by name and by handle prefix; registering two kinds with the same name or the same prefix is an error naming both; a kind name must be 2–40 characters of `a-z -` and a prefix 2–4 letters
- [X] T066 [P] [US2] Unit tests with fakes for `crates/trcli-application/src/records/{tag.rs,note.rs,link.rs,list_options.rs}` (test modules): tagging twice leaves one tagging; a note needs at least one character; a link to the same record and a duplicate link are rejected; listing filters by tag, searches by `SearchKey`, sorts, and limits with a count of the rest; each change records one audit entry

### Implementation for User Story 2

- [X] T067 [P] [US2] Implement `Tag`, `Note` (`body` is LongText with at least 1 character), and `Link` in `crates/trcli-domain/src/shared/{tag.rs,note.rs,link.rs}`
- [X] T068 [P] [US2] Implement `RecordKindDescriptor` (kind name, handle prefix, display name, search key source, `DeletionPolicy`, removal) and the kind registry in `crates/trcli-application/src/kinds.rs` per `specs/000-foundation/contracts/feature-contract.md` §1; registering two kinds with the same name or prefix is an error
- [X] T069 [P] [US2] Define the ports `RecordIndex`, `RecordResolver`, `TagStore`, `NoteStore`, `LinkStore` in `crates/trcli-application/src/ports/records.rs` and their fakes in `crates/trcli-application/src/testing/records.rs`
- [X] T070 [US2] Add migration `crates/trcli-infra-sqlite/src/migrations/m0002_records.rs`: `record` (id, kind, handle unique including deleted rows, display_name Text 0..500, search_key, created_at, updated_at, deleted_at), `tag` (name unique), `tagging` (tag, record; unique pair), `note` (record, body, created_at), `link` (from, to, relation, created_at; unique triple), all record columns as foreign keys to `record`; indexes `record(kind, deleted_at)`, `record(search_key)`, `tagging(record)`, `tagging(tag)`, `note(record)`, `link(from)`, `link(to)`
- [X] T071 [US2] Implement the stores in `crates/trcli-infra-sqlite/src/stores/{record_index.rs,tags.rs,notes.rs,links.rs}` and run the contract suites against them in `crates/trcli-infra-sqlite/tests/contract_records.rs`
- [X] T072 [US2] Implement the generic use cases in `crates/trcli-application/src/records/`: `handles.rs`, `resolve.rs` with `similar.rs` (edit distance, ~25 lines), `tag.rs`, `note.rs`, `link.rs` (add, remove, list), `delete.rs`, and `list_options.rs` (filter by tag, search by `SearchKey`, sort, limit with a count of the rest) — each change recording its audit entry (`tag`, `untag`, `note`, `link`, `unlink`, `delete`)
- [X] T073 [US2] Implement two sample kinds in `crates/trcli-application/src/sample/` behind the application feature `sample-kind` (enabled by the CLI feature of the same name; never in a release build): `specimen` (prefix `spc`, one field `title: Title`) in `specimen.rs` and `sample-note` (prefix `smp`, one field `body: LongText`) in `sample_note.rs`; their tables are created by a separate migration set in `crates/trcli-infra-sqlite/src/sample/`, applied after the foundation's only under the feature and not counted in the workspace format version; both are registered through the descriptor only — no foundation file may name them
- [X] T074 [US2] Implement `crates/trcli-cli/src/shared_verbs.rs`: given a registered kind, build the clap subcommands `list` (with `--search`, `--tag`, `--sort`, `--desc`, `--limit`), `show`, `rm`, `tag <ref> <tag>... [--remove]`, `note <ref> <text>`, and their handlers; and `crates/trcli-cli/src/args/link.rs`, `crates/trcli-cli/src/commands/link.rs` for `link add|rm|list` and `tag list` per `specs/000-foundation/contracts/cli-link.md`
- [X] T075 [US2] Wire the `specimen` commands (`add --title`, `edit`, plus the shared verbs) and the `sample-note` commands (`add --body`, `edit`, plus the shared verbs) into the CLI behind the `sample-kind` feature in `crates/trcli-cli/src/commands/specimen.rs`, and add counts per kind to `workspace show` through `RecordIndex`, with a unit test written first in `crates/trcli-application/src/workspace/show.rs`
- [X] T076 [US2] Write `docs/usage/records.md` (short names and prefixes, the shared verbs, tags, notes, links, deletion and confirmation) and make `tests/features/foundation/02_records.feature` pass

**Checkpoint**: Any registered kind has the shared behaviour; the sample kind proves it without foundation code naming it.

---

## Phase 5: User Story 3 - Have every input checked before anything changes (Priority: P3)

**Goal**: Whatever is given to the tool is checked first; if anything is wrong, nothing changes and every problem is reported at once with what would be right.

**Independent Test**: Give a command three invalid values at once and confirm all three are reported together, each named with what is expected, and that nothing was stored.

### Tests for User Story 3 ⚠️ write first, must fail

> **Write these tests FIRST and see them FAIL before implementing.**

- [X] T077 [P] [US3] Write `tests/features/foundation/03_validation.feature` with one scenario per acceptance scenario 1–12 of User Story 3, driven through `init`, `workspace edit`, `link add`, and the `specimen` commands (three invalid values at once; a form with an example; a set with its choices; a record or file that does not exist; empty, too long, control characters; a rule between two values; an unknown command and option; a warning that proceeds; free text in several scripts kept as written; exit code 2)
- [X] T078 [P] [US3] Unit tests for the command-constructor pattern in `crates/trcli-application/src/validation.rs` (test module): a constructor given three invalid fields returns one report with three problems in the order the fields were given; storage-backed checks added by the handler land in the same report; nothing is written when the report has an error
- [X] T079 [P] [US3] Unit tests for problem rendering in `crates/trcli-cli/src/render/problem.rs` (test module): each line shows the option as typed, the value, what is wrong, and what is expected; an example appears for a required form and the choices for a set; the last line is "Nothing was changed."; the JSON form has `error.code` `validation_failed` and one `details` item per problem

### Implementation for User Story 3

- [X] T080 [US3] Implement the validation helpers in `crates/trcli-application/src/validation.rs`: a builder that collects `FieldProblem`s per field name as typed, helpers for required, length, range, one-of, date form, and rule-between-values, and the documented pattern every command constructor follows (returns a valid command or a report with every error)
- [X] T081 [US3] Extend `crates/trcli-cli/src/render/problem.rs` and `crates/trcli-cli/src/main.rs` so that clap's own errors (unknown command or option, missing value) are rendered as a `usage` problem with how the command is used and clap's suggestion, exit code 2, in both output forms
- [X] T082 [US3] Add the warning channel: `crates/trcli-application/src/validation.rs` returns warnings beside a valid command; `crates/trcli-cli/src/main.rs` prints them to stderr and includes them in the JSON envelope's `warnings`; a warning never changes a value
- [X] T083 [US3] Add `tests/invalid_input_gate.rs`: walk the clap command tree and fail if any argument or option of any command is not mentioned in at least one scenario tagged `@invalid` under `tests/features/` (the gate of `feature-contract.md`)
- [X] T084 [US3] Tag the rejection scenarios written so far with `@invalid`, add the missing ones for `init`, `workspace edit`, `link`, `tag`, `note`, and make `tests/features/foundation/03_validation.feature` and the gate pass

**Checkpoint**: Invalid input is rejected completely and helpfully everywhere, and the build fails if a new input has no rejection scenario.

---

## Phase 6: User Story 4 - Get answers fit for people and for programs (Priority: P4)

**Goal**: Readable answers at a terminal, plain ones when piped, a structured form on request, exit codes that mean something, progress for long work, and no waiting for an answer nobody can give.

**Independent Test**: Run a listing at a terminal and confirm it is aligned and coloured; pipe it and confirm there is no colour; ask for the structured form and confirm it is valid and complete; provoke each kind of failure and confirm each ends differently.

### Tests for User Story 4 ⚠️ write first, must fail

> **Write these tests FIRST and see them FAIL before implementing.**

- [X] T085 [P] [US4] Write `tests/features/foundation/04_output.feature` with one scenario per acceptance scenario 1–12 of User Story 4 (laid out and coloured at a terminal; no colour when piped; structured form equals the form for people; results and messages on separate streams; each failure kind ends differently; failure in the structured form; progress; interruption; colour and symbol preferences and `NO_COLOR`; a message with a next step; a limited listing with a count; dates in one form)
- [X] T086 [P] [US4] Unit tests for the three `Prompter` implementations in `crates/trcli-cli/src/prompt.rs` (test module): terminal asks and treats anything but yes as no; assume-yes answers yes without asking; refuse returns `confirmation_required` immediately without reading input
- [X] T087 [P] [US4] Unit tests for `Progress` in `crates/trcli-cli/src/progress.rs` (test module): nothing is drawn when the stream is not a terminal; nothing before ~200 ms; the indication is erased when done
- [X] T088 [P] [US4] A test in `tests/forms_agree.rs`: for every `specimen` command, the fields present in `--output json` `data` are the fields shown in the human form (SC-005)

### Implementation for User Story 4

- [X] T089 [US4] Implement `crates/trcli-cli/src/prompt.rs` with the terminal, assume-yes, and refuse implementations of `Prompter`, chosen once at start-up from `--yes`, `--no-input`, and whether standard input is a terminal; make it the only code that reads standard input
- [X] T090 [US4] Implement `crates/trcli-cli/src/progress.rs`: a spinner and a counter on stderr, shown only on a terminal after ~200 ms, erased on completion
- [X] T091 [US4] Implement interruption in `crates/trcli-cli/src/main.rs`: await Ctrl-C alongside the handler; on interruption drop the unit of work (rolling back), report what was and was not done, and end with `Interrupted` (130); end quietly when the reader of a pipe stops early
- [X] T092 [US4] Report a busy workspace: map SQLite's busy error after the timeout to the `workspace_busy` problem (exit code 4) in `crates/trcli-infra-sqlite/src/unit_of_work.rs`, saying the command can be tried again
- [X] T093 [US4] Add a deliberately slow command `specimen slow --seconds <n>` behind `sample-kind` in `crates/trcli-cli/src/commands/specimen.rs` that reports progress and holds a unit of work open, for the interruption, busy, and power-loss checks
- [X] T094 [US4] Implement date and time rendering in `crates/trcli-cli/src/render/human.rs`: dates as `YYYY-MM-DD`; instants in local time with the zone when it matters; RFC 3339 UTC in the structured form
- [X] T095 [US4] Make `tests/features/foundation/04_output.feature` pass

**Checkpoint**: Both output forms agree, every failure kind has its exit code, and nothing can hang or be left half-done.

---

## Phase 7: User Story 5 - Set the tool up my way (Priority: P5)

**Goal**: Preferences for the person, choices for a workspace, overrides for a session or a command — and a listing that shows every value and where it comes from.

**Independent Test**: Set a preference for yourself, a different value for one workspace, and an override for one command; list settings and confirm each value and its source; set an invalid value and confirm it is refused.

### Tests for User Story 5 ⚠️ write first, must fail

> **Write these tests FIRST and see them FAIL before implementing.**

- [X] T096 [P] [US5] Write `tests/features/foundation/05_settings.feature` with one scenario per acceptance scenario 1–11 of User Story 5 (personal; workspace over personal; session and command override without storing; list with sources; describe a setting; invalid value and unknown key refused; a bad file stops every command and names file and key; wrong scope ignored with a warning; unset falls back; everything works with no settings; no secrets)
- [X] T097 [P] [US5] Unit tests for the settings use cases in `crates/trcli-application/src/settings/commands.rs` (test module): `set` validates against the definition and writes to one file only; `unset` removes the value from that file; `get` returns meaning, allowed values, default, and the effective value with its source

### Implementation for User Story 5

- [X] T098 [US5] Implement the settings use cases `list`, `get`, `set [--user]`, `unset [--user]`, `path` in `crates/trcli-application/src/settings/commands.rs`, each `set` and `unset` recording a `setting` audit entry when it changes the workspace file
- [X] T099 [US5] Implement `config list|get|set|unset|path` in `crates/trcli-cli/src/args/config.rs` and `crates/trcli-cli/src/commands/config.rs` per `specs/000-foundation/contracts/cli-workspace.md` and `configuration.md`, with help and examples
- [X] T100 [US5] Apply the settings everywhere they belong: `output.format`, `output.color`, `output.symbols`, `output.page_size`, and `theme.*` in `crates/trcli-cli/src/render/`; `storage.path` and `storage.busy_timeout_ms` in `crates/trcli-infra-sqlite/src/connection.rs`; `default_workspace` in the locator
- [X] T101 [US5] Write `docs/usage/config.md` (the places settings come from and their order, every foundation setting with allowed values and default, examples) and make `tests/features/foundation/05_settings.feature` pass

**Checkpoint**: Every setting is visible with its source and adjustable at the right level.

---

## Phase 8: User Story 6 - Know everything that happened (Priority: P6)

**Goal**: The record of every change can be looked through, checked for tampering, and handed over; local telemetry about the tool's use can be viewed and turned off.

**Independent Test**: Make a known series of changes, look up the record for one item and for a range of dates and confirm every change is there; alter one entry outside the tool and confirm the check reports it; turn telemetry off and confirm none is recorded while the trail continues.

### Tests for User Story 6 ⚠️ write first, must fail

> **Write these tests FIRST and see them FAIL before implementing.**

- [X] T102 [P] [US6] Write `tests/features/foundation/06_audit.feature` with one scenario per acceptance scenario 1–12 of User Story 6 (an entry for every change; a failed change leaves neither change nor entry; filters; an entry outlives its record with the name it had; an altered entry is detected with where; no command edits or removes entries; export; telemetry recorded and viewable; off stops it while the trail continues; nothing is sent; no secrets; who acted)
- [X] T103 [P] [US6] Unit tests for verification in `crates/trcli-application/src/governance/verify.rs` (test module) per `specs/000-foundation/data-model.md` §AuditEntry: a wrong `previous_hash`, a wrong `hash`, a gap in sequences, and a last entry that does not match the head are each reported with the sequence and which check failed (altered, missing, shortened); an intact trail passes
- [X] T104 [P] [US6] Contract test suite in `crates/trcli-application/src/testing/contract_audit.rs` for `AuditQuery` (filter by record, kind, actor, action, date range; newest first; limit) and `TelemetryLog` (written only while enabled; arguments and values never recorded)
- [X] T105 [P] [US6] Unit test in `crates/trcli-application/src/governance/record.rs` (test module): a field marked secret is absent from an audit entry's `changes`, from problem messages, and from the JSON envelope (FR-054)

### Implementation for User Story 6

- [X] T106 [P] [US6] Implement `TelemetryRecord` (`at`, `command` as the command path without arguments, `duration_ms`, `outcome`) in `crates/trcli-domain/src/governance/telemetry.rs`
- [X] T107 [P] [US6] Define the ports `AuditQuery`, `AuditHead`, `TelemetryLog` in `crates/trcli-application/src/ports/audit.rs` and their fakes in `crates/trcli-application/src/testing/audit.rs`
- [X] T108 [US6] Implement the audit head in `crates/trcli-infra-system/src/audit_head.rs`: the file `.trcli/audit.head` holding the sequence and hash of the last entry, rewritten (temporary name, then rename) after every commit
- [X] T109 [US6] Implement `crates/trcli-application/src/governance/{query.rs,verify.rs,export.rs}`: query with filters; verification walking from sequence 1 with progress for a long trail; export of a filtered range as Markdown, JSON, or CSV, itself recorded as an `export` audit entry (FR-046)
- [X] T110 [US6] Add migration `crates/trcli-infra-sqlite/src/migrations/m0003_telemetry.rs` (table `telemetry_record`) and implement `crates/trcli-infra-sqlite/src/stores/{audit_query.rs,telemetry.rs}`; run the contract suites in `crates/trcli-infra-sqlite/tests/contract_audit.rs`
- [X] T111 [US6] Implement the telemetry decorator in `crates/trcli-cli/src/main.rs` around command dispatch (command path, duration, outcome; only while `telemetry.enabled`), and `crates/trcli-application/src/governance/telemetry_summary.rs` to summarise it
- [X] T112 [US6] Implement `audit list|verify|export` and `telemetry show|on|off|status` in `crates/trcli-cli/src/args/audit.rs` and `crates/trcli-cli/src/commands/audit.rs` per `specs/000-foundation/contracts/cli-audit.md`; `audit verify` ends with exit code 6 naming the first problem; there is no `audit add`, `edit`, or `rm`
- [X] T113 [US6] Write `docs/usage/audit.md` (what is recorded, filters, verification and its limit, export, telemetry and how to turn it off) and make `tests/features/foundation/06_audit.feature` pass, including the two tamper scenarios (an altered row; a removed last row) and the power-loss scenario: kill the process during `specimen slow`, then `trcli audit verify` and `trcli workspace check` succeed (SC-008)

**Checkpoint**: The trail is queryable and tamper-evident; telemetry is local and optional.

---

## Phase 9: User Story 7 - Find out how to do anything (Priority: P7)

**Goal**: Every command explains itself with an example; every feature has a guide whose examples are known to be right; mistakes get a suggestion.

**Independent Test**: Ask for help on the tool, on a group of commands, and on one command and confirm each explains usage with an example; mistype a command and confirm a suggestion; run an example from a usage guide and confirm it behaves as written.

### Tests for User Story 7 ⚠️ write first, must fail

> **Write these tests FIRST and see them FAIL before implementing.**

- [X] T114 [P] [US7] Write `tests/features/foundation/07_help.feature` with one scenario per acceptance scenario 1–10 of User Story 7 (top-level help; help on any command with an example; a suggestion for a mistyped command or option; a usage guide exists; a guide's example behaves as written; version; completion; a failure names the next step; first steps in an empty workspace; the same words everywhere)
- [X] T115 [P] [US7] Write `tests/help_examples.rs`: walk the clap command tree and fail if any command lacks an `about`, or lacks an example in its long help; fail if any top-level command group has no `docs/usage/<noun>.md`
- [X] T116 [P] [US7] Write `tests/usage.rs`: run trycmd over every `console` block in `docs/usage/*.md` in a temporary workspace with the test clock

### Implementation for User Story 7

- [X] T117 [US7] Implement `completions <bash|zsh|fish|powershell>` in `crates/trcli-cli/src/args/completions.rs` with clap_complete
- [X] T118 [US7] Implement the first-steps hint in `crates/trcli-cli/src/commands/first_steps.rs`: in a workspace with no records, `trcli` with no arguments adds the first things to do after the short help
- [X] T119 [US7] Add a `next_step` to every foundation problem that has an obvious one (`no_workspace` → `trcli init`; `workspace_needs_upgrade` → `trcli workspace upgrade`; `confirmation_required` → `--yes`; `ambiguous_reference` → the matching handles) in `crates/trcli-application/src/outcome.rs`
- [X] T120 [US7] Review help text, messages, and guides for one vocabulary (workspace, record, short name/handle, tag, note, link, setting, audit trail) and record the glossary at the top of `docs/usage/README.md`
- [X] T121 [US7] Make `tests/features/foundation/07_help.feature`, `tests/help_examples.rs`, and `tests/usage.rs` pass

**Checkpoint**: Help and guides are complete and cannot silently go stale.

---

## Phase 10: User Story 8 - Add a feature without breaking the whole (Priority: P8)

**Goal**: A contributor adds a kind of record or a command by declaring what is particular to it, and the same gates every part passed are applied automatically.

**Independent Test**: Follow the contributor guide to add a trivial new kind of record and confirm it can be listed, tagged, noted, linked, deleted with confirmation, shown in both forms, and appears in the audit trail — without writing code for those; then confirm the gates reject a change lacking tests, documentation, or a usage guide.

### Tests for User Story 8 ⚠️ write first, must fail

> **Write these tests FIRST and see them FAIL before implementing.**

- [X] T122 [P] [US8] Write `tests/features/foundation/08_contributor.feature` with scenarios for acceptance scenarios 1, 2, 8, and 10 of User Story 8, the ones that can be shown through the binary (scenario 3 is automated by `tests/scenario_coverage.rs`, 4 by `tests/invalid_input_gate.rs`, 5 by the documentation lints and `tests/help_examples.rs`, 6 by the CI matrix, 7 by `tests/layering.rs`, and 9 by the upgrade tests): both sample kinds have every shared behaviour; their commands follow the grammar, options, forms, and exit codes; the tool runs with a fixed clock and seeded identifiers; `specimen` refers to another record only by kind and identity
- [X] T123 [P] [US8] Write `tests/layering.rs`: parse the output of `cargo metadata --format-version 1` and fail if `trcli-domain` depends on anything but thiserror, time, uuid, unicode-normalization; if `trcli-application` depends on clap, sea-orm, tokio, or toml; or if the two `trcli-infra-*` crates depend on each other (FR-070)
- [X] T124 [P] [US8] Write `tests/sample_kind_is_external.rs`: fail if `specimen` or `sample-note`/`sample_note` appears in any file under `crates/` outside the `src/sample/` directories and the files guarded by the `sample-kind` feature (proof that FR-068 needs no foundation change)
- [X] T125 [P] [US8] Write `tests/scenario_coverage.rs`: read `specs/000-foundation/spec.md`, count the numbered acceptance scenarios under each user story, and fail if any `@US<n>-<mm>` tag is missing from `tests/features/foundation/`; scenarios automated by a structural test instead of Gherkin are declared in a table at the top of the file, each with the test that covers it (User Story 8, scenario 3)

### Implementation for User Story 8

- [X] T126 [US8] Write `CONTRIBUTING.md`: prerequisites, build, every check and what it protects, the order of work in a phase (failing scenarios → failing unit tests → code → usage guide), the layers and where each kind of code goes, the commenting and naming rules of `specs/000-foundation/research.md` §15, the SOLID rules of `plan.md`, and a release checklist that includes adding the fixture of the outgoing format under `tests/fixtures/formats/`
- [X] T127 [P] [US8] Write `docs/contributing/adding-a-command.md`: a worked example adding one command end to end — scenario, validated command, handler, view model, clap definition, help example, usage guide
- [X] T128 [P] [US8] Write `docs/contributing/adding-a-record-kind.md`: a worked example following `specs/000-foundation/contracts/feature-contract.md` — descriptor, migration, store with its contract tests, use cases, commands — using `specimen` as the reference
- [X] T129 [US8] Add the acceptance gates to `.github/workflows/ci.yml` as required jobs on all three systems: `tests/layering.rs`, `tests/help_examples.rs`, `tests/invalid_input_gate.rs`, `tests/usage.rs`, `tests/sample_kind_is_external.rs`, `tests/scenario_coverage.rs`, the contract suites, and the upgrade test
- [ ] T130 [US8] Make `tests/features/foundation/08_contributor.feature`, `tests/layering.rs`, `tests/sample_kind_is_external.rs`, and `tests/scenario_coverage.rs` pass; then time a newcomer (or a fresh clone in CI) through `CONTRIBUTING.md` and `adding-a-command.md` and record the result in `specs/000-foundation/quickstart.md` (SC-015: under 1 hour)

**Checkpoint**: The consistent way is the easy way, and the build refuses the inconsistent one.

---

## Phase 11: Polish & Cross-Cutting Concerns

**Purpose**: The qualities every part must have (FR-062 to FR-067), measured; and the housekeeping this plan left for the end.

- [X] T131 [P] Add performance checks in `tests/performance.rs` (ignored by default, run in CI on Linux): `trcli --version` and `trcli workspace show` each under 100 ms; listing and searching 10,000 `specimen` records under 2 s (SC-012)
- [X] T132 [P] Add `tests/any_script.rs`: specimens titled "Ação", "acao", "東京", and a right-to-left title are stored and shown as written; `--search acao` finds both Portuguese spellings; `--sort title` orders without error (FR-067)
- [X] T133 [P] Add `tests/offline.rs`: with network access denied to the process, every foundation command works (SC-010, SC-011); assert no foundation crate depends on a network library by extending `tests/layering.rs`
- [X] T134 [P] Add `tests/no_half_written_files.rs`: interrupt each file-writing path (settings file, audit head, backup copy) at the rename step and assert the original is intact (FR-066)
- [X] T135 [P] Add `tests/read_only_workspace.rs`: with the workspace directory made read-only, reading commands (`workspace show`, `audit list`) succeed and changing commands fail with an explanation and no partial write (spec edge case)
- [ ] T136 Run `specs/000-foundation/quickstart.md` end to end on Linux, macOS, and Windows; fix what differs; note any Windows path edge case and, if `paths.rs` handles it badly, add the `directories` crate with the reason recorded in `specs/000-foundation/research.md` §3
- [X] T137 Create `.github/workflows/release.yml` building one self-contained `trcli` binary per system and architecture (Linux, macOS, Windows; x86_64 and aarch64) from a default build (no `sample-kind`, no `test-clock`), and add an "Install" section to `README.md` (FR-062, SC-001)
- [X] T138 Re-check the requirement numbers cited inside `specs/000-foundation/contracts/{cli-conventions.md,output-and-exit-codes.md,configuration.md,cli-workspace.md,cli-link.md,cli-audit.md}` against `specs/000-foundation/spec.md` and correct those still using the numbering of the first specification
- [ ] T139 Delete the superseded plan artefacts `specs/001-research-workspace/{plan.md,research.md,data-model.md,quickstart.md}` once this plan is accepted, or reduce them to what remains specified there (datasets, environment and reproducibility)
- [ ] T140 Propose the constitution amendment that replaces `TODO(TECH_STACK)` in `.specify/memory/constitution.md` with the stack settled by this plan (run `/speckit-constitution`; version 1.1.0)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: no dependencies. The two spikes at its end decide exact feature flags and whether the BDD runner needs sharding; nothing else should be built before they pass.
- **Phase 2 (Foundational)**: depends on Phase 1. **Blocks every user story.**
- **Phases 3–10 (User Stories)**: each depends on Phase 2. See the story dependencies below.
- **Phase 11 (Polish)**: depends on the stories it measures; its test tasks need US2 (the sample kinds).

### User Story Dependencies

```text
 Phase 2 ──▶ US1 workspace ──▶ US2 records ──┬──▶ US3 validation (uses the sample commands)
                  │                          ├──▶ US4 output and interaction (uses specimen commands)
                  │                          └──▶ US8 contributor (the sample kind is its proof)
                  ├──▶ US5 settings
                  └──▶ US6 audit queries (two of its scenarios use the sample commands of US2 and US4)
 US7 help ── last of the researcher-facing stories: its gates walk every command that exists
```

- **US1** needs only Phase 2. It is the MVP.
- **US2** needs US1 (records live in a workspace).
- **US3** needs US2: its scenarios exercise validation through `link add` and the sample commands.
- **US4** needs US2 for the sample commands; everything else in it needs only Phase 2.
- **US5** needs US1. Independent of US2, US3, US4.
- **US6** needs US1. Its "entry outlives its record" scenario uses a specimen (US2), and its power-loss scenario uses `specimen slow` (US4).
- **US7** needs the commands of US1, US2, US5, US6 to exist, since its gates check all of them.
- **US8** needs US2 (the sample kind) and US7 (the gates it adds to CI).

### Within Each Phase

- Test tasks first; run them and see them fail.
- Then inside-out: domain, application (ports, fakes, use cases), adapters, CLI.
- A contract suite is written in the application crate, passed by the fake, then by the SQLite adapter.
- The phase ends with its usage guide and its feature file green.

### Parallel Opportunities

- **Phase 1**: T003–T006 touch different files.
- **Phase 2**: all "tests first" tasks are in different files; the five domain-kernel tasks are in different files; the three system-adapter tasks likewise.
- **After US1**: US5 and US6 can proceed in parallel with US2, by different people; only the last task of US6 (its feature file, which uses the sample commands) waits for US2 and US4.
- **After US2**: US3, US4, and US8's guides can proceed in parallel.
- Within every story, the tasks marked [P] under "Tests" can be written together.

---

## Parallel Example: Phase 2 tests

```text
# Different files, no dependencies between them — write together, see them fail together:
Task: "Unit tests for the shared text value objects in crates/trcli-domain/src/shared/text.rs"
Task: "Unit tests for ValidationReport and FieldProblem in crates/trcli-domain/src/shared/problem.rs"
Task: "Unit tests for Outcome in crates/trcli-application/src/outcome.rs"
Task: "Unit tests for the JSON envelope in crates/trcli-cli/src/render/json.rs"
Task: "Unit tests for settings layering in crates/trcli-application/src/settings/layers.rs"
```

## Parallel Example: after User Story 1

```text
# Three people, three stories, no shared files:
Developer A: Phase 4 — User Story 2 (records, the sample kind)
Developer B: Phase 7 — User Story 5 (settings commands)
Developer C: Phase 8 — User Story 6 (audit queries, verification, telemetry; its final task waits for A)
```

---

## Implementation Strategy

### MVP First (User Story 1 only)

1. Phase 1: Setup — stop after the two spikes and record what they found.
2. Phase 2: Foundational — the checkpoint is a binary that answers in both forms with the right exit codes.
3. Phase 3: User Story 1.
4. **Stop and validate**: run the first three parts of `quickstart.md` (skeleton, validation, workspace). A workspace can be created, found, shown, edited, upgraded, and checked on three systems.

### Incremental Delivery

Each phase is one stacked pull request and leaves the tool working:

1. Setup + Foundational → skeleton
2. US1 → workspaces (**MVP**)
3. US2 → records with shared behaviour; from here other specifications can start building on the foundation
4. US5, US6 → settings; audit queries
5. US3, US4 → validation everywhere; interaction
6. US7, US8 → help and guides; contributor gates
7. Polish → measured qualities

Other specifications need Phases 1–4 at minimum (workspace, records, audit recording). `specs/003-research-projects` additionally relies on settings (US5) and prompts (US4).

### Notes

- [P] tasks touch different files and have no unfinished dependencies.
- Every task that writes code also writes its documentation; there is no separate "document it" task.
- Commit after each task or small group; open the phase's pull request when its feature file is green.
- If a task cannot be done without adding a library, stop and record the reason in `research.md` §3 first.

---

## Implementation status (2026-10-08)

135 of 140 tasks are done and marked. The five left open, and why:

| Task | Status |
|------|--------|
| T008 | The cucumber harness runs harness-less on Linux (110 scenarios in about 33 s, recorded in `research.md` §3 and §14). **Open**: confirmation on macOS and Windows, which the first CI run gives. |
| T130 | The four test files pass. **Open**: timing a newcomer through `CONTRIBUTING.md` and `adding-a-command.md` (SC-015); it needs a person who has not seen the code. |
| T136 | The quickstart was walked through by hand on Linux, including the prompt, colours, and progress at a real terminal. **Open**: macOS and Windows by hand; CI runs every automated suite there. |
| T139 | **Open** on purpose: the superseded plan artefacts of `specs/001-research-workspace` are removed once this plan is accepted (merged), in that specification's own pull request. |
| T140 | **Open** on purpose: amending the constitution is done with `/speckit-constitution`, not by this implementation. |

### Where the implementation differs from the tasks as written

Each is recorded with its reason in `research.md` §3 or beside the code it concerns.

| Task | As written | As built | Why |
|------|-----------|----------|-----|
| T001 | Doubles are hand-written (no crate was planned for them) | Unchanged, after considering `mockall` | Generated mocks are programmed call by call; these tests need storage that commits and rolls back, and assert state, not calls. See `research.md` §3 |
| T001, T116 | `trycmd` runs the examples of the usage guides | `tests/usage.rs` runs them with ~120 lines of our own | One guide is one continuous session with a fixed clock and seeded identifiers; one dependency fewer |
| T001, T040 | Colour through `anstream`/`anstyle` | `anstyle` for styles; whether to colour is our own rule in `render/theme.rs` | The contract's precedence (`--color always` over `NO_COLOR`) is ours and is unit-tested |
| T007 | The spike is kept as `tests/spike_seaorm.rs` | What it established is kept as `crates/trcli-infra-sqlite/tests/connection.rs` | One file says how the database is opened and proves it |
| T034, T070, T110 | Instants and identifiers as SeaORM `time`/`uuid` columns | Instants as whole milliseconds, identifiers as hyphenated text | They sort correctly as stored and come back exactly as written, which the audit hashes depend on |
| T030 | `Storage` has `begin` | It also has `read` and `close` | A reading command takes no write lock; closing leaves the workspace as one file (found by the "damaged workspace" scenario) |
| T051, T054 | `WorkspaceStore` creates the `.trcli/` directory | `StorageOpener` (SQLite) creates and opens the database; `WorkspaceFiles` (system) creates the directory, the settings file, and `backups/`; `WorkspaceStore` reads and saves the workspace row | The two adapters may not depend on each other |
| T068 | `RecordKindDescriptor` holds the display name, search key source, deletion policy, and removal | The descriptor is data (name, prefix, summary, name of the main text); `KindBehaviour<U>` is the trait for what needs storage (deletion block, dependents, removal, fields) | Ports use `async fn` and are not object-safe; a registry of data plus a statically dispatched behaviour needs no boxing |
| T073 | `sample-note` has one field, `body` | It also has `locked` | To show a deletion a feature forbids (User Story 2, scenario 11) |
| T002, T073, T075, T093, T124 | The sample kinds are modules of the three crates behind a `sample-kind` build feature | They are a Cargo example, `crates/trcli-cli/examples/sample_kinds/`: the tool with the two kinds added from outside through the `Extension` trait (`crates/trcli-cli/src/extension.rs`). The feature is gone from all three crates | `examples/` is where Rust keeps code that shows how a library is used; the binary cannot contain the samples by construction; the proof of FR-068 is stronger, since no source file of any crate names them |
| T002, T031 and every "fakes in `src/testing/`" task | Fakes and contract suites are a module of the application crate behind a `test-support` feature | They are a crate of their own, `crates/trcli-testing`, a development dependency only (six crates in the workspace, five in the tool) | Test support does not belong in the sources of a crate that is compiled into the tool |
| Every "(test module)" task of a use case | Unit tests inline in the use case's file | In `crates/trcli-application/tests/use_cases/<module>.rs`; tests of pure logic that needs no double stay inline | A crate's inline tests cannot use doubles that live in another crate, and a use case is rightly tested through what it makes public |
| T077 | Validation scenarios are driven through `init`, `workspace edit`, `link add`, and `specimen` | Also through `audit list` and `config set` | They are where a date, a rule between two values, a value naming a record, and file input can be shown |
| T112 | `audit export [filters] --to <file>` | The last day of an export is `--until`; `audit list` accepts both `--to` and `--until` | In `audit export` the contract gave `--to` two meanings; `contracts/cli-audit.md` is corrected |
| T112 | `telemetry show [--experiment <ref>]` | `telemetry show` | Experiments do not exist yet; the option arrives with `specs/004-experiments` |
| T085 | The progress scenario runs at a terminal | The Gherkin scenario covers "nothing is drawn when nobody is watching"; the terminal half is covered by the unit tests of `progress.rs` and was checked by hand with a pseudo-terminal | A test process has no terminal |
| T028 | Value kinds include lists of text and spans | Text, whole number, yes/no, one of a list, path, style | No foundation setting takes a list or a span; they arrive with the first feature that registers one |
