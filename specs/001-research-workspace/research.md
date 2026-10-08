# Phase 0 Research: TRCLI Research Workspace

**Date**: 2026-10-08 | **Plan**: [plan.md](./plan.md) | **Spec**: [spec.md](./spec.md)

The user fixed the main stack (Rust, clap, SeaORM, SQLite, a `.trcli` directory, async,
colored output, DDD/TDD/BDD, local now and remote later). This document records the
decisions that follow from it. Crate versions are the latest published on crates.io on
2026-10-08 (checked with `cargo search`); the installed toolchain is Rust 1.96.0.

There are no open NEEDS CLARIFICATION items. Points marked **Spike** are small things to
confirm in code at the start of implementation because they rest on library behavior that
was not exercised during planning.

---

## 1. Language, edition, and toolchain

- **Decision**: Rust, edition 2024, minimum supported version pinned to 1.96 in
  `rust-toolchain.toml`.
- **Rationale**: Chosen by the user. One static binary per platform, no runtime to install,
  and async traits in the language (no boxing macro needed for ports that are not used as
  trait objects).
- **Alternatives considered**: None; fixed by the user.

## 2. Code organization: a Cargo workspace with one crate per architectural layer

- **Decision**: Seven crates under `crates/`:
  `trcli-domain` → `trcli-application` → adapters (`trcli-infra-sqlite`,
  `trcli-infra-system`, `trcli-infra-biblio`, `trcli-infra-catalog`) → `trcli-cli`
  (composition root and the `trcli` binary).
- **Rationale**: The constitution requires that dependencies point inward and that this be
  *verified*. Separate crates make the compiler do the verifying: `trcli-domain` simply
  does not list SeaORM, clap, tokio, or reqwest as dependencies, so a forbidden import
  cannot compile. Adapters are split by the external thing they wrap, so each can be
  replaced (a second storage backend, another catalogue) without touching the others.
- **Alternatives considered**:
  - *Single crate with modules* — simplest, but layer rules would rest on review or an extra
    lint tool, and every test build compiles SeaORM.
  - *One crate per bounded context* — twelve contexts times three layers is far too many
    crates for one team; contexts are modules inside each layer crate instead.

## 3. Bounded contexts (DDD)

- **Decision**: Twelve contexts, each a module in `trcli-domain` and in `trcli-application`,
  plus a shared kernel:

  | Context | Aggregates | Spec story |
  |---------|------------|-----------|
  | `workspace` | Workspace | 1 |
  | `library` | Paper, Citation | 1 |
  | `inquiry` | ResearchQuestion (with Hypotheses) | 2 |
  | `review` | BibliographicResearch (with Searches, Candidates) | 3 |
  | `writing` | Draft (with Versions, Authors) | 4 |
  | `experimentation` | Experiment (with Pipeline), Run (with StepResults) | 5 |
  | `findings` | Result, Exhibit (figure or table) | 6 |
  | `assets` | Methodology, Dataset (with Versions) | 7 |
  | `reproducibility` | EnvironmentSnapshot, ReproPackage | 8 |
  | `people` | StaffMember | 9 |
  | `governance` | AuditEntry, TelemetryRecord | 10 |
  | `learning` | Course | 11 |
  | `shared` (kernel) | RecordRef, Handle, Tag, Note, Link, value objects | all |

- **Rationale**: Each context has its own language in the spec (a "candidate" exists only in
  a review; a "stage" only for a draft). Aggregates are sized so one command changes one
  aggregate in one transaction. Contexts refer to each other only by `RecordRef`
  (kind + id), never by holding another context's entity.
- **Alternatives considered**: One flat model — rejected, the spec already has ~25 entities
  and specs 002 and 003 add more.

## 4. Storage: SQLite through SeaORM, inside a `.trcli` directory

- **Decision**: Each workspace has a `.trcli/` directory at its root (found the way `.git`
  is: walk up from the current directory). It contains `trcli.db` (SQLite), `config.toml`,
  `runs/` (step logs), and `backups/`. SeaORM 2.0 with its SQLite driver on tokio;
  migrations with `sea-orm-migration`, embedded in the binary. The database path is a
  setting (`storage.path`), so the repository can live elsewhere.
- **Connection settings**: `journal_mode=WAL`, `foreign_keys=ON`, `busy_timeout=5000`,
  `synchronous=NORMAL`.
