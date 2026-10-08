# Phase 1 Data Model: TRCLI Foundation and Architecture

**Date**: 2026-10-08 | **Plan**: [plan.md](./plan.md) | **Spec**: [spec.md](./spec.md)

The model shared by every feature: the workspace, the identity of records, tags, notes,
links, settings, the audit trail, telemetry, and the fixed vocabulary of outcomes and
problems. Table and column names follow these names in `snake_case`.

## Conventions

- "Text(n..m)" is text of n to m characters after trimming surrounding whitespace. Control
  characters other than line break and tab are rejected everywhere (FR-026). Text is kept
  exactly as written apart from that trimming.
- Dates are calendar dates (`YYYY-MM-DD`). Instants are stored in UTC and shown in local
  time (FR-038).
- Rules cite the requirement they implement.
- **Shared** tables are part of what a workspace would exchange with another copy of itself;
  **local** ones are not.

## Value objects (the shared kernel)

| Value object | Rule |
|--------------|------|
| `RecordId` | UUID version 7; generated, never changes |
| `RecordKind` | a registered kind: a name of 2–40 characters of `a-z -`, and a handle prefix of 2–4 letters; both unique among registered kinds |
| `Handle` | `<prefix>-<code>`; code of 4–12 Crockford base-32 characters, lower case; compared ignoring case (FR-010, FR-011) |
| `RecordRef` | `RecordKind` + `RecordId`: the only way one feature refers to another's record (FR-019, FR-072) |
| `Title` | Text(1..500) |
| `Name` | Text(1..200) |
| `LongText` | Text(0..20000) |
| `TagName` | 1–50 characters of `a-z 0-9 - _`. Normalization (stated, FR-025): surrounding whitespace removed, upper case lowered. Anything else — inner spaces, other characters — is rejected with the rule |
| `Relation` | Text(1..50); default `related` |
| `SearchKey` | derived from text: lower case, accents and other combining marks removed, punctuation collapsed to single spaces (FR-067). Used for searching and sorting, never shown |
| `ActorName` | Text(1..200) |
| `FormatVersion` | positive integer |
| `SettingKey` | dotted name of segments `a-z 0-9 _`, for example `output.color`; must be registered |

## Workspace — one per database

| Field | Type | Rule |
|-------|------|------|
| `id` | RecordId | identifies the workspace across copies |
| `name` | Name | required (FR-001) |
| `description` | LongText | optional |
| `format_version` | FormatVersion | increases with each released change to storage (FR-007) |
| `created_at` | instant | |

The name under which the researcher's actions are recorded is the setting
`researcher.name`, kept with the workspace's settings, not in this row.

**States on opening** (FR-007, FR-008):

```text
 format = known ─────────────▶ usable
 format < known ─────────────▶ needs upgrade: reading commands work; writing commands refuse
        └─ workspace upgrade ─▶ copy kept ─▶ migrations ─▶ usable   (on failure: copy restored)
 format > known ─────────────▶ too new: every command refuses to change anything
 cannot open / fails check ──▶ damaged: reported with what and where; nothing written
```

**Rules**:
- A workspace is created only where none exists (FR-002).
- Nothing stored depends on the workspace's location (FR-005).

## Record (the record index) — shared

One row for every record of every kind. A feature's own table uses the same `id` as its
primary key and holds what is particular to that kind.

| Field | Type | Rule |
|-------|------|------|
| `id` | RecordId | primary key |
| `kind` | RecordKind | must be registered |
| `handle` | Handle | unique in the workspace, across all kinds, including deleted rows (FR-010) |
| `display_name` | Text(0..500) | supplied by the kind's descriptor; what messages and the audit trail call the record |
| `search_key` | SearchKey | supplied by the kind's descriptor |
| `created_at`, `updated_at` | instant | FR-018 |
| `deleted_at` | instant | set on deletion; the row stays so the handle is never reused |

**Handle assignment**: prefix of the kind, then 4 characters derived from the random part
of the id; if that handle exists, 5 characters, and so on.

**Resolving a typed reference** (FR-011), among non-deleted records of the kinds the
command accepts, ignoring case:

