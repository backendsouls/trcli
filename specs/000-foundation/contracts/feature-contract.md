# Contract: What a Feature Receives and Must Provide

**Plan**: [../plan.md](../plan.md) | **Data model**: [../data-model.md](../data-model.md) | **Spec**: [../spec.md](../spec.md) — User Story 8; FR-068 to FR-076

The other contracts of this specification are between TRCLI and its users. This one is
between the **foundation** and every **feature** built on it. It is what makes "add a
feature without breaking the whole" checkable. It describes roles and obligations; exact
signatures are settled in implementation and documented in the code, and a change to the
obligations below is a breaking change for every feature.

## What a feature provides

### 1. A descriptor for each kind of record it owns

| Item | Meaning | Rule |
|------|---------|------|
| Kind name | What the kind is called in output and in `--kind` filters (`reference`, `task`) | 2–40 characters of `a-z -`; unique among all kinds |
| Handle prefix | The start of its records' short names (`ref`, `tsk`) | 2–4 letters; unique among all kinds; never changed after release |
| Display name | For a record, the short text messages and the audit trail call it | Up to 500 characters; never empty for a stored record |
| Search key source | The text a record is found by with `--search` | The foundation normalizes it |
| Deletion policy | For a record: what blocks deleting it, with the alternative to offer; and what else refers to it and would be affected | Must answer without changing anything |
| Removal | How to remove the kind's own rows for a record | Called inside the foundation's unit of work |

Registering the descriptor is the **only** step needed for the kind to have: handle
assignment; reference resolution by unique prefix; `tag`, `note`, `link`; guarded `rm`;
`--tag`, `--search`, `--sort`, `--limit` on its list; the common fields in both output
forms; its count in `workspace show`; and audit entries for the shared actions.

### 2. Its own use cases, following four obligations

| Obligation | Why | How it is checked |
|------------|-----|-------------------|
| Take a **validated command**, built by a constructor that returns every problem at once | FR-020 to FR-023 | Scenarios for invalid input exist for every input the command accepts (gate) |
| Change data only inside a **unit of work**, and record an **audit entry** in it for every change | FR-046, FR-047 | The contract tests of `UnitOfWork`; a scenario per command checks `audit list` |
| Return a **view model** that can be rendered for people and serialized | FR-028 | A scenario per command compares the two forms |
| Return an **Outcome** from the fixed set; describe failures as a **Problem** with a registered code | FR-032, FR-033 | Exit codes asserted in scenarios |

### 3. Its commands, following the grammar

- `trcli <noun> <verb>`; the shared verbs keep their meaning and options
  ([cli-conventions.md](./cli-conventions.md)).
- Every command has help with at least one example; every command group has
  `docs/usage/<noun>.md` whose examples run as tests.
- Anything that asks a question goes through the `Prompter`; anything long reports through
  `Progress`. A feature never reads standard input or draws on the terminal by itself.

### 4. By registration, whatever else it adds

| Addition | Registered as |
|----------|---------------|
| Settings | `SettingDefinition`s: key, kind of value, allowed values, default, scope, one-line meaning |
| Kinds of problem | A code and its outcome |
| Audit actions beyond the shared ones | A name |
| Storage | Migrations, applied in order with the foundation's |
| Things that are due, telemetry, report sections | The registries of the specifications that own those |

### 5. Its place in the layers

| The feature's… | Goes in | May use |
|----------------|---------|---------|
| Rules, aggregates, value objects | `trcli-domain/src/<context>/` | The shared kernel only |
| Use cases and its ports | `trcli-application/src/<context>/` | Its domain module; foundation ports |
| Tables, migrations, stores | `trcli-infra-sqlite/src/<context>/` | Its application module |
| Arguments, commands, rendering | `trcli-cli/src/{args,commands,render}/` | Everything, through `compose.rs` |

A feature refers to another feature's records by `RecordRef` only, and reaches another
feature's behaviour only through a port that feature publishes (FR-072).

## What the foundation guarantees in return

| Guarantee | Requirement |
|-----------|-------------|
| A record's id and handle never change and are never reused | FR-010 |
| A tag, note, or link never points at a record that no longer exists | FR-017 |
| A unit of work commits everything in it, including the audit entry, or nothing | FR-009, FR-047 |
| A handler is not called with a workspace that is too new, needs upgrading, or is damaged | FR-007, FR-008 |
| Settings handed to a handler are already validated | FR-043 |
| "Now", new identifiers, and the actor come from ports and are fixed in tests | FR-071 |
| The `Prompter` never blocks when nobody can answer | FR-035 |
| Rendering honours colour, symbols, width, and the output form without the feature's involvement | FR-029, FR-030 |
| Shared tables do not change shape when a feature is added | data model |

## What is checked before a feature is accepted (FR-073, FR-074)

| Gate | Check |
|------|-------|
| Tests came first | The pull request shows the failing run before the change |
| Behaviour | Every acceptance scenario of the feature's spec is a Gherkin scenario that passes against the binary |
| Invalid input | Every argument and option of every new command appears in at least one rejection scenario |
| Documentation | The build passes with documentation required for every item, public and private |
| Usage guide | `docs/usage/<noun>.md` exists and its examples pass |
| Help | Every new command has help text with an example |
| Layers | `tests/layering.rs` passes |
| Contracts | Every new port has a contract suite passed by its fake and its adapter |
| Three systems | All of the above on Linux, macOS, and Windows |
| Upgrade | If storage changed: a migration, and the upgrade test passes for every earlier format |

## Proof that this contract is sufficient

Two sample kinds, `specimen` and `sample-note`, are defined under `src/sample/` in the
application crate and compiled only with the `sample-kind` build feature, which no release
enables. The scenarios of user stories 2 and 8 run against them. If making them work
requires editing foundation code rather than registering, this contract has a gap.
