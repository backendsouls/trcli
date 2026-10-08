# Contracts: Venues — Places to Publish

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli venue` | [cli-venue.md](./cli-venue.md) | User Stories 1 and 4; FR-001 to FR-011, FR-032 to FR-036 |
| `trcli event`, `trcli call`, `trcli deadlines` | [cli-call.md](./cli-call.md) | User Story 2; FR-012 to FR-022 |
| `trcli manuscript venue`, `trcli venue counts` | [cli-candidate.md](./cli-candidate.md) | User Story 3; FR-023 to FR-031 |
| `trcli event attend`, `trcli talk`, `trcli talks` | [cli-talk.md](./cli-talk.md) | User Story 5; FR-037 to FR-041 |

Submissions and peer review are not here: they are `trcli submission` in
`specs/002-research-lifecycle`, which names a venue from this register. This specification
reads those submissions to show `trcli venue history` and `trcli venue waiting`.

Every call date, and every practical date of attending an event, appears in `trcli due`
(`specs/003-research-projects`) as well as in `trcli deadlines`.
