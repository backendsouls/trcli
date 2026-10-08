# Phase 1 Data Model: TRCLI Research Workspace

**Date**: 2026-10-08 | **Plan**: [plan.md](./plan.md) | **Spec**: [spec.md](./spec.md)

This is the domain model, grouped by bounded context. It names aggregates, their fields,
validation rules, relationships, and state transitions. Table and column names used by the
SQLite adapter follow these names in `snake_case`; mapping details belong to implementation.

## Conventions

- **Aggregate root** entities are marked **(AR)**. One command changes one aggregate in one
  transaction, together with its audit entry.
- Every aggregate root has these fields, not repeated below:

  | Field | Type | Rule |
  |-------|------|------|
  | `id` | UUID v7 | primary key, generated, immutable |
  | `handle` | Handle | `<prefix>-<code>`, unique per workspace and type, immutable |
  | `created_at`, `updated_at` | timestamp (UTC) | set by the system |

- References to other contexts are by `RecordRef` or by id, never by embedding.
- "Text(n..m)" means trimmed text whose length in characters is between n and m. Control
  characters other than newline and tab are rejected everywhere.
- Dates are calendar dates (`YYYY-MM-DD`); timestamps are UTC instants.
- Each rule cites the requirement it implements where one exists.

## Shared kernel (`shared`)

### Value objects

| Value object | Rule |
|--------------|------|
| `Handle` | prefix of 2–4 lowercase letters, `-`, code of 4–12 Crockford base32 characters |
| `RecordKind` | one of: paper, citation, question, hypothesis, review, draft, experiment, run, result, exhibit, methodology, dataset, environment, staff, course |
| `RecordRef` | `RecordKind` + id |
| `Title` | Text(1..500) |
| `Name` | Text(1..200) |
| `LongText` | Text(0..20000) |
| `TagName` | 1–50 characters of `a-z 0-9 - _`, lowercased on input |
| `Year` | integer 1000 to current year + 1 |
| `Url` | absolute `http` or `https` URL, at most 2000 characters |
| `Location` | a local path (absolute, or relative to the workspace root) or a `Url` |
| `Fingerprint` | algorithm name + lowercase hex digest (`blake3:…`) |
| `EmailAddress` | one `@`, non-empty local part, domain with a dot, at most 254 characters |
| `Orcid` | `0000-0000-0000-000X` with a valid check digit |
| `Doi` | starts with `10.`, has a `/`, stored lowercase without resolver prefix |
| `ArxivId` | new (`2401.01234`, optional `vN`) or old (`math/0301001`) form |
| `Isbn` | ISBN-10 or ISBN-13 with a valid check digit, stored without hyphens |
| `PersonName` | `family` Text(1..100) + optional `given` Text(0..100) |

### Cross-cutting entities

| Entity | Fields | Rules |
|--------|--------|-------|
| `Tag` | `name: TagName` | unique per workspace |
| `Tagging` | `tag`, `target: RecordRef` | unique pair (FR-006) |
| `Note` | `target: RecordRef`, `body: LongText (1..)`, `created_at` | any number per record (FR-006) |
| `Link` | `from: RecordRef`, `to: RecordRef`, `relation: Text(1..50)`, optional `stance` | unique triple; `from ≠ to`; both ends must exist; shown from both ends (FR-007). Removed with either end, after confirmation (FR-008). |

Typed relationships that carry rules (a draft's citations, an experiment's hypotheses) are
modelled inside their aggregates below; `Link` is for the free linking FR-007 asks for.

## `workspace`

### Workspace (AR) — one per database

| Field | Type | Rule |
|-------|------|------|
| `name` | Name | required (FR-001) |
| `description` | LongText | optional |
| `researcher_name` | Name | optional; used as actor name |
| `schema_version` | integer | written by migrations |

Settings such as `telemetry.enabled` live in `.trcli/config.toml`, not here
(see [contracts/configuration.md](./contracts/configuration.md)).

## `library`

> Specified by `specs/005-literature` since 2026-10-08, together with `review` below. These
> sections cover what was in this spec when the plan was written; other kinds of reference,
> bibliographies, annotations, relations, two-stage screening, and synthesis are added when
> that spec is planned.

