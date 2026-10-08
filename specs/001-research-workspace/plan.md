# Implementation Plan: TRCLI Research Workspace

**Branch**: `001-research-workspace` | **Date**: 2026-10-08 | **Spec**: [spec.md](./spec.md)

**Input**: Feature specification from `/specs/001-research-workspace/spec.md`

**Note**: This template is filled in by the `/speckit-plan` command; its definition describes the execution workflow.

> **Superseded as the foundation plan (2026-10-08)**: the foundation and architecture of
> TRCLI are now specified in `specs/000-foundation`. The architectural decisions below —
> layering, storage, identifiers, validation, testing — are the starting point for that
> specification's plan and should be consolidated there, trimmed by the later rule to keep
> libraries to a minimum (see `specs/003-research-projects/research.md` §3). This document
> is kept for reference until that plan exists.

## Summary

Build `trcli`, a command-line research workspace covering the eleven user stories of the
spec: papers and citations, research questions and hypotheses, bibliographic research,
drafts, experiments with pipelines and manual steps, results and figures, methodologies and
datasets, environment and reproducibility, staff, audit and telemetry, and courses.

Technical approach: a Rust Cargo workspace with one crate per clean-architecture layer
(domain → application → adapters → CLI), so the compiler enforces dependency direction. The
domain is organized in twelve bounded contexts. Each workspace keeps a SQLite database in a
`.trcli/` directory, accessed through SeaORM behind repository ports. Everything is async on
tokio; long operations show progress, can be cancelled, and a run that reaches a manual step
saves its state and returns the terminal. Storage, files, processes, paths, and catalogues
are all behind ports, so the tool is local today and a remote backend (Google Drive or
other) can be added later as another adapter. Behavior is specified in Gherkin and tested
against the compiled binary; usage guides are executed as tests.