- **Rationale**: Chosen by the user. A single file makes a workspace easy to copy and back
  up; WAL lets a second `trcli` process read while one writes (for example `trcli run show`
  while a run is in progress).
- **Search (SC-003, 10,000 papers under 2 s)**: a normalized lowercase `search_text` column
  per searchable record, queried with `LIKE`, plus ordinary indexes on filter columns. At
  this size a scan takes milliseconds. Full-text indexing is deferred until measurements
  ask for it.
- **Alternatives considered**: sqlx directly (more control, far more hand-written SQL for
  ~40 tables); plain files (JSON/TOML per record — friendly to version control, but
  cross-record queries, integrity, and transactions would be rebuilt by hand).
- **Spike**: confirm the exact SeaORM 2.0 feature flags for SQLite + tokio, and that SQLite
  is compiled in (bundled) so Windows needs no system library.

## 5. Record identity

- **Decision**: Every record has two identifiers.
  - `id`: UUID version 7, the primary key, never shown by default.
  - `handle`: what the user types — a type prefix and a short code, for example `pap-7k3f`.
    The code is 4 characters of Crockford base32 drawn from the random part of the UUID,
    lengthened on collision, unique per workspace and record type. Any unique prefix of a
    handle is accepted; an ambiguous prefix lists the candidates.
- **Rationale**: FR-004 asks for identifiers that are unique, stable, and short. Counters
  (`paper 12`) are shorter but collide as soon as two copies of a workspace are ever merged,
  which the future remote/shared storage requires. A UUID underneath keeps that door open
  at no cost today.
- **Alternatives considered**: Auto-increment integers (collide on merge); showing full
  UUIDs or ULIDs (not typeable).

## 6. Local now, remote later

- **Decision**: Build nothing remote now. Keep four seams so that Google Drive or any other
  backend can be added as a new adapter crate without changing domain or application code:
  1. **Repositories are ports.** Application code sees traits such as `PaperRepository`;
     SQLite is one implementation.
  2. **`UnitOfWork` port.** Transactions are requested by the application, not opened by
     adapters ad hoc, so a different store can supply its own.
  3. **`ArtifactStore` port.** Everything the tool reads by location (dataset files,
     figures, manuscripts, local paper copies) goes through one port that resolves a
     `Location` (today: a local path or a URL that is recorded but not read). A Drive or
     object-storage adapter implements the same port later.
  4. **The audit trail is an ordered, hash-chained change journal.** A future
     `SyncProvider` can replay it between copies; globally unique ids (section 5) make the
     replay safe.
- **Rationale**: The user asked to architect for the future while staying local. These four
  seams are needed anyway for clean architecture and testing (in-memory fakes implement the
  same ports), so they cost nothing extra. A sync engine now would be speculation.
- **Alternatives considered**: Adopting a storage-abstraction library (OpenDAL or
  `object_store`) now — deferred; they are candidates for the future adapter, not something
  the core should depend on. Putting the SQLite file itself in a synced folder is known to
  corrupt databases and will be documented as unsupported.

## 7. Cross-platform behavior (Linux, macOS, Windows)

- **Decision**: All platform differences live in `trcli-infra-system`, behind ports:
  - `PlatformPaths` — user config, data, and cache directories via the `directories` crate
    (XDG on Linux, `~/Library/Application Support` on macOS, `%APPDATA%` on Windows).
  - `ProcessRunner` — runs automated steps with `tokio::process`. A step is a program plus
    arguments, run **without a shell** by default so the same definition works everywhere;
    `shell = true` opts into `sh -c` or `cmd /C`.
  - `EnvironmentProbe` — operating system, architecture, hardware summary, tool versions.
  - `Clock`, `IdGenerator`, `ActorProvider` (OS user name plus configured researcher name).
  - Paths are stored as given and also normalized to forward slashes for comparison; no
    code builds paths by string concatenation.
- **CI**: GitHub Actions matrix on `ubuntu-latest`, `macos-latest`, `windows-latest`; every
  test suite, including the BDD suite, runs on all three.
- **Rationale**: "Modular to work on macOS/Linux/Windows" — one crate owns the differences,
  and the application is tested with fakes of those ports.

## 8. Configuration