### Paper (AR) — prefix `pap`

| Field | Type | Rule |
|-------|------|------|
| `title` | Title | required (FR-016) |
| `authors` | ordered list of PersonName | 0..200 |
| `year` | Year | optional |
| `venue` | Text(0..300) | optional |
| `abstract` | LongText | optional |
| `doi` / `arxiv_id` / `isbn` | Doi / ArxivId / Isbn | each optional; `doi` unique when present |
| `url` | Url | optional |
| `local_copy` | Location | optional; must exist when given |
| `status` | ReadingStatus | default `to_read` (FR-017) |
| `status_changed_at` | timestamp | set on every status change |
| `search_text` | derived | lowercase title + authors + venue |

**ReadingStatus**: `to_read`, `reading`, `read`, `discarded`. Any transition is allowed;
each is recorded with its time.

**Duplicate rule (FR-018)**: a paper is a likely duplicate of another when the normalized
DOIs are equal, or when normalized title, first author's family name, and year are all
equal. Normalization: lowercase, strip punctuation and diacritics, collapse whitespace.
The domain service `DuplicateDetector` reports candidates; the user decides add, merge, or
cancel. **Merge** keeps the existing paper, fills only its empty fields, and moves the
other paper's citations, notes, tags, and links to it.

### Citation (AR) — prefix `cit`

| Field | Type | Rule |
|-------|------|------|
| `paper_id` | id | required; the paper must exist |
| `key` | CitationKey | 1–64 characters of `A-Z a-z 0-9 _ : -`, unique per workspace (FR-019) |
| `entry_type` | enum | article, book, inproceedings, incollection, thesis, report, preprint, misc |
| `extra_fields` | map text → text | pages, volume, issue, publisher, … |

**Key generation**: `<family><year><first title word>` lowercased and reduced to allowed
characters (`silva2024deep`); on collision a letter suffix is added (`…a`, `…b`) and the
user is told.

## `inquiry`

> Specified by `specs/013-ideas-questions` since 2026-10-08, with ideas, topics, and review.

### ResearchQuestion (AR) — prefix `rq`; owns its hypotheses

| Field | Type | Rule |
|-------|------|------|
| `statement` | Title | required (FR-058) |
| `motivation` | LongText | optional |
| `parent_id` | id | optional; must not create a loop (FR-064) |
| `status` | `open` / `answered` / `abandoned` | default `open` |
| `closing_note` | LongText (1..) | required when status is `answered` or `abandoned` (FR-062) |
| `status_history` | list of (status, at, note) | appended on every change |

**Transitions**: `open → answered`, `open → abandoned`, and back to `open` (reopen clears
nothing; history is kept).

### Hypothesis (entity of ResearchQuestion) — prefix `hyp`

| Field | Type | Rule |
|-------|------|------|
| `statement` | Title | required (FR-059) |
| `support_criteria`, `refutation_criteria` | LongText | optional |
| `status` | `untested` / `supported` / `refuted` / `inconclusive` | default `untested` |
| `status_history` | list of (status, at) | appended on every change |

**Rule (FR-062)**: leaving `untested` requires at least one `Evidence` (see `findings`)
linked to the hypothesis.

Links from papers, reviews, experiments, results, drafts, methodologies, and datasets to
questions and hypotheses (FR-060) use `Link` with relation `addresses`. FR-063 (records
linked to no question) is a query over `Link`.

## `review`

### BibliographicResearch (AR) — prefix `rev`; owns searches and candidates

| Field | Type | Rule |
|-------|------|------|
| `title` | Title | required (FR-022) |
| `question` | LongText (1..) | required |
| `scope` | LongText | optional |
| `inclusion_criteria`, `exclusion_criteria` | lists of Text(1..500) | optional |

**Search** (entity): `query: Text(1..2000)`, `source: Name`, `searched_on: date` (not in the
future), `result_count: integer ≥ 0` (FR-023).

**Candidate** (entity): `paper_id` (unique within the review), `decision`: `unscreened` /
`included` / `excluded`, `reason: Text(1..500)`, `decided_at`.

**Rules (FR-024)**: `excluded` requires a reason; a decision can be changed, and each change
is audited. **Summary (FR-025)** is a query: counts found (sum of `result_count`),
candidates, screened, included, and excluded grouped by reason.