This plan sets the architecture and the full data model for the whole spec. Delivery is in
slices, one user story at a time, each as its own stacked pull request (see
[Delivery Slices](#delivery-slices)).

## Technical Context

**Language/Version**: Rust 1.96, edition 2024

**Primary Dependencies**: clap 4.6 (CLI), SeaORM 2.0 + sea-orm-migration (persistence), tokio 1.53 (async), anstream/anstyle + comfy-table + indicatif (colored output, tables, progress), figment + directories (configuration and platform paths), biblatex + hayagriva (bibliography), reqwest 0.13 with rustls (online lookup), blake3 + sha2 (fingerprints, audit chain), thiserror, serde, uuid v7, time

**Storage**: SQLite, one database per workspace at `<workspace>/.trcli/trcli.db` (path configurable via `storage.path`); step logs under `.trcli/runs/`; settings in TOML

**Testing**: `cargo test` with rstest and proptest (unit); temporary SQLite databases (adapter integration); cucumber 0.23 + assert_cmd running Gherkin scenarios against the built `trcli` binary (BDD); trycmd executing the examples in `docs/usage/*.md`

**Target Platform**: Linux, macOS, and Windows (x86_64 and aarch64), as a single self-contained binary

**Project Type**: CLI

**Performance Goals**: list/filter/search over 10,000 papers in under 2 s (SC-003); import of 1,000 bibliography entries in under 1 minute (SC-002); start-up of a simple command under 100 ms; failed online lookup reported in under 10 s (SC-015)

**Constraints**: every capability except online lookup works offline; no data leaves the machine except the identifier of an explicit lookup; no command may wait indefinitely on a prompt when not attached to a terminal; no secret values stored in environment snapshots; all input validated before any write

**Scale/Scope**: single researcher; up to ~10,000 papers, ~1,000 runs, ~100,000 audit entries per workspace; 12 bounded contexts, ~40 tables, ~25 CLI nouns; 11 user stories, 76 functional requirements

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Gate | How this plan meets it | Status |
|-----------|------|------------------------|--------|
| I. Clean Architecture | Layers; dependencies inward only; external concerns behind ports | One crate per layer; `trcli-domain` has no dependency on clap, SeaORM, tokio, or reqwest, so a violation does not compile. Storage, files, processes, paths, clock, ids, catalogues are ports with adapters. | PASS |
| II. Domain-Driven Design | Ubiquitous language; entities, value objects, aggregates, repositories; explicit contexts; invariants in the domain | Twelve bounded contexts named in the spec's language; aggregates and value objects listed in `data-model.md`; contexts reference each other only by `RecordRef`. | PASS |
| III. Test-First (non-negotiable) | Tests written and failing before code; unit + integration | Slice workflow: failing Gherkin scenarios → failing unit tests → code → adapter tests. In-memory fakes of all ports make unit tests fast. | PASS |
| IV. BDD | Given/When/Then for every user-facing feature, automated; primary, error, and invalid-input flows | Every acceptance scenario of the spec becomes a Gherkin scenario in `tests/features/`, run against the compiled binary. | PASS |
| V. Input Validation (non-negotiable) | All input validated at the boundary; clear message on stderr; non-zero exit; invalid-input tests for every input | Three layers: clap parsers, validating command constructors that report all errors together, value objects that cannot hold invalid values. Imports and lookups go through the same path. Exit code 2 for invalid input. | PASS |
| VI. Full Code Documentation | Doc comment on every module and public item | `#![deny(missing_docs)]` in every crate; `cargo doc` with warnings as errors in CI. | PASS |
| VII. Feature Docs and Usage Guides | Usage Markdown per feature, shipped with it | `docs/usage/<noun>.md` per CLI noun, created in the same slice; its examples are executed by trycmd, so a stale guide fails the build. | PASS |

**Quality gates from the constitution**: full test suite (unit, integration, BDD) on three
platforms; `cargo fmt --check`; `cargo clippy -- -D warnings`; documentation coverage;
dependency direction (enforced by crate boundaries); boundary-validation tests per command;
usage file per feature. All are CI jobs defined in slice 0.

**Constitution follow-up**: the constitution carries `TODO(TECH_STACK)`. This plan settles
it (Rust, clap, SeaORM, SQLite, tokio, cucumber). The constitution should be amended
(MINOR, to 1.1.0) with `/speckit-constitution`; that amendment is outside this command.

**Post-design re-check (after Phase 1)**: PASS. The data model keeps every invariant in the
domain; the CLI contract gives every command a validation and error path; no design choice
required an outward dependency from the domain or application crates.

## Project Structure

### Documentation (this feature)

```text
specs/001-research-workspace/
├── plan.md              # This file (/speckit-plan command output)
├── research.md          # Phase 0 output (/speckit-plan command)
├── data-model.md        # Phase 1 output (/speckit-plan command)
├── quickstart.md        # Phase 1 output (/speckit-plan command)
├── contracts/           # Phase 1 output (/speckit-plan command)
│   ├── cli-conventions.md
│   ├── output-and-exit-codes.md
│   └── configuration.md
└── tasks.md             # Phase 2 output (/speckit-tasks command - NOT created by /speckit-plan)
```

### Source Code (repository root)

```text
Cargo.toml                       # workspace manifest, shared dependency versions and lints
rust-toolchain.toml
.github/workflows/ci.yml         # fmt, clippy, doc, test on linux/macos/windows

crates/
├── trcli-domain/                # pure domain: no I/O, no async runtime, no framework
│   └── src/
│       ├── shared/              # RecordRef, Handle, Tag, Note, Link, value objects, errors
│       ├── workspace/
│       ├── library/             # Paper, Citation
│       ├── inquiry/             # ResearchQuestion, Hypothesis
│       ├── review/              # BibliographicResearch, Search, Candidate
│       ├── writing/             # Draft, DraftVersion
│       ├── experimentation/     # Experiment, Pipeline, Step, Run, StepResult
│       ├── findings/            # Result, Exhibit
│       ├── assets/              # Methodology, Dataset, DatasetVersion
│       ├── reproducibility/     # EnvironmentSnapshot, ReproCheck, ReproPackage
│       ├── people/              # StaffMember, Assignment
│       ├── governance/          # AuditEntry, TelemetryRecord
│       └── learning/            # Course
│
├── trcli-application/           # use cases and ports
│   └── src/
│       ├── ports/               # repositories, UnitOfWork, ArtifactStore, ProcessRunner,
│       │                        # EnvironmentProbe, MetadataCatalog, BibliographyCodec,
│       │                        # CitationFormatter, Clock, IdGenerator, ActorProvider
│       ├── validation/          # ValidationReport, command constructors
│       ├── <context>/           # one module per context: commands, queries, handlers
│       ├── run_engine/          # executes pipelines, pauses, resumes, cancels
│       ├── reference_guard.rs   # finds dependents before a deletion (FR-008)
│       ├── auditing.rs          # records AuditEntry inside the unit of work
│       └── testing/             # in-memory fakes of every port (feature "test-support")
│
├── trcli-infra-sqlite/          # SeaORM entities, migrations, repository implementations
│   ├── src/{entities,migrations,repositories,unit_of_work.rs,connection.rs}
│   └── tests/                   # repositories against a temporary database
│
├── trcli-infra-system/          # everything platform-specific
│   ├── src/{paths.rs,config.rs,process.rs,environment.rs,artifact_store.rs,
│   │        fingerprint.rs,clock.rs,ids.rs,actor.rs}
│   └── tests/
│
├── trcli-infra-biblio/          # BibTeX, RIS, CSL-JSON codecs; citation formatting
│   ├── src/
│   └── tests/
│
├── trcli-infra-catalog/         # online lookup: Crossref, arXiv, Open Library
│   ├── src/
│   └── tests/                   # against a local stub server
│
└── trcli-cli/                   # the `trcli` binary: composition root and presentation
    └── src/
        ├── main.rs              # runtime, signal handling, exit codes
        ├── compose.rs           # wires adapters into use cases
        ├── args/                # clap definitions, one module per noun
        ├── commands/            # maps parsed args → use case → view
        ├── render/              # human (color, tables) and JSON renderers, Theme
        ├── prompt.rs            # confirmations, respecting --yes / --no-input
        └── progress.rs

tests/                           # workspace-level black-box tests of the binary
├── features/                    # Gherkin, one file per user story
│   ├── 01_papers_citations.feature
│   ├── 02_questions_hypotheses.feature
│   └── ...
├── bdd/                         # cucumber world and generic step definitions
└── usage.rs                     # trycmd over docs/usage/*.md

docs/
└── usage/                       # one guide per CLI noun: paper.md, cite.md, run.md, ...
```

**Structure Decision**: A Cargo workspace with seven crates, one per architectural layer
or external concern. Dependency direction is `trcli-cli` → adapters → `trcli-application`
→ `trcli-domain`; adapters never depend on each other. Bounded contexts are modules with
the same names in the domain and application crates. Black-box tests live at the workspace
root because they exercise the binary, not a crate.

## Architecture Notes

### Flow of one command

```text
argv ─▶ clap (syntax) ─▶ command constructor (validates all fields, ValidationReport)
     ─▶ use-case handler ─▶ aggregate (invariants) ─▶ repository port ┐
                         └▶ audit entry ────────────────────────────── ├─▶ UnitOfWork.commit
     ◀─ view model ◀──────────────────────────────────────────────────┘
     ─▶ renderer (human+color | json) ─▶ stdout      errors ─▶ stderr + exit code
```

### Ports that keep the tool local today and extensible later

| Port | Today | Later |
|------|-------|-------|
| `*Repository`, `UnitOfWork` | SQLite via SeaORM | another database or a synced store |
| `ArtifactStore` | local file system | Google Drive, object storage |
| `MetadataCatalog` | Crossref, arXiv, Open Library | more catalogues |
| `ProcessRunner` | local child process | remote execution |
| `SyncProvider` *(not defined yet)* | — | replays the audit journal between copies |

### Workspace location

1. `--workspace <dir>` or `TRCLI_WORKSPACE`, if given;
2. otherwise the nearest `.trcli/` found walking up from the current directory;
3. otherwise `default_workspace` from the user's settings, if set;
4. otherwise an error explaining how to run `trcli init`.

## Delivery Slices

Each slice is one stacked pull request, opens with failing Gherkin scenarios, and ends with
its usage guides. Slices follow the spec's priorities.

> **Note (2026-10-08)**: after this plan was written, experiments, pipelines, runs, results,
> figures, and tables moved to `specs/004-experiments`. Slices 5 and 6 below are therefore
> specified there, and that spec adds experiment design, parameters, metrics, sweeps,
> software versions, conclusions, and replication. The architecture in this plan and the
> `experimentation` and `findings` sections of `data-model.md` remain the technical basis;
> they are to be extended when `specs/004-experiments` is planned.
>
> Likewise, papers, citations, online lookup, and bibliographic research moved to
> `specs/005-literature`. Slices 1 and 3 below are specified there, and that spec adds other
> kinds of reference, bibliographies, a reading queue, annotations, relations between
> references, two-stage screening, and synthesis. The `library` and `review` sections of
> `data-model.md` are to be extended when `specs/005-literature` is planned. Slice 0 (the
> workspace itself) and slices 2, 4, and 7 to 11 remain specified by this spec.
>
> Draft papers then moved to `specs/008-manuscripts` as well, so slice 4 is specified
> there, with kinds of manuscript, authorship and contributions, parts, readiness, and
> publication added. Courses moved to `specs/011-courses-roadmaps`, so slice 11 is specified
> there. Staff moved to `specs/012-people`, so slice 9 is specified there. Slice 0 and
> slices 7, 8, and 10 remain specified by this spec; slice 2 (research questions and
> hypotheses) moved to `specs/013-ideas-questions`, and the methodology half of slice 7 to
> `specs/014-conventions-methods`.

| Slice | Content | Spec |
|-------|---------|------|
| 0 | Walking skeleton: Cargo workspace, CI on three platforms, configuration, `.trcli` discovery, `trcli init`, connection and migrations, unit of work, audit recording, handles, renderers and theme, exit codes, BDD harness, `trcli config` | FR-001–015, FR-051 |
| 1 | Papers and citations; import/export; duplicates; online lookup; tags, notes, links; guarded delete | Story 1 |
| 2 | Research questions and hypotheses | Story 2 |
| 3 | Bibliographic research | Story 3 |
| 4 | Drafts, versions, per-draft bibliography | Story 4 |
| 5 | Experiments, pipelines, run engine, manual steps, interrupt and resume | Story 5 |
| 6 | Results, figures, tables; provenance; stale-result flag | Story 6 |
| 7 | Methodologies and datasets; fingerprints and versions | Story 7 |
| 8 | Environment snapshots; reproducibility check and package | Story 8 |
| 9 | Staff and assignments | Story 9 |
| 10 | Audit queries, verification, export; telemetry views and switch | Story 10 |
| 11 | Courses | Story 11 |

Slices 5–8 are the technically riskiest (process execution on three platforms, state
recovery, fingerprinting large data). Slice 0 carries the three spikes named in
`research.md`.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

There are no constitution violations. Two choices add structure beyond the minimum and are
recorded here for reviewers:

| Choice | Why Needed | Simpler Alternative Rejected Because |
|--------|------------|-------------------------------------|
| Seven crates instead of one | The constitution requires dependency direction to be verified; crate boundaries make the compiler verify it, and keep SeaORM out of domain test builds | One crate with modules relies on review or an extra lint tool to catch a forbidden import |
| Two identifiers per record (UUID + short handle) | FR-004 needs short typeable ids; future remote/shared storage needs ids that never collide across copies | A single counter is short but collides on merge; a single UUID is unique but not typeable |