- **Decision**: Layered settings, later layers override earlier ones:
  built-in defaults → user file (`<config dir>/trcli/config.toml`) → workspace file
  (`.trcli/config.toml`) → environment (`TRCLI_*`) → command-line flags. Loaded with
  `figment`; exposed through `trcli config get|set|list|path`. Every setting is validated
  on load and on `set`; unknown keys are errors, not silently ignored.
- **Rationale**: "Configurable." The layering is the conventional one for CLIs and gives a
  place for both personal preferences (color, citation style) and workspace facts (storage
  path, telemetry on/off, lookup on/off).
- **Alternatives considered**: the `config` crate (equivalent; figment's provider model is
  simpler to test); settings stored in the database (cannot be read before the database is
  located).

## 9. Async and a terminal that is never blocked

- **Decision**: tokio multi-thread runtime; every port is async.
  - Database and network calls are awaited; file hashing and other CPU-bound work runs on
    the blocking pool.
  - Any operation that may exceed ~200 ms shows a spinner or progress bar (`indicatif`) on
    stderr, only when stderr is a terminal.
  - Ctrl-C is handled with `tokio::signal`: the current operation is cancelled, a running
    step's process is terminated, the run is recorded as `interrupted` (FR-036), and the
    process exits with a distinct code.
  - **A run never holds the terminal waiting for a person.** On reaching a manual step the
    run is saved as `paused` and the command returns immediately, printing the instructions
    and the command to continue. The researcher confirms later with `trcli run confirm`,
    from any terminal.
  - Step output is streamed line by line to the terminal and to a log file concurrently.
  - Online lookups have an 8-second total timeout (SC-015 requires failure under 10 s).
- **Rationale**: The user asked for async "in order to not block the terminal". The
  pause-and-return behavior for manual steps is the part that matters most: manual steps can
  last days.
- **Alternatives considered**: Blocking I/O with threads (SeaORM is async-only, so this
  would mean wrapping it). Running automated steps as detached background jobs
  (`--detach`) — a real improvement, deferred; it needs a supervisor process and is not
  required by the spec.

## 10. Command-line interface and colored output

- **Decision**: `clap` 4 with derive. Grammar is `trcli <noun> <verb> [args]`
  (`trcli paper add`, `trcli run start`). Global flags: `--workspace`, `--output
  human|json`, `--color auto|always|never`, `--yes`, `--no-input`, `--quiet`, `--verbose`.
  - Color through `anstream` and `anstyle` (what clap itself uses), so help text and our
    output follow the same rules: color only on a terminal, and `NO_COLOR`, `CLICOLOR`,
    and `--color` are respected. A small `Theme` maps meanings (success, warning, error,
    identifier, status) to styles and is configurable.
  - Tables with `comfy-table`, sized to the terminal width.
  - `--output json` prints one JSON document on stdout, never colored, with a stable
    documented shape (FR-009).
  - Results go to stdout; diagnostics, progress, and prompts go to stderr (FR-010).
  - Exit codes are part of the contract (see `contracts/output-and-exit-codes.md`).
  - Confirmations (FR-008) prompt on a terminal; with `--no-input` or no terminal they fail
    with a specific exit code unless `--yes` is given. A script can never hang on a prompt.
- **Rationale**: "The CLI frontend should use colors", plus constitution principle V
  (clear message on stderr, non-zero exit) and FR-009/FR-010.
- **Alternatives considered**: `owo-colors` or `colored` alone (do not strip codes for
  pipes by themselves); a full-screen terminal interface (not asked for, and hostile to
  scripting).

## 11. Input validation

- **Decision**: Three layers, each with one job.
  1. **Syntax, in the CLI**: clap value parsers reject malformed dates, numbers, and enum
     values before any code runs.
  2. **Boundary validation, entering the application**: each use case takes a command
     object built from raw input by a constructor that validates every field and returns
     **all** failures at once in a `ValidationReport` (FR-015). Nothing reaches a
     repository unless the report is empty (FR-014).
  3. **Invariants, in the domain**: value objects (`Title`, `Year`, `Doi`, `CitationKey`,
     `EmailAddress`, `Orcid`, `TagName`, ...) can only be built through a checking
     constructor, so an invalid value cannot exist in memory. Aggregates enforce rules
     that span fields (no loop in a pipeline, a reason for every exclusion).
  - Imported files and fetched catalogue data pass through layers 2 and 3 exactly like
    typed input (FR-012, FR-074).
  - Existence checks (referenced records, file locations) are part of layer 2 and use
    ports.