## `writing`

> Specified by `specs/008-manuscripts` since 2026-10-08, where a draft is called a
> manuscript. This section covers what was in this spec when the plan was written.

### Draft (AR) — prefix `drf`; owns versions, authors, and typed references

| Field | Type | Rule |
|-------|------|------|
| `title` | Title | required (FR-026) |
| `abstract` | LongText | optional |
| `authors` | ordered list of author entries | each a staff id or a free PersonName |
| `target_venue` | Text(0..300) | optional |
| `deadline` | date | optional |
| `manuscript` | Location | optional; must exist when a local path |
| `stage` | DraftStage | default `idea` (FR-027) |
| `stage_history` | list of (stage, at) | appended on every change |
| `citations` | set of citation ids | FR-029 |
| `reported` | set of (result or exhibit id, `pinned_at`) | FR-068 |

**DraftStage**: `idea`, `outlining`, `drafting`, `in_review`, `submitted`, `accepted`,
`published`, `abandoned`. Any transition is allowed (research is not linear); every one is
recorded.

**DraftVersion** (entity): `number` (1, 2, 3 … assigned in sequence, never reused),
`summary: Text(1..2000)`, `created_at`, optional `manuscript_fingerprint` (FR-028).

Links to experiments, datasets, and methodologies use `Link`.

## `experimentation`

> Specified by `specs/004-experiments` since 2026-10-08. This section and `findings` below
> cover what was in this spec when the plan was written; design, parameters, metrics,
> sweeps, software, conclusions, and replication are added when that spec is planned.

### Experiment (AR) — prefix `exp`; owns its pipeline

| Field | Type | Rule |
|-------|------|------|
| `name` | Name | required (FR-030) |
| `description` | LongText | optional |
| `status` | `planned` / `active` / `completed` / `abandoned` | default `planned`; becomes `active` on first run |
| `hypothesis_ids` | set of ids | each must exist |
| `methodology_id` | id | optional |
| `dataset_version_ids` | set of ids | each must exist |
| `responsible_staff_ids` | set of ids | FR-049 |
| `pipeline` | Pipeline | may be empty until a run is started |

**Step** (entity of the pipeline) — FR-031:

| Field | Type | Rule |
|-------|------|------|
| `key` | 1–50 characters of `a-z 0-9 - _` | unique within the pipeline |
| `name` | Name | required |
| `kind` | `automated` / `manual` | required |
| `instructions` | LongText | required for `manual` |
| `command` | program + arguments + `shell` flag + working directory | required for `automated`; program non-empty |
| `depends_on` | set of step keys | each must exist; no loop (FR-032) |
| `outputs` | list of Location | files the step is expected to produce |
| `performer_staff_id` | id | optional, manual steps only (FR-049) |
| `position` | integer | order of definition, used to break ties |

**Pipeline invariants**: step keys unique; every `depends_on` resolves; the dependency
graph has no cycle — the error names the steps in the cycle. Execution order is a
topological order, ties broken by `position`.

### Run (AR) — prefix `run`

| Field | Type | Rule |
|-------|------|------|
| `experiment_id` | id | required |
| `number` | integer | sequence per experiment |
| `status` | RunStatus | see below |
| `pipeline_snapshot` | the full pipeline as it was at start | immutable (FR-038) |
| `parameters` | map text → text | optional |
| `methodology_id`, `dataset_version_ids` | ids | copied from the experiment at start (FR-042) |
| `environment_id` | id | captured at start (FR-044) |
| `started_at`, `ended_at` | timestamps | |
| `started_by` | actor | |

**StepResult** (entity of Run): `step_key`, `status`, `started_at`, `ended_at`,
`exit_code`, `log: Location`, `output_fingerprints`, `notes: LongText`, `performed_by`.

**RunStatus transitions** (FR-033 to FR-036):