1. exact handle → that record;
2. otherwise records whose handle starts with what was typed: one → that record; several →
   "ambiguous", listing them; none → "not found", suggesting up to three handles closest by
   edit distance.

A feature may accept further names for its own records (a citation key, a step key); those
are resolved by the feature before falling back to this rule.

## Tag, Note, Link — shared

| Entity | Fields | Rules |
|--------|--------|-------|
| `Tag` | `name: TagName` | unique in the workspace |
| `Tagging` | `tag`, `record` | unique pair; removed with either (FR-015) |
| `Note` | `record`, `body: LongText (1..)`, `created_at` | any number per record; removed with the record |
| `Link` | `from`, `to`, `relation: Relation`, `created_at` | `from ≠ to`; the triple `(from, to, relation)` is unique, and so is its reverse — a link has no direction for the user and is shown from both ends (FR-016); removed with either record |

Each of `record`, `from`, `to` is a foreign key to the record index. Because of that, a
tag, note, or link cannot point at something that does not exist — FR-017's "nothing is
left pointing to the deleted record" is a constraint of the database, not a convention.

Typed relationships that carry rules of their own (a manuscript's authors, a run's
experiment) belong to the features that define them; `Link` is for free linking.

## Deletion (FR-012, FR-017)

Deleting a record is one generic use case, driven by the kind's `DeletionPolicy`:

```text
 ask the kind: is deletion blocked?  ── yes ──▶ refuse; say what stands in the way and the alternative
                │ no
 list dependents: links, taggings, notes, and what the kind reports as referring to it
                │
 confirm (Prompter) ── no / cannot ask ──▶ nothing changes
                │ yes
 in one unit of work: the kind removes its own rows ─▶ links, taggings, notes removed
                      ─▶ index row marked deleted ─▶ audit entry
```

## Setting

A setting exists because a feature registered its **definition**; values come from sources.

**SettingDefinition** (in code, registered at start-up):

| Field | Meaning |
|-------|---------|
| `key` | SettingKey |
| `value_kind` | text, whole number with a range, yes/no, one of a list, list of text, path, span |
| `default` | a value of that kind; every setting has one (FR-042) |
| `scope` | `user`, `workspace`, or `both` (FR-044) |
| `summary` | one line shown by `config get` (FR-041) |
| `secret` | always `false` in the foundation; settings must not hold secrets (FR-045). The field exists so that a definition can never be marked secret and stored — registering one is an error |

**Sources and precedence** (FR-040), later wins:

```text
 default  ‹  user file  ‹  workspace file  ‹  TRCLI_* variables (session)  ‹  command-line option
```

**EffectiveSetting** (computed): key, value, and the source it came from.

**Rules**:
- An unknown key or an invalid value in any source is an error naming the source and the
  key; the tool does not run on a partly valid configuration (FR-043).
- A key found in a scope it does not have is ignored with a warning (FR-044).
- `unset` removes the value from one file; the next source applies (FR-039).

**Settings the foundation registers**: `default_workspace`, `storage.path`,
`storage.busy_timeout_ms`, `output.format`, `output.color`, `output.symbols`,
`output.page_size`, the `theme.*` styles, `researcher.name`, `telemetry.enabled`. Their
allowed values and defaults are in [contracts/configuration.md](./contracts/configuration.md).

## AuditEntry — shared, append-only

| Field | Type | Rule |
|-------|------|------|
| `sequence` | whole number | starts at 1, increases by 1, no gaps; the order of the trail (FR-051) |
| `at` | instant | when it happened; informative, not used for ordering |
| `actor` | ActorName | `researcher.name`, else the system user name, else `unknown` (FR-050) |
| `action` | AuditAction | see below |
| `kind` | RecordKind | of the record concerned; absent for workspace-level actions |
| `record_id` | RecordId | not a foreign key: the entry outlives the record |
| `handle`, `display_name` | text | as they were at the time (FR-046) |
| `changes` | list of `{ field, before, after }` | empty for actions that change no field |
| `previous_hash` | 32 bytes | hash of the entry before; zeros for the first |
| `hash` | 32 bytes | SHA-256 over `previous_hash` and the canonical form of every other field |

**AuditAction**: `create`, `update`, `delete`, `status`, `link`, `unlink`, `tag`, `untag`,
`note`, `import`, `export`, `run`, `confirm`, `setting`, `upgrade`. Features may register
further actions.