- **Rationale**: Constitution principle V requires validation at the boundary *and* domain
  invariants. Hand-written value objects keep `trcli-domain` free of dependencies.
- **Alternatives considered**: `garde` or `validator` derive macros — convenient, but they
  validate structs after construction (invalid values exist first) and put a dependency in
  the domain. Validating only in clap — cannot cover files, imports, or lookups.

## 12. Testing: TDD and BDD against the real binary

- **Decision**:

  | Level | What it covers | Tools | Where |
  |-------|----------------|-------|-------|
  | Unit | Value objects, aggregates, domain services, use cases with in-memory fakes | `cargo test`, `rstest`, `proptest` | inside `trcli-domain`, `trcli-application` |
  | Adapter integration | Repositories against a real temporary SQLite database; process runner; parsers | `cargo test`, `tempfile` | `crates/trcli-infra-*/tests` |
  | **Behavior (BDD)** | Every acceptance scenario of the spec, written in Gherkin and run against the compiled `trcli` binary in a temporary directory | `cucumber`, `assert_cmd` | `tests/features/*.feature`, `tests/bdd` |
  | Documentation | Every example in `docs/usage/*.md` is executed and its output compared | `trycmd` | `tests/usage.rs` |

  - **Order (TDD)**: for each slice — write the Gherkin scenarios from the spec and see them
    fail; then, inside-out, a failing unit test before each piece of domain and application
    code; then adapter tests; the scenarios turn green last.
  - BDD steps are generic ("When I run `trcli paper add ...`", "Then the exit code is 2",
    "Then stderr mentions `year`"), so most scenarios need no new step code.
  - In-memory fakes of every port live in `trcli-application` behind a `test-support`
    feature and are reused by all crates' tests.
  - Online lookup is tested against a local stub HTTP server; no test touches the network.
- **Rationale**: The user asked for TDD and for BDD "by testing the CLI itself". Running
  usage-guide examples as tests makes principle VII (docs ship with the feature and stay
  accurate) enforceable instead of aspirational.
- **Alternatives considered**: BDD at the application layer only — faster, but does not
  test argument parsing, exit codes, or output, which is where CLI regressions happen.
  `insta` snapshots for output — `trycmd` covers the same need and doubles as documentation.

## 13. Documentation gates

- **Decision**: `#![deny(missing_docs)]` and `#![deny(rustdoc::broken_intra_doc_links)]` in
  every crate; `cargo doc` runs in CI with warnings as errors. `cargo fmt --check` and
  `cargo clippy -- -D warnings` are required. Each noun of the CLI has
  `docs/usage/<noun>.md`, verified by `trycmd`.
- **Rationale**: Constitution principles VI and VII, turned into checks that fail the build.

## 14. Audit trail and telemetry

- **Decision**:
  - Every use case that changes data records an `AuditEntry` through the same `UnitOfWork`,
    so the change and its audit entry commit or fail together (SC-010).
  - Each entry stores the hash of the previous entry: `hash = SHA-256(prev_hash ‖ canonical
    entry)`. `trcli audit verify` recomputes the chain and reports the first break. The
    tool offers no command to edit or delete entries (FR-052).
  - Telemetry is a separate local table written by a decorator around use-case execution
    (command name, duration, outcome) and by the run engine (per-step timings). It is
    governed by `telemetry.enabled`; nothing is transmitted (FR-055).
- **Rationale**: A hash chain is the simplest mechanism that detects edits and removals made
  outside the tool. It also yields the ordered journal that section 6 relies on.
- **Limit, stated honestly**: someone who rewrites the whole chain consistently is not
  detected; that needs an external anchor and is out of scope.

## 15. Pipelines and runs

- **Decision**:
  - A pipeline is a set of steps with `depends_on`; the domain validates it (unknown
    reference, loop) with a topological sort and names the offending steps (FR-032).
  - Starting a run copies the pipeline definition into the run as an immutable snapshot
    (FR-038), together with parameters, linked methodology and dataset versions, and an
    environment snapshot (FR-042, FR-044).
  - Steps execute one at a time in dependency order. Run and step states are persisted
    before and after every step, so a killed process leaves an accurate record; on the next
    start, a run found `running` with no live process is marked `interrupted`.
  - Step logs go to `.trcli/runs/<run>/<step>.log`; declared output files are fingerprinted.