```text
            start                 all steps succeeded
 (new) ───────────▶ running ───────────────────────────▶ succeeded
                     │  ▲  │
       manual step   │  │  │ automated step failed / manual step marked failed
       reached       ▼  │  ▼
                   paused │ failed ──── resume (from failed step) ──▶ running
                     │    │   │
   confirm done ─────┘    │   └──── cancel ──▶ cancelled
   (or resume)            │
                          └── process killed / Ctrl-C ──▶ interrupted ── resume ──▶ running
 paused ── cancel ──▶ cancelled        interrupted ── cancel ──▶ cancelled
```

**StepResult status**: `pending`, `running`, `waiting` (manual step reached), `succeeded`,
`failed`, `skipped` (after a failure or cancellation).

**Rules**:
- A manual step is completed only by an explicit confirmation carrying the actor, time, and
  optional notes, or failed with a required reason (FR-034).
- Resume never repeats a step whose status is `succeeded` (SC-006).
- A run found `running` with no live process is set to `interrupted` on next access.
- Several runs of one experiment may exist at once and are independent.
- A run cannot be deleted while a `Result` or `Exhibit` refers to it (FR-071).

## `findings`

### Result (AR) — prefix `res`

| Field | Type | Rule |
|-------|------|------|
| `run_id` | id | required, immutable (FR-067) |
| `name` | 1–100 characters of `A-Z a-z 0-9 _ . -` | required; identifies "the same result" across runs |
| `value` | number or text | number must be finite |
| `unit` | Text(0..50) | optional |
| `uncertainty` | number ≥ 0 | optional; only with a numeric value |
| `description` | LongText | optional |

### Exhibit (AR) — prefix `fig` (figure) or `tbl` (table)

| Field | Type | Rule |
|-------|------|------|
| `run_id` | id | required, immutable |
| `kind` | `figure` / `table` | required |
| `title` | Title | required (FR-066) |
| `caption` | LongText | optional |
| `file` | Location | must exist when recorded |
| `fingerprint` | Fingerprint | computed when recorded; re-checked by `verify` (FR-070) |

### Evidence (relationship) — FR-068

`result_id`, `hypothesis_id`, `stance`: `supports` / `contradicts` / `neutral`. Unique pair.

**Rules**:
- Recording from a run whose status is not `succeeded` requires confirmation.
- **Provenance (FR-067)** is a query from the run: pipeline snapshot, dataset versions,
  methodology, environment.
- **Stale flag (FR-069)**: a draft reporting result *R* is flagged when another result with
  the same `name` exists for the same experiment from a run started later than *R*'s run.
  The user keeps *R* (the flag is dismissed for that newer result) or switches.

## `assets`

> Methodology is specified by `specs/014-conventions-methods` since 2026-10-08; Dataset
> remains specified here.

### Methodology (AR) — prefix `mth`

`name: Name` (required, unique), `description: LongText`, `procedure: LongText`,
`reference_paper_ids: set of ids` (FR-039).

### Dataset (AR) — prefix `dat`; owns its versions

| Field | Type | Rule |
|-------|------|------|
| `name` | Name | required, unique (FR-040) |
| `description` | LongText | optional |
| `origin` | Text(0..500) | optional |
| `license` | Text(0..100) | optional |
| `location` | Location | required; a local path must exist at registration |

**DatasetVersion** (entity) — prefix `dv`: `number` (sequence), `fingerprint`,
`size_bytes`, `file_count`, `recorded_at`, `note`.

**Rules (FR-041)**: registration creates version 1. `verify` recomputes the fingerprint and
answers `unchanged`, `changed` (offers to record a new version), or `unreachable` (the
location cannot be read — never reported as changed). Versions are never edited or deleted
while a run refers to them. A URL location is recorded but not fingerprinted; its version
has no fingerprint and `verify` answers `unreachable`.

## `reproducibility`

### EnvironmentSnapshot (AR) — prefix `env`; immutable once captured

| Field | Type | Rule |
|-------|------|------|
| `name` | Name | required for manual captures; generated for run captures (FR-043) |
| `os_name`, `os_version`, `architecture` | text | captured |
| `hardware` | CPU model, core count, memory | captured |
| `tools` | map tool → version text | from `environment.tools` |
| `variables` | map name → value | only names in `environment.variables`; secret-like names refused (FR-045) |
| `digest` | Fingerprint | over the canonical content; equal digests mean equal environments |

**Diff (FR-043)**: item-by-item comparison of two snapshots: added, removed, changed.