**Canonical form**: the fields in the order above, each as UTF-8 text with a length prefix,
so that no two different entries have the same form.

**AuditHead** — local, the file `.trcli/audit.head`: `sequence` and `hash` of the last
entry, rewritten after every commit.

**Rules**:
- An entry is written in the same unit of work as the change it describes (FR-047).
- No layer has an operation that changes or removes an entry (FR-048).
- **Verification** walks the trail from 1: each `previous_hash` must equal the hash before
  it; each `hash` must equal the recomputed one; sequences must have no gap; the last
  entry must match `AuditHead`. The first failure is reported with its sequence and which
  check failed (altered, missing, or shortened).
- Values marked secret are never written to `changes` (FR-054).

## TelemetryRecord — local

| Field | Type |
|-------|------|
| `at` | instant |
| `command` | text, the command path without its arguments (for example `ref add`) |
| `duration_ms` | whole number |
| `outcome` | Outcome |

Written only while `telemetry.enabled` is true (FR-052). Arguments and values are never
recorded. Never transmitted (FR-053). Features add their own telemetry (runs, steps) in
their own tables.

## Outcome and Problem — the fixed vocabulary

**Outcome** (FR-032) — how a command ends; the same for every command, never changed
between versions:

| Outcome | Exit code | Meaning |
|---------|-----------|---------|
| `Success` | 0 | includes an empty list, and work that paused by design |
| `Failure` | 1 | unexpected |
| `InvalidInput` | 2 | invalid input or usage |
| `NotFound` | 3 | nothing matched, or a reference was ambiguous |
| `WorkspaceProblem` | 4 | none found, already exists, too new, needs upgrade, damaged, busy |
| `Refused` | 5 | confirmation required and not given, declined, or blocked |
| `CheckFailed` | 6 | a verification did not pass |
| `OperationFailed` | 7 | something outside the tool could not be done |
| `Interrupted` | 130 | stopped by the researcher |

**Problem** (FR-033, FR-034):

| Field | Meaning |
|-------|---------|
| `code` | a stable, lower-case name such as `validation_failed`, `ambiguous_reference`, `no_workspace` |
| `message` | one sentence for a person |
| `details` | for `validation_failed`, the list of `FieldProblem`; otherwise what is needed to act |
| `changed` | whether anything was changed (almost always "nothing") |
| `next_step` | what to do next, when there is an obvious step |

**FieldProblem** (FR-022): `field` as the user gave it (for example `--year`), `value`,
`problem`, `expected`, and `example` or `choices`.

**ValidationReport**: an ordered list of `FieldProblem` (errors) and a separate list of
warnings. A command constructor returns either a valid command or a report with at least
one error; all errors found are in it (FR-023).

The foundation's problem codes are listed in
[contracts/output-and-exit-codes.md](./contracts/output-and-exit-codes.md); features
register more.

## Tables

| Table | Holds | Scope |
|-------|-------|-------|
| `workspace` | the one workspace row | shared |
| `record` | the record index | shared |
| `tag`, `tagging` | tags and which records carry them | shared |
| `note` | notes | shared |
| `link` | links | shared |
| `audit_entry` | the trail | shared |
| `telemetry_record` | local telemetry | local |
| *(file)* `audit.head` | end of the trail | local |
| *(file)* `config.toml` | workspace settings | shared |

Indexes: `record(kind, deleted_at)`, `record(handle)` unique, `record(search_key)`,
`tagging(record)`, `tagging(tag)`, `note(record)`, `link(from)`, `link(to)`,
`audit_entry(record_id)`, `audit_entry(at)`.

## What features add

- One table per kind (or more), with `id` referring to `record`.
- A `RecordKindDescriptor` per kind, registered in the composition root.
- Their own settings, audit actions, problem codes, and telemetry, by registration.
- Nothing in the tables above changes when a feature is added.
- The sample kinds used to test the foundation (`specimen`, `sample-note`) have their own
  tables, created by the example they live in, in the workspaces it is used in; they are
  not part of the workspace format.

The exact shape of those registrations is in
[contracts/feature-contract.md](./contracts/feature-contract.md).