- **Alternatives considered**: Parallel execution of independent steps — deferred; the spec
  does not require it and it complicates output and resume.

## 16. Fingerprints, environment capture, and secrets

- **Decision**:
  - Fingerprints use BLAKE3, streamed, with a progress bar. A directory is fingerprinted
    over its files in sorted relative-path order with normalized separators, so the same
    data gives the same fingerprint on every platform.
  - An environment snapshot contains OS and version, architecture, CPU and memory summary,
    versions of tools listed in `environment.tools`, and **only** the environment variables
    listed in `environment.variables`. Variables are never captured by default, and names
    matching `*KEY*`, `*TOKEN*`, `*SECRET*`, `*PASSWORD*`, `*CREDENTIAL*` are refused even
    when listed (FR-045).
- **Rationale**: An allow-list is the only approach that cannot leak a secret nobody
  thought to exclude.

## 17. Bibliography formats, citation styles, and online lookup

- **Decision**:
  - Import and export: BibTeX/BibLaTeX (`biblatex` crate), RIS (small parser of our own),
    and CSL-JSON (`serde`). These are the "formats in common use" of FR-020.
  - Citation rendering through `hayagriva`, which implements the Citation Style Language;
    APA and IEEE are built in and any `.csl` file can be configured (ABNT included).
  - Lookup behind a `MetadataCatalog` port in `trcli-infra-catalog` (`reqwest` with rustls):
    Crossref for DOIs, the arXiv service for preprints, Open Library for ISBNs. Only the
    identifier is sent; `lookup.enabled = false` turns it off (FR-075).
  - Duplicates (FR-018): equal normalized DOI, or equal normalized title + first author's
    family name + year.
- **Spike**: confirm `hayagriva`'s API for rendering a single entry with a user-supplied CSL
  file; if it does not fit, fall back to built-in APA/IEEE formatters and treat CSL files as
  a later enhancement.

## 18. Forward compatibility with specs 002 and 003

- **Decision**: Nothing from those specs is built here, and two choices keep them additive:
  - Tags, notes, and links are generic tables keyed by `RecordRef` (kind + id). Spec 003's
    "a record belongs to several projects" becomes one more such table; no table from this
    plan needs a new column.
  - The schema version is stored in the database, and before applying any migration the
    tool copies `trcli.db` to `.trcli/backups/`. Spec 002's full backup/restore/upgrade
    story builds on this.
- **Recommendation**: plan spec 003 (projects) right after the first slice of this plan,
  because the "current project" changes the default scope of every list command.

## 19. Dependency summary

| Purpose | Crate | Version | Used in |
|---------|-------|---------|---------|
| CLI parsing | `clap` (+ `clap_complete`) | 4.6 | cli |
| ORM / migrations | `sea-orm`, `sea-orm-migration` | 2.0 | infra-sqlite |
| Async runtime | `tokio` | 1.53 | application (traits only), adapters, cli |
| Color | `anstream`, `anstyle` | 1.0 | cli |
| Tables / progress | `comfy-table`, `indicatif` | 8.0 / 0.18 | cli |
| Configuration | `figment`, `directories` | 0.10 / 6.0 | infra-system, cli |
| Errors | `thiserror` | 2.0 | all |
| Serialization | `serde`, `serde_json`, `toml` | 1.0 | application, adapters, cli |
| Ids / time | `uuid` (v7), `time` | 1.27 / 0.3 | domain (types only), adapters |
| Hashing | `blake3`, `sha2` | 1.8 / 0.11 | infra-system |
| Bibliography | `biblatex`, `hayagriva` | 0.12 / 0.10 | infra-biblio |
| HTTP | `reqwest` (rustls) | 0.13 | infra-catalog |
| Logging | `tracing`, `tracing-subscriber` | 0.1 | all but domain |
| Tests | `cucumber`, `assert_cmd`, `trycmd`, `rstest`, `proptest`, `tempfile` | 0.23 / 2.2 / 1.2 / 0.27 / 1.11 / 3.27 | tests |

`trcli-domain` depends only on `thiserror`, `uuid`, and `time`.