### ReproCheck (computed, not stored) — FR-046

Compares a past run with the present: environment (run's snapshot vs a fresh capture),
dataset versions (recorded fingerprint vs current), methodology (changed since the run),
pipeline (snapshot vs current definition). Verdict `reproducible` only when there is no
difference; otherwise the list of differences, each with its category.

### ReproPackage — FR-047

A single archive containing a manifest (format version, run, experiment, parameters),
the pipeline snapshot, methodology text, dataset references with fingerprints (not the
data), the environment snapshot, results and exhibits metadata, and step logs.

## `people`

> Specified by `specs/012-people` since 2026-10-08.

### StaffMember (AR) — prefix `stf`

| Field | Type | Rule |
|-------|------|------|
| `name` | PersonName | required (FR-048) |
| `role` | Text(1..100) | required |
| `affiliation` | Text(0..300) | optional |
| `email` | EmailAddress | optional |
| `orcid` | Orcid | optional, unique when present |
| `active` | boolean | `false` after removal |

**Rule (FR-050)**: removing a staff member with assignments or history sets `active =
false` and removes current assignments after confirmation; names already written into
runs, versions, and audit entries are stored there as text and remain.

**Assignments (FR-049)** are held by the owning aggregates: `Draft.authors`,
`Experiment.responsible_staff_ids`, `Step.performer_staff_id`. "Assignments of a person" is
a query across them.

## `learning`

> Specified by `specs/011-courses-roadmaps` since 2026-10-08.

### Course (AR) — prefix `crs`

| Field | Type | Rule |
|-------|------|------|
| `title` | Title | required (FR-056) |
| `institution` | Text(0..300) | optional |
| `mode` | `taking` / `teaching` | required |
| `starts_on`, `ends_on` | dates | optional; `ends_on ≥ starts_on` |
| `status` | `planned` / `in_progress` / `completed` / `dropped` | default `planned` |
| `notes` | LongText | optional |

Links to papers, methodologies, and drafts use `Link` (FR-057).

## `governance`

### AuditEntry — append-only (FR-051, FR-052)

| Field | Type | Rule |
|-------|------|------|
| `sequence` | integer | strictly increasing, no gaps |
| `at` | timestamp | |
| `actor` | text | OS user and researcher name, stored as text |
| `action` | `create` / `update` / `delete` / `import` / `export` / `run` / `status` / `confirm` | |
| `target` | RecordRef + handle + display name | stored as text so it survives deletion |
| `changes` | list of (field, before, after) | secret-free by construction |
| `prev_hash`, `hash` | SHA-256 | `hash = SHA-256(prev_hash ‖ canonical(entry))` |

No update or delete operation exists for this entity in any layer. `verify` walks the chain
and reports the first sequence number at which it breaks.

### TelemetryRecord (FR-054, FR-055)

`at`, `kind`: `command` / `run` / `step`, `name`, `duration_ms`, `outcome`, optional
`experiment_id`, `run_id`. Written only when `telemetry.enabled` is true. Never transmitted.

## Deletion rules (FR-008)

| Deleting | Blocked when | Otherwise, after confirmation |
|----------|--------------|-------------------------------|
| Paper | — | its citations, candidate entries, notes, tags, links are listed, then removed |
| Citation | — | removed from drafts that use it (listed first) |
| ResearchQuestion | it has sub-questions | hypotheses and their evidence links are removed; linked records are kept |
| Experiment | it has runs | pipeline removed |
| Run | a result or exhibit refers to it (FR-071) | step results and logs removed |
| Dataset / DatasetVersion | a run refers to a version | removed |
| Methodology | a run refers to it | removed from experiments (listed first) |
| EnvironmentSnapshot | a run refers to it | removed |
| StaffMember | — | deactivated, see `people` |
| Any record | — | `Link`, `Tagging`, and `Note` rows that point to it are removed in the same transaction, so nothing dangles |

## Forward compatibility

- Spec 003 (projects) adds a `ProjectMembership(project_id, target: RecordRef)` table and a
  "current project" setting. No entity above changes.
- Spec 002 adds entities that attach by `RecordRef` (annotations, tasks, notebook entries).
